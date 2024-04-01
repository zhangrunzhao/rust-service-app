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
#### 后端开发
在根目录执行 cd backend
1. 服务端开发: cargo watch -q -c -w src/ -w .cargo/ -x "run"
2. 客户端测试: cargo watch -q -c -w examples/ -x "run --example quick_dev"
3. 单元测试: cargo watch -q -c -x "test -- --nocapture"

#### 前端开发
在根目录执行 cd frontend
1. 执行 pnpm dev 进入到开发模式（调用接口时需要同时调用服务端开发的第一项）
2. 执行 pnpm watch 进行自测（调用接口时需要同时调用服务端开发的第一项）

## 本地连接数据库
export PATH=/Library/PostgreSQL/16/bin:$PATH
export DATABASE_URL=postgres://postgres:321chenjixink@localhost:5432/postgres?sslmode=disable
psql $DATABASE_URL
