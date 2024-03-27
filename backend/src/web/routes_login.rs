use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForLogin};
use crate::model::ModelManager;
use crate::web::{Error, Result, AUTH_TOKEN};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/login", post(api_login_handler))
		.with_state(mm) // 往提取器中塞进一个 modalManager
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

	cookies.add(Cookie::new(AUTH_TOKEN, "runzhao.niu.bi"));

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
