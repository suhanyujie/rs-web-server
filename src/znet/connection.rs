use std::net::TcpStream;

use crate::interface::iconnection::IConnection;

// 声明一个连接，其中包含
#[derive(Debug)]
pub struct Connection{
    pub conn_id: u64,
    pub conn: TcpStream,
}

impl IConnection for Connection {
    fn start() {
        todo!()
    }
    fn stop() {
        todo!()
    }
    fn get_tcp_conn() -> TcpStream {
        todo!()
    }
    fn get_conn_id() -> u64 {
        todo!()
    }
    fn remote_addr() -> String {
        todo!()
    }
    fn send_msg(msg_id: u32, data: Vec<u8>) -> Result<(), String> {
        todo!()
    }
}

impl Connection {
    pub fn new(conn_id: u64, stream: TcpStream) -> Connection {
        Connection {
            conn_id,
            conn: stream,
        }
    }
}