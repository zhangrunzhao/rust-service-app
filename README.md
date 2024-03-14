# rust-service-app
瓜皮钊的第一个 rust 服务器项目

## backend
1. 每个模块都有自己的 error 子模块


### 开发环境运行时
<!-- 
    -q --quiet，表示不打印一些多余的 cargo watch 相关日志
    -c --clear，表示每次执行清空终端信息
    -w --watch [目录]，监控下面的目录，如果发生改变则重新执行
    -x --execute [命令]，执行后面的命令。
-->
1. 服务端：cargo watch -q -c -w src/ -w .cargo/ -x "run"
2. 客户端测试：cargo watch -q -c -w examples/ -x "run --example quick_dev"

