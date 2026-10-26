use serde::{Deserialize, Serialize};

use crate::{mouse_button::MouseButton, mouse_move::MouseMove};

#[derive(Debug, Serialize, Deserialize)]
pub enum MouseInput {
    Click(MouseButton),
    Move(MouseMove),
}
