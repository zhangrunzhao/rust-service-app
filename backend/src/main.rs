#![allow(unused)]
// region:    --- Modules

use crate::web::{mw_auth::mw_ctx_require, routes_login, rpc};
use model::ModelManager;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use axum::{middleware, response::Html, routing::get, Router};
use web::{mw_auth::mw_ctx_resolve, mw_res_map::mw_response_map, routes_static};

pub use self::error::{Error, Result};
pub use config::config;

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;

pub mod _dev_utils;

// endregion: --- Modules

#[tokio::main] // 它会使 main 异步执行
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- 仅为了测试
	_dev_utils::init_dev().await;

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	let routes_rpc =
		rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

	// 洋葱模型，写在后面的函数越早执行
	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		.nest("/api", routes_rpc)
		// 这个中间件主要是用于处理返回到客户端中的 res body
		.layer(middleware::map_response(mw_response_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		// 引入 CookieManagerLayer 后，所有的路由中的函数的第一个参数都是 cookie，参考 api_login_handler
		// 或者是：https://docs.rs/tower-cookies/latest/tower_cookies/
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

	info!("{:<12} - {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}
