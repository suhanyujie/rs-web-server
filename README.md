# rs-web-server
Rust 编写的基于线程池的示例 web 服务器

## branch desc
### v0.1.1
这个分支的 feature 是参考基于 go 语言的 [zinx 服务器](https://github.com/suhanyujie/zinxStudy1)的实现，加入解决粘包问题的处理逻辑。

#### 一些疑问
* 声明 trait 中的方法时，方法第一个形参为什么必须是 `self: Self`？
* 为什么可以使用 `use web_server::thread_pool::lib::ThreadPool;`，却不能使用 `use crate::thread_pool::lib::ThreadPool;`？

## reference
* https://kaisery.github.io/trpl-zh-cn/
* crossbeam crate https://rustcc.cn/article?id=fdcca4e9-7552-43a8-86c2-589011129b82
* 类似于 crossbeam 的库 https://github.com/stjepang/async-channel