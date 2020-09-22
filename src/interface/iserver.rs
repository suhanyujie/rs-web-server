// server 的抽象
pub trait IServer {
    // 服务器的构造方法
    fn new(name: String) -> Self;
    // 服务器启动服务
    fn serve(self: Self);
    // 开始处理连接
    fn start();
    // 停止服务
    fn stop();
}