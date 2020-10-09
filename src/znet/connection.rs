use crate::interface::iconnection::IConnection;
use crate::interface::idatapack::IDataPack;
use crate::interface::imessage::IMessage;

use crate::znet::request::Request;
use crate::znet::datapack::DataPack;
use crate::znet::message::Message;

use std::{fmt::Debug, net::TcpStream};
use std::thread::sleep;
use std::time::Duration;
use std::io::Read;

// 声明一个连接，其中包含
pub struct Connection{
    pub conn_id: u64,
    pub conn: TcpStream,
    pub handler_func: fn(u64, &mut TcpStream),
}

impl IConnection for Connection {
    fn start(&mut self) {
        println!("new connection is established...");
        self.start_reader();
        // 处理连接
        // (self.handler_func)(self.conn_id, &mut self.conn);
    }
    // 停止当前连接
    fn stop(self) {
        drop(self.conn);
    }
    fn get_tcp_conn(&self) -> &TcpStream {
        return &self.conn;
    }
    fn get_conn_id(&self) -> u64 {
        return self.conn_id;
    }
    fn remote_addr(&self) -> std::net::SocketAddr {
        return self.conn.peer_addr().unwrap();
    }
    fn send_msg(&self, msg_id: u32, data: Vec<u8>) -> Result<(), String> {
        todo!()
    }
}

impl Connection {
    pub fn new(conn_id: u64, stream: TcpStream) -> Connection {
        Connection {
            conn_id,
            conn: stream,
            handler_func: Connection::handle1,
        }
    }

    pub fn handle1(conn_id: u64, stream: &mut TcpStream){
        // 在循环体中 read 客户端过来的数据，再将其写回客户端
        let mut buffer = [0; 1024];
        loop {
            buffer = [0; 1024];
            // 缓冲区需要足够大，才能读取到请求的所有内容，从而正常响应客户端
            // 如果不够大，就会导致响应客户端异常
            // stream.read(&mut buffer).unwrap();
            // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
            // stream.write(&buffer).unwrap();
            // stream.flush().unwrap();
            sleep(Duration::from_secs(2));
            println!("this is test from conn {}...", conn_id);
        }
    }

    // 读取请求连接中的数据
    // 将其封装成 request
    pub fn start_reader(&self){
        let mut conn = self.get_tcp_conn();
        let mut buffer = [0 as u8; 1024];
        loop {
            let cnt = conn.read(&mut buffer).unwrap();
            println!("read data from conn {} data is: {:?}", self.conn_id, String::from_utf8_lossy(&buffer[..cnt]));
            let req_data: Vec<u8> = buffer[..cnt].to_vec();
            let dp = DataPack::new();
            // 通过 datapack 将数据处理成一个一个的 message
            let msg = dp.unpack(req_data);
            // 把 message 组装成 request 对象
            // 这里的 msg data 应该被拷贝了一份，待优化
            let req = Request::new(self, msg.get_data().to_vec());
            println!("the conn id is: {:?}", req.conn.get_conn_id());
            // 将 message 再发送回客户端
            // todo
            sleep(Duration::from_secs(3));
        }
    }
}
