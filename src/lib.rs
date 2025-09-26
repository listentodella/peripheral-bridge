use async_trait::async_trait;

pub mod bus;
pub mod error;
pub mod pb;
pub mod transport;

use error::BridgeError;
use pb::Msg;

/// Core trait for transport layer implementations
/// 定义了与外设通信的基本操作, 如连接、断开连接、接收数据和发送数据
/// 上位机与下位机都需要实现该 trait, 以达到通信的目的
/// 因此该 trait 不区分上位机或下位机, 定义了通用的通信操作
#[async_trait]
pub trait Transport {
    /// Establish connection
    async fn connect(&mut self) -> Result<(), BridgeError>;

    /// Close connection
    async fn disconnect(&mut self) -> Result<(), BridgeError>;

    /// Receive msg from host
    async fn rx(&mut self) -> Result<Msg, BridgeError>;

    /// Send msg to host
    async fn tx(&mut self, msg: Msg) -> Result<(), BridgeError>;

    /// Check connection status
    fn is_connected(&self) -> bool;
}

/// Core trait for peripheral bus implementations
#[async_trait]
pub trait PeripheralBus {
    /// Write data to peripheral
    async fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), BridgeError>;

    /// Read data from peripheral
    async fn read(&mut self, addr: u8, len: usize) -> Result<Vec<u8>, BridgeError>;

    /// Perform read-write transaction
    async fn transfer(
        &mut self,
        addr: u8,
        write: &[u8],
        read_len: usize,
    ) -> Result<Vec<u8>, BridgeError>;
}
