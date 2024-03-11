# rust-service-app
瓜皮钊的第一个 rust 服务器项目

## backend
1. 每个模块都有自己的 error 子模块


### 开发环境运行时
服务端：cargo watch -q -c -w src/ -w .cargo/ -x "run"
客户端测试：cargo watch -q -c -w examples/ -x "run --example quick_dev"

