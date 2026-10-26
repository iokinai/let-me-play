use std::{collections::HashMap, sync::Arc};

use let_me_play_models::Event;
use tokio::sync::{RwLock, mpsc::Receiver};
use trl::Trl;

use crate::room::Room;

#[derive(Trl, Clone)]
#[constructor]
#[getters]
pub struct AppState {
    rooms: Arc<RwLock<HashMap<String, Room>>>,
    receivers: Arc<RwLock<HashMap<String, Receiver<Event>>>>,
}
