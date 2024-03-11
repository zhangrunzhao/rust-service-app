use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Login
    LoginFail,
    //  -- CtxExtError，常用于识别 auth 中的错误，并能够快速得提取出错误信息和错误码
    // CtxExt(web::mw_auth::CtxExtError),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
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
impl Error {
    // pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
    //     use web::Error::*;

    //     match self {
    //         // -- Login/Auth
    //         CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

    //         // -- Fallback
    //         _ => (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             ClientError::SERVICE_ERROR,
    //         ),
    //     }
    // }
}

// #[allow(non_camel_case_types)]
// pub enum ClientError {
//     // 账号密码错误等原因引起的登录失败
//     LOGIN_FAIL,
//     // 票据验证失败
//     NO_AUTH,
//     // 服务端位置错误
//     SERVICE_ERROR,
// }

// endregion: --- Client Error
