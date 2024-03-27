use super::{Error, Result};
use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent};

// 使用默认方案加密密码
pub fn encrypt_pwd(enc_content: &EncryptContent) -> Result<String> {
	let key = &config().PWD_KEY;

	let encrypted = encrypt_into_b64u(key, enc_content)?;

	Ok(format!("#01#{encrypted}"))
}

// 验证加密的内容是否匹配
pub fn validate_pwd(enc_content: &EncryptContent, pwd_ref: &str) -> Result<()> {
	let pwd = encrypt_pwd(enc_content)?;

	if pwd == pwd_ref {
		Ok(())
	} else {
		Err(Error::PwdNotMatching)
	}
}
