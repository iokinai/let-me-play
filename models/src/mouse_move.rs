use serde::{Deserialize, Serialize};
use trl::Trl;

#[derive(Trl, Debug, Serialize, Deserialize)]
#[constructor]
#[getters]
pub struct MouseMove {
    x: i32,
    y: i32,
}
