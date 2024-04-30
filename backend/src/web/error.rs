use std::borrow::Cow;

use crate::{model, token, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

// #[serde(xxx)] 会把 json 解析后的结果 key value 提前配一下
// 像下面的 case，Error 转成 json 会得到如下结果：
// {"type": "LoginFailUsernameNotFound"}
// {"type": "LoginFailUserHasNoPwd", "value": { "user_id": xxx }}
#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd { user_id: i64 },
    LoginFailPwdNotMatching { user_id: i64 },

    // -- Register
    RegisterFail,

    // -- Modules
    Model(model::Error),

    // -- Token
    Token(token::Error),

    //  -- CtxExtError，常用于识别 auth 中的错误，并能够快速得提取出错误信息和错误码
    CtxExt(web::mw_auth::CtxExtError),
}

// region:    --- Froms

impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}

impl From<token::Error> for Error {
    fn from(value: token::Error) -> Self {
        Self::Token(value)
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error 默认的错误信息输出

// region:    --- Client Error
#[allow(non_snake_case)]
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::*;

        match self {
            // -- Login/Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- 密码错误
            LoginFailUserHasNoPwd { user_id } => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientError::LOGIN_FAIL)
            }

            // -- Model Error
            Model(error) => {
                match error {
                    model::Error::Sqlx(sqlx_err) => {
                        if let Some(db_err) = sqlx_err.as_database_error() {
                            if db_err.code().take() == Some(Cow::Owned(String::from("23505"))) {
                                return (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    ClientError::USERNAME_ALREADY_EXIST,
                                );
                            }
                        };

                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            ClientError::SERVICE_ERROR,
                        );
                    }

                    _ => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            ClientError::SERVICE_ERROR,
                        );
                    }
                };
            }

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

    // 用户名已存在
    USERNAME_ALREADY_EXIST,
}

// endregion: --- Client Error
