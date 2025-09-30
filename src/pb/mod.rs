pub mod msg;
pub use msg::*;
// re-export prost
pub use prost;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_proto_types_to_json() {
        let transport = TransportType::Cdc;
        let bus = BusType::I2c;
        let operation = Operation::Read;

        let transport = serde_json::to_string(&transport).unwrap();
        let bus = serde_json::to_string(&bus).unwrap();
        let operation = serde_json::to_string(&operation).unwrap();
        println!("transport: {}", transport);
        println!("bus: {}", bus);
        println!("operation: {}", operation);
    }

    #[test]
    fn test_proto_msg_json_conversion() {
        // 创建 Protobuf 消息
        let proto_msg = Msg {
            transport: TransportType::Cdc as i32,
            bus: BusType::I2c as i32,
            seqs: vec![BusOps {
                operation: Operation::Read as i32,
                address: 0x50,
                data: Some(vec![1, 2, 3, 4, 5]),
                delay_us: Some(1000),
            }],
        };

        // 转换为 JSON
        let json = serde_json::to_string(&proto_msg).expect("Failed to convert to JSON");
        println!("converted JSON: {}", json);

        // 从 JSON 解析回 Protobuf 消息
        let json_str = r#"{"transport":0,"bus":0,"seqs":[{"operation":1,"address":80,"data":[1,2,3,4,5],"delay_us":1000}]}"#;
        let parsed_msg: Msg = serde_json::from_str(json_str).expect("Failed to parse from JSON");

        // 验证字段
        assert_eq!(proto_msg.transport, parsed_msg.transport);
        assert_eq!(proto_msg.bus, parsed_msg.bus);
        assert_eq!(proto_msg.seqs[0].operation, parsed_msg.seqs[0].operation);
        assert_eq!(proto_msg.seqs[0].address, parsed_msg.seqs[0].address);
        assert_eq!(proto_msg.seqs[0].data, parsed_msg.seqs[0].data);
        assert_eq!(proto_msg.seqs[0].delay_us, parsed_msg.seqs[0].delay_us);
    }

    #[test]
    fn test_batch_msg() {
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

        let json = serde_json::to_string_pretty(&init_msg).expect("Failed to convert to JSON");
        println!("converted JSON: {}", json);
    }
}
