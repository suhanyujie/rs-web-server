use crate::interface::irequest::IRequest;
use crate::interface::iconnection::IConnection;

// 实现 IRequest 抽象
#[derive(Debug)]
pub struct Request<'a, C: IConnection> {
    pub conn: &'a C,
    pub data: Vec<u8>,
}

impl<'a, C> IRequest<C> for Request<'a, C>
where
    C: 'a + IConnection,
{
    // 获取连接
    fn get_connection(&self) -> &C  {
        return self.conn;
    }
    // 获取请求消息
    fn get_data(&self) {
        todo!();
    }
}

impl<'a, C> Request<'a, C> 
where 
    C: IConnection
{
    pub fn new(conn: &C, data: Vec<u8>) -> Request<C>{
        Request {
            conn,
            data,
        }
    }
}
