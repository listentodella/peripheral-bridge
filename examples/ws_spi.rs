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

    info!("test start");

    let chip_id_msg = Msg {
        transport: TransportType::WebSocket as i32,
        bus: BusType::Spi as i32,
        seqs: vec![BusOps {
            operation: Operation::Read as i32,
            address: 0x00,
            data: Some(vec![0x00; 5]),
            delay_us: Some(100),
        }],
    };
    ws_transport.tx(&[chip_id_msg]).await?;
    let rx_msgs = ws_transport.rx().await?;
    debug!("get chip id: {:x?}", rx_msgs);

    let reset_msg = Msg {
        transport: TransportType::WebSocket as i32,
        bus: BusType::Spi as i32,
        seqs: vec![
            BusOps {
                operation: Operation::Write as i32,
                address: 0x36,
                data: Some(vec![0xb6]),
                delay_us: Some(1000 * 200),
            },
            BusOps {
                operation: Operation::Read as i32,
                address: 0x33,
                data: Some(vec![0x00; 2]),
                delay_us: Some(1000 * 200),
            },
        ],
    };
    ws_transport.tx(&[reset_msg]).await?;
    let rx_msgs = ws_transport.rx().await?; //for read ack
    debug!("reset done: {:x?}", rx_msgs);

    let init_msg = Msg {
        transport: TransportType::WebSocket as i32,
        bus: BusType::Spi as i32,
        seqs: vec![
            BusOps {
                operation: Operation::Write as i32,
                address: 0x57,
                data: Some(vec![0x07]),
                delay_us: Some(100),
            },
            BusOps {
                operation: Operation::Write as i32,
                address: 0x10,
                data: Some(vec![0x62]),
                delay_us: Some(100),
            },
            BusOps {
                operation: Operation::Write as i32,
                address: 0x0F,
                data: Some(vec![0x02]),
                delay_us: Some(100),
            },
            BusOps {
                operation: Operation::Write as i32,
                address: 0x11,
                data: Some(vec![0xC5]),
                delay_us: Some(100),
            },
            BusOps {
                operation: Operation::Write as i32,
                address: 0x3e,
                data: Some(vec![0x47]),
                delay_us: Some(100),
            },
            BusOps {
                operation: Operation::Write as i32,
                address: 0x31,
                data: Some(vec![32]),
                delay_us: Some(100),
            },
        ],
    };
    ws_transport.tx(&[init_msg]).await?;
    // ws_transport.rx().await?; //for ack

    loop {
        let data_msg = vec![Msg {
            transport: TransportType::WebSocket as i32,
            bus: BusType::Spi as i32,
            seqs: vec![BusOps {
                operation: Operation::Read as i32,
                address: 0x01,
                data: Some(vec![0x00; 6]),
                delay_us: Some(100),
            }],
        }];
        ws_transport.tx(&data_msg).await?;
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
