use crate::interface::iconnection::IConnection;
use crate::interface::imessage::IMessage;
use crate::interface::irequest::IRequest;

use std::sync::{Arc, Mutex};

// 实现 IRequest 抽象
pub struct Request<'a> {
    pub conn: Arc<Mutex<&'a mut dyn IConnection>>,
    pub msg: Arc<Mutex<Box<dyn IMessage>>>,
}

impl<'a, C> IRequest<'a, C> for Request<'a>
where
    C: 'a + IConnection,
{
    // 获取连接
    fn get_connection(&self) -> &Arc<Mutex<&'a mut dyn IConnection>> {
        return &self.conn;
    }
    // 获取请求消息
    fn get_data(&self) {
        todo!();
    }
}

impl<'a> Request<'a> {
    pub fn new(conn: Arc<Mutex<&'a mut dyn IConnection>>, msg: Arc<Mutex<Box<dyn IMessage>>>) -> Request {
        Request { conn, msg }
    }
}
