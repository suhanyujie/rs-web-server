extern crate chrono;

use chrono::prelude::*;

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::interface::iconnection::IConnection;
use crate::interface::iserver::IServer;
use crate::thread_pool::lib::ThreadPool;
use crate::znet::connection::Connection;

// 声明一个服务器类型
pub struct Server {
    pub name: String,
    pub ip_version: String,
    pub ip: String,
    pub port: i32,
}

// 为该服务器类型实现 IServer trait
impl IServer for Server {
    // 服务器启动服务
    fn serve(self: Server) {
        println!("server is serving...");
        let host = "127.0.0.1";
        let port = 8000;
        println!("server start in port: {}", port);
        // 初始化 work 线程池
        let pool = if let Ok(pool) = ThreadPool::new(3) {
            pool
        } else {
            panic!("new threadpool failed.")
        };
        let listener = TcpListener::bind((host, port)).unwrap();
        // listener.set_nonblocking(true).expect("Cannot set non-blocking");
        let mut conn_id: u64 = 0;
        for stream in listener.incoming() {
            conn_id += 1;
            let stream = stream.unwrap();
            // 将 stream 包装成 connection
            let mut conn = Connection::new(conn_id, stream);
            pool.execute(move || {
                // handle_tcp_connection(conn_id, stream);
                conn.start();
            });

            // 启动线程处理 work
            // thread::spawn(move || {
            //     handle_connection(stream);
            // });
            println!("after get conn: {}", conn_id);
        }
    }

    // 开始处理连接
    fn start() {}

    // 停止服务
    fn stop() {}
}

impl Server {
    // 实例化服务器
    pub fn new(name: String) -> Server {
        // 处理 tcp 连接
        Server {
            name,
            ip_version: "tcp4".to_string(),
            ip: "127.0.0.1".to_string(),
            port: 3001,
        }
    }
}

// 处理连接
fn handle_connection(mut stream: TcpStream) {
    let get = b"GET / HTTP/1.1\n";
    let sleep = b"GET /sleep HTTP/1.1\n";

    // 缓冲区需要足够大，才能读取到请求的所有内容，从而正常响应客户端
    // 如果不够大，就会导致响应客户端异常
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    // println!("{}", response);
    stream.write(response.as_bytes()).unwrap();
    std::thread::sleep(Duration::from_secs(8));
    stream.flush().unwrap();
    let t1: DateTime<Local> = Local::now();
    println!("{:?}", t1);
    // 也可以直接调用 shutdown 方法显式地关闭连接
    // stream.shutdown(std::net::Shutdown::Both)
}
