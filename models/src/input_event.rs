use serde::{Deserialize, Serialize};

use crate::{MouseMove, PressEvent};

#[derive(Debug, Serialize, Deserialize)]
pub enum InputEvent {
    PressEvent(PressEvent),
    MouseMove(MouseMove),
}
