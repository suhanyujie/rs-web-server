use crate::interface::iconnection::IConnection;
use crate::interface::idatapack::IDataPack;
use crate::interface::imessage::IMessage;
use crate::interface::irequest::IRequest;

use crate::thread_pool::lib::ThreadPool;
use crate::znet::datapack::DataPack;
use crate::znet::message::Message;
use crate::znet::request::{Request, UserConnection, UserMessage, UserRequest};

use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::{fmt::Debug, net::TcpStream};
use std::{
    fs,
    io::{Read, Write},
};

// 声明一个连接，其中包含
pub struct Connection {
    pub conn_id: u64,
    pub conn: TcpStream,
    pub handler_func: fn(&mut Self),
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

    fn get_mut_conn(&mut self) -> &mut TcpStream {
        return &mut self.conn;
    }

    pub fn handle1(&mut self) {
        let conn_id = self.conn_id;
        let stream  = &mut self.conn;
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
    pub fn start_reader(&mut self)  {
        let dp = DataPack::new();
        let mut req_data: Vec<u8> = vec![];
        let pool = if let Ok(pool) = ThreadPool::new(3) {
            pool
        } else {
            panic!("start_reader new threadpool failed.")
        };
        // 事实上，在这个循环中，需要对连接进行读操作，处理完数据后，再将结果写入连接中，因此这里的场景是，需要对一个变量进行读写操作
        // 因此，这里的场景可以抽象为，在一个循环中，对变量进行读写操作
        loop {
            match self.read_content() {
                (buff, cnt) => {
                    if cnt == 0 {
                        continue;
                    }
                    println!("server receive byte num is: {}", cnt);
                    println!("Received content: {}", String::from_utf8_lossy(&buff));
                    req_data = buff;
                }
                _ => {
                    // 如果没有读到内容，或者 eof，则退出循环 todo
                    break;
                },
            }
            // 通过 datapack 将数据处理成一个一个的 message
            let msg = dp.unpack(req_data.clone()); // as Box<UserMessage>
            // 把 message 组装成 request 对象
            // 通过一个新的线程池，进行处理 req todo
            // 处理 request，将 message 再发送回客户端
            // let mut req = Request::new(Arc::new(Mutex::new(self)), msg);
            // start writer immediately
            self.start_writer(msg);

            sleep(Duration::from_secs(3));
        }
    }

    // 从连接中读取数据
    fn read_content(&self) -> (Vec<u8>, usize) {
        let mut buffer = [0; 512];
        let mut cnt: usize = 0;
        if let Ok(tmp_cnt) = self.get_tcp_conn().read(&mut buffer) {
            cnt = tmp_cnt;
            println!("server receive byte num is: {}", cnt);
            println!("Received content: {}", String::from_utf8_lossy(&buffer));
        } else {
        }
        return (buffer.to_vec(), cnt);
    }

    // 向连接中写入数据
    fn write_content(&mut self, content: &[u8]) {
        let contents = String::from("hello world...");
        self.get_mut_conn().write(content).unwrap();
        self.get_mut_conn().flush().unwrap();
    }

    // 开始向连接中写入数据
    pub fn start_writer<'a>(&mut self, boxed_msg: Box<UserMessage>) {
        let data = &(*boxed_msg.data);
        let contents = String::from("hello world 111 ...");
        self.write_content(data);
        (self.handler_func)(self);
    }
}
