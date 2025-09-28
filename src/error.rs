use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum BridgeError {
    #[error("Transport error: {0}")]
    TransportError(#[from] std::io::Error), // 直接包裹 io 错误

    #[error("WebSocket error: {0}")]
    WsError(#[from] tokio_websockets::Error),

    #[error("Bus error: {0}")]
    BusError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Invalid msg format: {0}")]
    MsgError(String),

    #[error("Operation timeout")]
    Timeout,

    #[error("Pb Decode Error: {0}")]
    PbDeError(#[from] prost::DecodeError),
    #[error("Pb Encode Error: {0}")]
    PbEnError(#[from] prost::EncodeError),

    #[error(transparent)]
    Other(#[from] anyhow::Error), // 兜底，任何其他可转换为anyhow::Error的错误
}
