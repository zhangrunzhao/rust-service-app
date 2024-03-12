#![allow(unused)]
// region:    --- Modules

use crate::web::routes_login;
use model::ModelManager;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

use axum::{middleware, Router};
use web::{mw_auth::mw_ctx_resolve, mw_res_map::mw_response_map, routes_static};

pub use self::error::{Error, Result};

mod ctx;
mod error;
mod log;
mod model;
mod web;

// endregion: --- Modules

#[tokio::main] // 它会使 main 异步执行
async fn main() -> Result<()> {
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
        .fallback_service(routes_static::serve_dir());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
