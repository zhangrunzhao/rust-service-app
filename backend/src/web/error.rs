use crate::{crypt, model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	// -- RPC
	RpcMethodUnknown(String),
	RpcMissingParams { rpc_method: String },
	RpcFailJsonParams { rpc_method: String },

	// -- Login
	LoginFailUsernameNotFound,
	LoginFailUserHasNoPwd { user_id: i64 },
	LoginFailPwdNotMatching { user_id: i64 },

	// -- Modules
	Model(model::Error),

	// -- CtxExtError，常用于识别 auth 中的错误，并能够快速得提取出错误信息和错误码
	CtxExt(web::mw_auth::CtxExtError),

	// -- Crypt
	Crypt(crypt::Error),

	// External Modules
	SerdeJson(String),
}

// region:    --- Froms
impl From<model::Error> for Error {
	fn from(value: model::Error) -> Self {
		Self::Model(value)
	}
}

impl From<crypt::Error> for Error {
	fn from(value: crypt::Error) -> Self {
		Self::Crypt(value)
	}
}

impl From<serde_json::Error> for Error {
	fn from(value: serde_json::Error) -> Self {
		Self::SerdeJson(value.to_string())
	}
}
// endregion: --- Froms

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("->> {:<12} - model::Error {self:?}", "INTO_RES");

		// 生成一个默认的 Axum response
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// 把错误信息塞入到 Axum response 中
		response.extensions_mut().insert(self);

		response
	}
}
// endregion: --- Axum IntoResponse

// region:    --- Error 默认的错误信息输出
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter<'_>,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error 默认的错误信息输出

// region:    --- Client Error
impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		use web::Error::*;

		match self {
			// -- Login
			LoginFailUsernameNotFound
			| LoginFailUserHasNoPwd { .. }
			| LoginFailPwdNotMatching { .. } => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			}

			// -- Model
			Model(model::Error::EntityNotFound { entity, id }) => (
				StatusCode::BAD_REQUEST,
				ClientError::ENTITY_NOT_FOUND { entity, id: *id },
			),

			// -- Auth
			CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			// -- Fallback
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
	// 账号密码错误等原因引起的登录失败
	LOGIN_FAIL,
	// 票据验证失败
	NO_AUTH,
	ENTITY_NOT_FOUND { entity: &'static str, id: i64 },

	// 服务端位置错误
	SERVICE_ERROR,
}

// endregion: --- Client Error
