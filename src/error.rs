use thiserror::Error;

#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("Transport error: {0}")]
    TransportError(String),

    #[error("Bus error: {0}")]
    BusError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Invalid msg format: {0}")]
    MsgError(String),

    #[error("Operation timeout")]
    Timeout,
}
