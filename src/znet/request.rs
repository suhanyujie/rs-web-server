use crate::interface::iconnection::IConnection;
use crate::interface::imessage::IMessage;
use crate::interface::irequest::IRequest;

use crate::znet::connection::Connection;
use crate::znet::message::Message;

use std::sync::{Arc, Mutex};

// 取一个别名，作为一种抽象，防止后面需要调整 IConnection、IMessage 的具体实现
pub type UserConnection = Connection;
pub type UserMessage = Message;
pub type UserRequest<'a> = Request<'a>;


// 实现 IRequest 抽象
pub struct Request<'a> {
    pub conn: Arc<Mutex<&'a mut UserConnection>>,
    pub msg: Arc<Mutex<Box<UserMessage>>>,
}

impl<'a, C> IRequest<'a, C> for Request<'a>
where
    C: 'a + IConnection,
{
    // 获取连接
    fn get_connection(&self) -> &Arc<Mutex<&'a mut UserConnection>> {
        return &self.conn;
    }
    // 获取请求消息
    fn get_data(&self) {
        todo!();
    }
}

impl<'a> Request<'a> {
    pub fn new(conn: Arc<Mutex<&'a mut UserConnection>>, msg: Arc<Mutex<Box<UserMessage>>>) -> Request {
        Request { conn, msg }
    }
}
