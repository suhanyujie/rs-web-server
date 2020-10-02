extern crate ctrlc;

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use web_server::thread_pool::lib::ThreadPool;
use web_server::znet::server::Server;
use web_server::interface::iserver::IServer;

fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let s1 = Server::new("[Rust zinx server v0.1]".to_string());
    s1.serve()
}

fn server() {
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
}
