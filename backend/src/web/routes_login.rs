use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForCreate, UserForLogin};
use crate::model::ModelManager;
use crate::pwd::{self, ContentToHash};
use crate::web::{self, remove_token_cookie, Error, Result, AUTH_TOKEN};
use axum::extract::State;
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::{debug, info};
use ts_rs::TS;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/register", post(api_register_handler))
        .route("/api/logoff", post(api_logoff_handler))
        .with_state(mm)
}

async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginReq>,
) -> Result<Json<Value>> {
    info!("->> {:<12} - api_login_handler", "HANDLER");

    let LoginReq {
        username,
        pwd: pwd_clear,
    } = payload;

    let root_ctx = Ctx::root_ctx();

    // 获取 User
    let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;

    let user_id = user.id;

    // 验证密码
    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };

    pwd::validate_pwd(
        &ContentToHash {
            salt: user.pwd_salt,
            content: pwd_clear.clone(),
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    // 设置 web token
    // 此处的 token_salt 是在建表时添加的 uuid
    web::set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

    let body = Json(json!({
      "data": {
        "user_id": user.id
      }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "user/")]
struct LoginReq {
    username: String,
    pwd: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "user/")]
struct LoginResp {
    user_id: i32,
}

// region:    --- Register
async fn api_register_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<RegisterReq>,
) -> Result<Json<Value>> {
    info!("->> {:<12} - api_register_handler", "HANDLER");

    let RegisterReq { username, pwd } = payload;
    let root_ctx = Ctx::root_ctx();

    let user_id: i64 = UserBmc::create::<UserForCreate>(
        &root_ctx,
        &mm,
        UserForCreate {
            username,
            pwd: pwd.clone(),
        },
    )
    .await?;

    UserBmc::update_pwd(&root_ctx, &mm, user_id, &pwd).await?;

    let body = Json(json!({
      "data": {
        "user_id": user_id
      }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "user/")]
struct RegisterReq {
    username: String,
    pwd: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "user/")]
struct RegisterResp {
    user_id: i32,
}

// endregion: --- Register

// region:    --- Logoff
async fn api_logoff_handler(
    cookies: Cookies,
    Json(payload): Json<LogoffReq>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logoff_handler", "HANDLER");
    let should_logoff = payload.logoff;

    if should_logoff {
        remove_token_cookie(&cookies)?;
    }

    // Create the success body.
    let body = Json(json!({
      "data": {
        "logged_off": should_logoff
      }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "user/")]
struct LogoffReq {
    logoff: bool,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "user/")]
struct LogoffResp {
    logged_off: bool,
}

// endregion: --- Logoff

// region:    --- Test
mod test {
    #![allow(unused)]
    use super::*;
    use crate::_dev_utils;
    use axum::{
        body::Body,
        http::{self, Request},
    };
    use serde::Deserialize;
    use serde_json::json;
    use tower::ServiceExt;
    use tower_cookies::CookieManagerLayer;

    #[derive(Debug, Deserialize)]
    struct ResponseBody<T> {
        data: T,
    }

    #[tokio::test]
    async fn test_register_api() -> Result<()> {
        // 初始化
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo12";
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
                          "username": "demo3",
                          "pwd": "welcome"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        let register_body = hyper::body::to_bytes(register_response.into_body())
            .await
            .unwrap();
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

// endregion: --- Test
