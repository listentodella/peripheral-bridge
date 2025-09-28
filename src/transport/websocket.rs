use crate::pb::MsgBatch;
use crate::{BridgeError, Msg, Transport};
use anyhow::Result;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use log::trace;
use prost::Message;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_websockets::WebSocketStream;
use tokio_websockets::{Message as WsMessage, ServerBuilder};
pub struct WebSocketTransport {
    url: String,
    connection: Option<WebSocketStream<TcpStream>>,
}

impl WebSocketTransport {
    pub async fn new(url: String) -> Result<Self, BridgeError> {
        let mut ws_transport = Self {
            url,
            connection: None,
        };

        ws_transport.connect().await?;

        Ok(ws_transport)
    }
}

#[async_trait]
impl Transport for WebSocketTransport {
    async fn connect(&mut self) -> Result<()> {
        trace!("try to connect {}", self.url);
        let listener = TcpListener::bind(&self.url).await?;
        let (conn, _) = listener.accept().await?;
        let (_request, ws_stream) = ServerBuilder::new().accept(conn).await?;
        self.connection = Some(ws_stream);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        if let Some(mut ws_stream) = self.connection.take() {
            // 发送关闭帧
            ws_stream.close().await?;
            // 等待连接完全关闭
            while let Some(Ok(item)) = ws_stream.next().await {
                // 处理剩余消息直到连接关闭
                trace!("Received: {:?}", item);
            }
        }

        self.connection = None;
        Ok(())
    }

    async fn rx(&mut self) -> Result<MsgBatch> {
        let ws_msg = self
            .connection
            .as_mut()
            .ok_or(BridgeError::ConnectionError("connection closed".into()))?
            .next()
            .await
            .ok_or(BridgeError::ConnectionError("websocket closed".into()))??;

        let buffer = ws_msg.into_payload().to_vec();
        let msgs = MsgBatch::decode(buffer.as_slice())?;
        Ok(msgs)
    }

    async fn tx(&mut self, msgs: &[Msg]) -> Result<()> {
        let batch = MsgBatch {
            msgs: msgs.to_vec(),
        };
        let buffer = batch.encode_to_vec();

        self.connection
            .as_mut()
            .ok_or(BridgeError::ConnectionError("connection closed".into()))?
            .send(WsMessage::binary(buffer))
            .await?;

        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connection.is_some()
    }
}
