use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::{any_service, MethodRouter},
};
use tower_http::services::{ServeDir, ServeFile};

const WEB_FOLDER: &str = "../frontend/dist";
const WEB_FILE: &str = "../frontend/dist/index.html";

// https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs
pub fn serve_dir() -> MethodRouter {
    // 页面路由交由前端进行处理，服务端仅需要把静态文件服务处理好即可
    any_service(ServeDir::new(WEB_FOLDER).not_found_service(ServeFile::new(WEB_FILE)))
}
