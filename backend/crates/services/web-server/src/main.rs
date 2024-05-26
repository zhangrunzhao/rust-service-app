#![allow(unused)]
// region:    --- Modules

mod config;
mod error;
mod log;
mod web;

pub use self::error::{Error, Result};
use config::web_config;

use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use web::mw_res_map::mw_response_map;

use crate::web::{routes_login, routes_static};
use lib_core::_dev_utils;
use lib_core::model::ModelManager;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

use axum::{middleware, response::Html, routing::get, Router};

// endregion: --- Modules

#[tokio::main] // 它会使 main 异步执行
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    _dev_utils::init_dev().await;

    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    let routes_hello = Router::new()
        .route("/hello", get(|| async { Html("Hello World") }))
        .route_layer(middleware::from_fn(mw_ctx_require));

    // 洋葱模型，写在后面的函数越早执行
    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .merge(routes_hello)
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

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use crate::{_dev_utils, web::routes_login::routes};
    use anyhow::{Context, Result};
    use axum::{
        body::Body,
        http::{self, Request},
        Router,
    };
    use lib_auth::pwd::{self, ContentToHash};
    use lib_core::{
        ctx::Ctx,
        model::user::{User, UserBmc, UserForCreate, UserForLogin},
    };
    use serde::Deserialize;
    use serde_json::json;
    use serial_test::serial;
    use tower::ServiceExt;
    use tower_cookies::CookieManagerLayer;
    use ts_rs::TS;

    #[derive(Debug, Deserialize)]
    struct ResponseBody<T> {
        data: T,
    }

    #[derive(Debug, Deserialize, TS)]
    #[ts(export, export_to = "user/")]
    struct RegisterResp {
        user_id: i32,
    }

    #[derive(Debug, Deserialize, TS)]
    #[ts(export, export_to = "user/")]
    struct LoginResp {
        user_id: i32,
    }

    #[tokio::test]
    async fn test_register_api() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        test_register_interface(mm.clone()).await;
        test_create_user_ok_demo12(mm.clone()).await;
        test_first_by_username_ok_demo1(mm.clone()).await;

        Ok(())
    }

    async fn test_first_by_username_ok_demo1(mm: ModelManager) -> Result<()> {
        // 初始化
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        // 执行
        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .context("Should have user 'demo1'")?;

        // 检查
        assert_eq!(user.username, fx_username);

        Ok(())
    }

    async fn test_create_user_ok_demo12(mm: ModelManager) -> Result<()> {
        // 初始化
        let ctx = Ctx::root_ctx();
        let fx_username = "demo12";
        let fx_pwd = "welcome";

        // 执行
        // 先创建一个角色
        let _ = UserBmc::create::<UserForCreate>(
            &ctx,
            &mm,
            UserForCreate {
                username: fx_username.to_string(),
                pwd: fx_pwd.to_string(),
            },
        )
        .await?;

        // 创建完毕后再进行登录
        let user: UserForLogin = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .context("Should have user 'demo12'")?;

        // 注册完毕后在数据库中的 pwd 是已经被加密过的字段，我们校验的时候需要把明文加密一遍
        let pwd = pwd::hash_pwd(&ContentToHash {
            content: fx_pwd.to_string(),
            salt: user.pwd_salt,
        })?;

        // 校验账号密码是否对得上
        assert_eq!(user.username, fx_username);
        assert_eq!(user.pwd, Some(pwd));

        Ok(())
    }

    async fn test_register_interface(mm: ModelManager) -> Result<()> {
        let ctx = Ctx::root_ctx();
        let fx_username = "demo3";
        let fx_pwd = "welcome";

        let route = Router::new()
            .merge(routes(mm))
            .layer(CookieManagerLayer::new());

        // 执行
        // 先注册账号
        let register_response = route
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/register")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(&json!({
                          "username": fx_username.to_string(),
                          "pwd": fx_pwd.to_string()
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        println!("register_response --------------> {:?}", register_response);

        let register_body = hyper::body::to_bytes(register_response.into_body())
            .await
            .unwrap();

        println!("register_body --------------> {:?}", register_body);

        let register_body: ResponseBody<RegisterResp> =
            serde_json::from_slice(&register_body).unwrap();

        // 登录新创建的账号
        let login_response = route
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/login")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(&json!({
                          "username": "demo3",
                          "pwd": "welcome"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        let login_body = hyper::body::to_bytes(login_response.into_body())
            .await
            .unwrap();
        let login_body: ResponseBody<LoginResp> = serde_json::from_slice(&login_body).unwrap();

        // 检查注册和登录是否成功
        assert_eq!(register_body.data.user_id, login_body.data.user_id);

        Ok(())
    }
}

// endregion: --- Tests
