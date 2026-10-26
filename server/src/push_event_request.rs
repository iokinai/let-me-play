use let_me_play_models::Event;
use serde::{Deserialize, Serialize};
use trl::Trl;

#[derive(Trl, Serialize, Deserialize)]
#[constructor]
pub struct PushEventRequest {
    #[get]
    room_id: String,
    #[get(move)]
    event: Event,
}
