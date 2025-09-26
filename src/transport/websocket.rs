use crate::{BridgeError, Message, Transport};
use async_trait::async_trait;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, WebSocketStream};

pub struct WebSocketTransport {
    url: String,
    connection: Option<WebSocketStream<TcpStream>>,
}

impl WebSocketTransport {
    pub fn new(url: String) -> Self {
        Self {
            url,
            connection: None,
        }
    }
}

#[async_trait]
impl Transport for WebSocketTransport {
    async fn connect(&mut self) -> Result<(), BridgeError> {
        // Implementation here
        todo!()
    }

    async fn disconnect(&mut self) -> Result<(), BridgeError> {
        // Implementation here
        todo!()
    }

    async fn rx(&mut self) -> Result<Message, BridgeError> {
        // Implementation here
        todo!()
    }

    async fn tx(&mut self, message: Message) -> Result<(), BridgeError> {
        // Implementation here
        todo!()
    }

    fn is_connected(&self) -> bool {
        self.connection.is_some()
    }
}
