use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use crate::interface::iserver::IServer;
use crate::thread_pool::lib::ThreadPool;

// 声明一个服务器类型
pub struct Server {
    pub name: String,
    pub ip_version: String,
    pub ip: String,
    pub port: i32,
}

// 为该服务器类型实现 IServer trait
impl IServer for Server {

    // 实例化服务器
    fn new(name: String) -> Server {
        Server {
            name,
            ip_version: "tcp4".to_string(),
            ip: "127.0.0.1".to_string(),
            port: 3001,
        }
    }

    // 服务器启动服务
    fn serve(self: Server) {
        println!("server is serving...");
        let host = "127.0.0.1";
        let port = 8000;
        println!("server start in port: {}", port);
        let listener = TcpListener::bind((host, port)).unwrap();
        let pool = if let Ok(pool) = ThreadPool::new(4) {
            pool
        } else {
            panic!("new threadpool failed.")
        };
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(move || {
                handle_connection(stream);
            })
        }
    }

    // 开始处理连接
    fn start() {}

    // 停止服务
    fn stop() {}
}

impl Server {
}

// 处理连接
fn handle_connection(mut stream: TcpStream) {
    let get = b"GET / HTTP/1.1\n";
    let sleep = b"GET /sleep HTTP/1.1\n";

    // 缓冲区需要足够大，才能读取到请求的所有内容，从而正常响应客户端
    // 如果不够大，就会导致响应客户端异常
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

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
    println!("{}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // 也可以直接调用 shutdown 方法显式地关闭连接
    // stream.shutdown(std::net::Shutdown::Both)
}
