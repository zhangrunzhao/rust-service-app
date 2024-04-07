use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForLogin};
use crate::model::ModelManager;
use crate::web::{self, remove_token_cookie, Error, Result, AUTH_TOKEN};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/logoff", post(api_logoff_handler))
        .with_state(mm)
}

async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login_handler", "HANDLER");

    let LoginPayload {
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
        &EncryptContent {
            salt: user.pwd_salt.to_string(),
            content: pwd_clear.clone(),
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    // 设置 web token
    // 此处的 token_salt 是在建表时添加的 uuid
    web::set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

    let body = Json(json!({
      "result": {
        "success": true
      }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

// region:    --- Logoff
async fn api_logoff_handler(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logoff_handler", "HANDLER");
    let should_logoff = payload.logoff;

    if should_logoff {
        remove_token_cookie(&cookies)?;
    }

    // Create the success body.
    let body = Json(json!({
      "result": {
        "logged off": should_logoff
      }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
    logoff: bool,
}

// endregion: --- Logoff
