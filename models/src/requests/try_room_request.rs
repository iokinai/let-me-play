use serde::{Deserialize, Serialize};
use trl::Trl;

#[derive(Serialize, Deserialize, Trl)]
#[getters]
#[setters]
pub struct TryRoomRequest {
    id: String,
}
