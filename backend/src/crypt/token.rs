use axum::body::HttpBody;

use crate::{
    config,
    utils::{b64u_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_utc},
};

use super::{encrypt_into_b64u, EncryptContent, Error, Result};
use std::{fmt::Display, str::FromStr};

// region:    --- Token Type
#[derive(Debug)]
pub struct Token {
    pub ident: String,     // 基本身份验证信息
    pub exp: String,       // 过期时间（Rfc3339 格式）
    pub sign_b64u: String, //签名后的 base64字符串
}

// 因为 token 是一个 String 类型，使用 FromStr 特征来给 token.parse 赋能
// 这样我们使用 token.parse 时可以快速地得到解构后的数据
impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split(".").collect();

        if splits.len() != 3 {
            return Err(Error::TokenInvalidFormat);
        }

        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64u_decode(ident_b64u).map_err(|_| Error::TokenCannotDecodeIdent)?,

            exp: b64u_decode(exp_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,

            sign_b64u: sign_b64u.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

// endregion: --- Token Type

// region:    --- Web Token Gen and Validation

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
    let config = &config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}

// endregion: --- Web Token Gen and Validation

// region:    --- (private) Web Token Gen and Validation

fn _generate_token(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<Token> {
    // 计算前面两个字段
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    // 将前面两项进行哈希加密, 得到 token 的第三个值
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: &str, key: &[u8]) -> Result<()> {
    // 验证签名
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

    // 验证签名是否被篡改
    if new_sign_b64u != origin_token.sign_b64u {
        return Err(Error::TokenSignatureNotMatching);
    }

    // 从 token 里拿出过期时间，验证是否过期
    let origin_exp = parse_utc(&origin_token.exp).map_err(|_| Error::TokenExpNotIso)?;

    let now = now_utc();

    if origin_exp < now {
        return Err(Error::TokenExpired);
    }

    Ok(())
}

fn _token_sign_into_b64u(ident: &str, exp: &str, salt: &str, key: &[u8]) -> Result<String> {
    // 生成 token 的前两项
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));

    let signature = encrypt_into_b64u(
        key,
        &EncryptContent {
            content,
            salt: salt.to_string(),
        },
    )?;

    Ok(signature)
}

// endregion: --- (private) Web Token Gen and Validation
