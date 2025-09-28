use log::{debug, error, info};
use peripheral_bridge::{pb::msg::*, transport::WebSocketTransport, Transport};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut ws_transport = WebSocketTransport::new("0.0.0.0:8765".to_string()).await?;

    if !ws_transport.is_connected() {
        error!("WebSocket transport is not connected");
        return Ok(());
    }
    let mut count = 0u8;
    loop {
        let tx_msg = Msg {
            transport: TransportType::Cdc as i32,
            bus: BusType::I2c as i32,
            operation: Operation::Read as i32,
            address: 0x50,
            data: Some(vec![1, 2, 3, 4, count]),
            delay_us: Some(1000),
        };
        ws_transport.tx(tx_msg).await?;
        let rx_msg = ws_transport.rx().await?;
        debug!("Received message: {:?}", rx_msg);
        count += 1;
        if count >= 10 {
            break;
        }
    }
    info!("test done");
    Ok(())
}
