use serde::{Deserialize, Serialize};

use crate::mouse_input::MouseInput;

#[derive(Debug, Serialize, Deserialize)]
pub enum InputEvent {
    Keyboard(char),
    Mouse(MouseInput),
}
