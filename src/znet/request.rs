use crate::interface::irequest::IRequest;

// 实现 IRequest 抽象
struct Request {
    conn: IConnection,
}

impl IRequest for Request<T: IConnection> {
    // 实例化 Request
    fn new(stream: TcpStream) -> Self {
        
        Request {
            conn: stream,
        }
    }

    // 获取连接
    fn get_connection() -> T 
    where T: IConnection
    {

    }
    // 获取请求消息
    fn get_data();
    // 获取消息id
    fn get_msg_id();
}


