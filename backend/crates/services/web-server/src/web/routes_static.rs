use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::{any_service, MethodRouter},
};
use tower_http::services::{ServeDir, ServeFile};

use crate::config::{self, web_config};

// https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs
pub fn serve_dir() -> MethodRouter {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Resource not found.")
    }

    let config = &web_config();

    // 页面路由交由前端进行处理，服务端仅需要把静态文件服务处理好即可
    any_service(
        ServeDir::new(&config.WEB_FOLDER).not_found_service(ServeFile::new(&config.WEB_FILE)),
    )
}
