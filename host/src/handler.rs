use std::time::Duration;

use enigo::{Direction, Enigo, Mouse};
use let_me_play_models::{
    Event, InputEvent, MouseButton, MouseInput, MouseMove, SequenceEvent, SingleEvent,
};
use tokio::sync::Mutex;

use crate::{
    error::{HandlerError, LmpError},
    mappings::send_input,
};

pub struct Handler {
    enigo: Mutex<Enigo>,
    min_seq_delay_ms: u64,
}

impl Handler {
    pub fn new(enigo: Enigo, min_seq_delay_ms: u64) -> Handler {
        Handler {
            enigo: Mutex::new(enigo),
            min_seq_delay_ms,
        }
    }

    async fn handle_mouse_click(&self, button: &MouseButton) -> Result<(), LmpError> {
        let mut enigo = self.enigo.lock().await;

        let btn = match button {
            MouseButton::Left => enigo::Button::Left,
            MouseButton::Right => enigo::Button::Right,
        };

        enigo
            .button(btn, Direction::Click)
            .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;

        Ok(())
    }

    async fn handle_mouse_move(&self, mv: &MouseMove) -> Result<(), LmpError> {
        let mut enigo = self.enigo.lock().await;
        enigo
            .move_mouse(*mv.x(), *mv.y(), enigo::Coordinate::Rel)
            .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;

        Ok(())
    }

    async fn handle_keyboard_input(&self, input: char) -> Result<(), LmpError> {
        let mut enigo = self.enigo.lock().await;

        send_input(&mut enigo, input)?;

        Ok(())
    }

    async fn handle_mouse_input(&self, input: &MouseInput) -> Result<(), LmpError> {
        match input {
            MouseInput::Click(mouse_button) => self.handle_mouse_click(mouse_button).await,
            MouseInput::Move(mouse_move) => self.handle_mouse_move(mouse_move).await,
        }
    }

    async fn handle_input_event(&self, input: &InputEvent) -> Result<(), LmpError> {
        match input {
            InputEvent::Keyboard(keyboard_input) => {
                self.handle_keyboard_input(*keyboard_input).await
            }
            InputEvent::Mouse(mouse_input) => self.handle_mouse_input(mouse_input).await,
        }
    }

    async fn handle_event_sequence(&self, events: &[SequenceEvent]) -> Result<(), LmpError> {
        for event in events.iter() {
            self.handle_single_event(event.event()).await?;

            let delay = (*event.delay_ms()).max(self.min_seq_delay_ms);

            tokio::time::sleep(Duration::from_millis(delay)).await;
        }

        Ok(())
    }

    async fn handle_single_event(&self, event: &SingleEvent) -> Result<(), LmpError> {
        match event {
            SingleEvent::Input(input_event) => self.handle_input_event(input_event).await,
        }
    }

    pub async fn handle_event(&self, event: &Event) -> Result<(), LmpError> {
        match event {
            Event::Single(event) => self.handle_single_event(event).await,
            Event::Sequence(events) => self.handle_event_sequence(events).await,
        }
    }
}
