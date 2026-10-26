use thiserror::Error;

use crate::error::{ServerError, handler_error::HandlerError};

#[derive(Debug, Error)]
pub enum LmpError {
    #[error("Handler error: {0}")]
    HandlerError(HandlerError),
    #[error("Server error: {0}")]
    ServerError(ServerError),
}
