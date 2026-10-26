use serde::{Deserialize, Serialize};
use trl::Trl;

#[derive(Serialize, Deserialize, Trl)]
#[getters]
#[constructor]
pub struct TryRoomResponse {
    exists: bool,
}
