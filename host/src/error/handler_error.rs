use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("could not send input")]
    CouldNotSendInput,
    #[error("key is not supported")]
    KeyIsNotSupported,
}
