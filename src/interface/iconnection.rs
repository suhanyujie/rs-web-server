//! 抽象 tcp 连接的接口
//! 
use std::net::TcpStream;


pub trait IConnection {
    fn start(&mut self);
    fn stop(self);
    fn get_tcp_conn(&self) -> &TcpStream;
    fn get_conn_id(&self) -> u64;
    fn remote_addr(&self) -> std::net::SocketAddr;
    fn send_msg(&self, msg_id: u32, data: Vec<u8>) -> Result<(), String>;
}
