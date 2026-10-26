use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("could not create room")]
    CouldNotCreateRoom,
    #[error("websocket connection failed")]
    WebSocketConnectError,
    #[error("websocket error")]
    WebSocketError,
    #[error("invalid event")]
    InvalidEvent,
}
