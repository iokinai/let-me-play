use serde::{Deserialize, Serialize};

use crate::MouseButton;

#[derive(Serialize, Deserialize, Debug)]
pub enum PressEvent {
    Keyboard {
        button: char,
        ty: PressEventType,
    },
    Mouse {
        button: MouseButton,
        ty: PressEventType,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PressEventType {
    Click,
    // duration, ms
    Hold(u64),
}
