use thiserror::Error;

#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("Transport error: {0}")]
    TransportError(String),

    #[error("Bus error: {0}")]
    BusError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Invalid message format: {0}")]
    MessageError(String),

    #[error("Operation timeout")]
    Timeout,
}
