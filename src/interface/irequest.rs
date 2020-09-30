use std::net::TcpStream;

use crate::interface::iconnection::IConnection;

// 将用户的请求包装到 Request 中
// 一个用户的请求中，我们一般需要从中获取连接、请求数据、消息id
pub trait IRequest<T: IConnection> {
    fn new(stream: TcpStream) -> Self;
    // 获取连接
    fn get_connection() -> T;
    // 获取请求消息
    fn get_data();
    // 获取消息id
    fn get_msg_id();
}
