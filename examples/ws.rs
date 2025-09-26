use axum::{
    body::Bytes,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    response::IntoResponse,
    routing::any,
    Extension, Router,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    // 加载config.toml
    //ws://10.2.12.169:8765
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8765").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await?;
        let mut std_tcp_stream = socket.into_std()?;
    }

    // if let Err(e) = axum::serve(listener, routes().await).await {
    //     log::error!("Server error: {}", e);
    // } else {
    //     log::warn!("Server exit");
    // }
}

// async fn routes() -> Router {
//     log::info!("Start ");
//     println!("hello");
//     let mut router = Router::new().route("/ws/{id}", any(ws_handler));

//     router
// }

// async fn ws_handler() -> impl IntoResponse {
//     let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<u8>();
//     log::info!("ws_handler tx: {:?}", tx);
//     println!("world");
//     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
// }
