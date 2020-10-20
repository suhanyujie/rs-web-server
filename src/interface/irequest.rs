use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::interface::iconnection::IConnection;

use crate::znet::request::{UserConnection, UserMessage};

// 将用户的请求包装到 Request 中
// 一个用户的请求中，我们一般需要从中获取连接、请求数据、消息id
pub trait IRequest<'a, C: IConnection> {
    // 获取连接
    fn get_connection(&self) -> &Arc<Mutex<&'a mut UserConnection>>;
    // 获取请求消息
    fn get_data(&self);
}
