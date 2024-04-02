#![allow(unused)]
// region:    --- Modules

use crate::web::routes_login;
use model::ModelManager;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

use axum::{middleware, Router};
use web::{mw_auth::mw_ctx_resolve, mw_res_map::mw_response_map, routes_static};

pub use self::error::{Error, Result};
pub use config::config;

mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

// endregion: --- Modules

#[tokio::main] // 它会使 main 异步执行
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    // 洋葱模型，写在后面的函数越早执行
    let routes_all = Router::new()
        .merge(routes_login::routes())
        // 这个中间件主要是用于处理返回到客户端中的 res body
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        // 引入 CookieManagerLayer 后，所有的路由中的函数的第一个参数都是 cookie，参考 api_login_handler
        // 或者是：https://docs.rs/tower-cookies/latest/tower_cookies/
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir())
        .layer(
            // 解决跨域问题
            CorsLayer::new()
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                .allow_origin(Any)
                .allow_headers(Any),
        );

    let port = 8080;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("正在尝试开启服务，请稍后访问 127.0.0.1:{} ", port);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
