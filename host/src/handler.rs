use std::time::Duration;

use enigo::{Direction, Enigo, Keyboard, Mouse};
use let_me_play_models::{
    Event, InputEvent, MouseButton, MouseMove, PressEvent, PressEventType, SequenceEvent,
    SingleEvent,
};
use tokio::sync::Mutex;

use crate::{
    error::{HandlerError, LmpError},
    mappings::char_to_keycode,
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

    async fn handle_mouse_click(
        &self,
        button: &MouseButton,
        ty: &PressEventType,
    ) -> Result<(), LmpError> {
        let btn = match button {
            MouseButton::Left => enigo::Button::Left,
            MouseButton::Right => enigo::Button::Right,
        };

        match ty {
            PressEventType::Click => {
                let mut enigo = self.enigo.lock().await;
                enigo
                    .button(btn, Direction::Click)
                    .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;
            }
            PressEventType::Hold(ms) => {
                {
                    let mut enigo = self.enigo.lock().await;
                    enigo
                        .button(btn, Direction::Press)
                        .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;
                }

                tokio::time::sleep(Duration::from_millis(*ms)).await;

                {
                    let mut enigo = self.enigo.lock().await;
                    enigo
                        .button(btn, Direction::Release)
                        .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;
                }
            }
        }

        Ok(())
    }

    async fn handle_mouse_move(&self, mv: &MouseMove) -> Result<(), LmpError> {
        let mut enigo = self.enigo.lock().await;
        enigo
            .move_mouse(*mv.x(), *mv.y(), enigo::Coordinate::Rel)
            .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;

        Ok(())
    }

    async fn handle_keyboard_input(
        &self,
        input: char,
        ty: &PressEventType,
    ) -> Result<(), LmpError> {
        let code = char_to_keycode(input)
            .ok_or(LmpError::HandlerError(HandlerError::KeyIsNotSupported))?;

        match ty {
            PressEventType::Click => {
                let mut enigo = self.enigo.lock().await;
                enigo
                    .raw(code, Direction::Click)
                    .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;
            }
            PressEventType::Hold(ms) => {
                {
                    let mut enigo = self.enigo.lock().await;
                    enigo
                        .raw(code, Direction::Press)
                        .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;
                }

                tokio::time::sleep(Duration::from_millis(*ms)).await;

                {
                    let mut enigo = self.enigo.lock().await;
                    enigo
                        .raw(code, Direction::Release)
                        .map_err(|_| LmpError::HandlerError(HandlerError::CouldNotSendInput))?;
                }
            }
        }

        Ok(())
    }

    async fn handle_input_event(&self, input: &InputEvent) -> Result<(), LmpError> {
        match input {
            InputEvent::PressEvent(press_event) => match press_event {
                PressEvent::Keyboard { button, ty } => {
                    self.handle_keyboard_input(*button, ty).await
                }

                PressEvent::Mouse { button, ty } => self.handle_mouse_click(button, ty).await,
            },
            InputEvent::MouseMove(mouse_move) => self.handle_mouse_move(mouse_move).await,
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
