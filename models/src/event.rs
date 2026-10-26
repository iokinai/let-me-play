use serde::{Deserialize, Serialize};
use trl::Trl;

use crate::input_event::InputEvent;

#[derive(Trl, Debug, Serialize, Deserialize)]
#[constructor]
#[getters]
pub struct SequenceEvent {
    event: SingleEvent,
    delay_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SingleEvent {
    Input(InputEvent),
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    Single(SingleEvent),
    Sequence(Vec<SequenceEvent>),
}
