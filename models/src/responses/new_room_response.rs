use serde::{Deserialize, Serialize};
use trl::Trl;

#[derive(Serialize, Deserialize, Trl)]
#[constructor]
#[getters]
pub struct NewRoomResponse {
    id: String,
}
