use futures_util::{SinkExt, StreamExt};
use log::{debug, info};
use tokio::net::TcpListener;
use tokio_websockets::{Message, ServerBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:8765").await?;
    info!("Listening on 0.0.0.0:8765");
    loop {
        let (conn, _) = listener.accept().await?;
        let (_request, mut ws_stream) = ServerBuilder::new().accept(conn).await?;
        ws_stream.send(Message::text("Hello, client!")).await?;
        while let Some(Ok(item)) = ws_stream.next().await {
            if item.is_binary() || item.is_text() {
                debug!("Received: {:?}", item);
                ws_stream.send(item).await?;
            }
        }
    }

    // Ok(())
}
