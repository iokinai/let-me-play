use std::time::Instant;

use tokio::sync::mpsc::Sender;

use let_me_play_models::Event;
use trl::Trl;

#[derive(Trl, Clone)]
#[constructor]
#[getters(option)]
pub struct Room {
    sender: Sender<Event>,
    #[set]
    last_sent: Option<Instant>,
}
