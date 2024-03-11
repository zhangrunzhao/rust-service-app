use crate::web::routes_login;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

use axum::Router;

pub use self::error::{Error, Result};

mod error;
mod web;

#[tokio::main] // 它会使 main 异步执行
async fn main() -> Result<()> {
    let routes_all = Router::new()
        .merge(routes_login::routes())
        // 引入 CookieManagerLayer 后，所有的路由中的函数的第一个参数都是 cookie，参考 api_login_handler
        // 或者是：https://docs.rs/tower-cookies/latest/tower_cookies/
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
