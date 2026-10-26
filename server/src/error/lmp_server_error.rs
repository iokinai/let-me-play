use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum LmpServerError {
    #[error("Invalid room")]
    InvalidRoom,
    #[error("Could not send event")]
    CouldNotSend,
    #[error("Cound not receive event")]
    CouldNotReceive,
    #[error("Could not serialize event")]
    CouldNotSerialize,
    #[error("Sending is on cooldown")]
    Cooldown,
}

impl IntoResponse for LmpServerError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::InvalidRoom => StatusCode::NOT_FOUND,

            Self::CouldNotSend | Self::CouldNotReceive | Self::CouldNotSerialize => {
                StatusCode::INTERNAL_SERVER_ERROR
            }

            Self::Cooldown => StatusCode::BAD_REQUEST,
        };

        let error_json = json!({
            "type": self,
            "message": self.to_string()
        });

        (status, Json(error_json)).into_response()
    }
}
