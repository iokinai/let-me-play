use serde::{Deserialize, Serialize};
use trl::Trl;

#[derive(Trl, Serialize, Deserialize)]
#[constructor]
#[getters]
pub struct WsConnectRequest {
    code: String,
}
