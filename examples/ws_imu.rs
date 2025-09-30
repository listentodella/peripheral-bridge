use log::{debug, error, info};
use peripheral_bridge::{pb::msg::*, transport::WebSocketTransport, Transport};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
struct ImuConfig {
    name: String,
    detect: MsgBatch,
    reset: MsgBatch,
    init: MsgBatch,
    data: MsgBatch,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    info!("test start");
    // let mut imu_config = ImuConfig::default();
    let json_file = std::fs::read_to_string("./assets/ws_spi.json")?;
    let imu_config: ImuConfig = serde_json::from_str(&json_file)?;

    // let json_str = serde_json::to_string_pretty(&imu_config).unwrap();
    // debug!("imu config: {}", json_str);

    let mut ws_transport = WebSocketTransport::new("0.0.0.0:8765".to_string()).await?;

    if !ws_transport.is_connected() {
        error!("WebSocket transport is not connected");
        return Ok(());
    }

    ws_transport.tx(&imu_config.detect.msgs).await?;
    let rx_msgs = ws_transport.rx().await?;
    debug!("get chip id: {:x?}", rx_msgs);

    ws_transport.tx(&imu_config.reset.msgs).await?;
    let rx_msgs = ws_transport.rx().await?; //for read ack
    debug!("reset done: {:x?}", rx_msgs);

    ws_transport.tx(&imu_config.init.msgs).await?;
    // ws_transport.rx().await?; //for ack
    debug!("init done: {:x?}", rx_msgs);

    loop {
        ws_transport.tx(&imu_config.data.msgs).await?;
        let rx_msgs = ws_transport.rx().await?;
        // debug!("data: {:x?}", rx_msgs);

        let data = rx_msgs.msgs[0].to_owned().seqs[0].to_owned().data.unwrap();
        let (x, y, z) = (
            i16::from_le_bytes([data[0], data[1]]),
            i16::from_le_bytes([data[2], data[3]]),
            i16::from_le_bytes([data[4], data[5]]),
        );
        let rsl = 9.807f32 * 4.0 * 2.0 / 65535.0;
        let acc = (x as f32 * rsl, y as f32 * rsl, z as f32 * rsl);
        debug!("acc: {:?}", acc);
    }
    info!("test done");
    Ok(())
}
