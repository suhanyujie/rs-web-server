//! 抽象 tcp 连接的接口
//! 
use std::net::TcpStream;


pub trait IConnection {
    fn start();
    fn stop();
    fn get_tcp_conn() -> TcpStream;
    fn get_conn_id() -> u64;
    fn remote_addr() -> String;
    fn send_msg(msg_id: u32, data: Vec<u8>) -> Result<(), String>;

}