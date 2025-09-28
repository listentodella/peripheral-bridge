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
    info!("test start");
    loop {
        let tx_msg0 = Msg {
            transport: TransportType::Cdc as i32,
            bus: BusType::I2c as i32,
            seqs: vec![BusOps {
                operation: Operation::Read as i32,
                address: 0x50,
                data: Some(vec![1, 2, 3, 4, count]),
                delay_us: Some(1000),
            }],
        };
        let tx_msg1 = Msg {
            transport: TransportType::Cdc as i32,
            bus: BusType::Spi as i32,
            seqs: vec![
                BusOps {
                    operation: Operation::Read as i32,
                    address: 0x51,
                    data: Some(vec![count, 4, 3, 2, 1]),
                    delay_us: Some(100),
                },
                BusOps {
                    operation: Operation::Write as i32,
                    address: 0x51,
                    data: Some(vec![count, count, count, count, count]),
                    delay_us: Some(100),
                },
            ],
        };
        ws_transport.tx(&[tx_msg0, tx_msg1]).await?;
        let rx_msgs = ws_transport.rx().await?;
        debug!("Received message: {:?}", rx_msgs);
        count = count.wrapping_add(1);
        if count >= 255 {
            break;
        }
    }
    info!("test done");
    Ok(())
}
