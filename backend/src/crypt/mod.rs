// region:    --- Modules

mod error;
pub mod pwd;

pub use self::error::{Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha512;

// endregion: --- Modules

pub struct EncryptContent {
	pub content: String, // Clear content
	pub salt: String,    // Clear salt.
}

pub fn encrypt_into_b64u(
	key: &[u8],
	enc_content: &EncryptContent,
) -> Result<String> {
	let EncryptContent { content, salt } = enc_content;

	// 从给定的字节数组创建一个 HMAC-SHA-512 哈希
	let mut hmac_sha512 =
		Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

	// update 方法用于添加额外的数据到 HMAC 计算中
	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	// 返回一个 HMAC 对象中的哈希值，即消息的 HMAC-SHA512 哈希。
	let hmac_result = hmac_sha512.finalize();
	// 这个方法将哈希值转换为字节数组，可以将其使用作为消息认证的签名。
	let result_bytes = hmac_result.into_bytes();

	let result = base64_url::encode(&result_bytes);

	Ok(result)
}

// region:    --- Tests
#[cfg(test)]
mod tests {
	#![allow(unused)]
	use super::*;
	use anyhow::Result;
	use rand::RngCore;

	#[test]
	fn test_encrypt_into_b64u_ok() -> Result<()> {
		// 初始化
		let mut fx_key = [0u8, 64]; // 512 bits = 64 bytes
		rand::thread_rng().fill_bytes(&mut fx_key);

		let fx_enc_content = EncryptContent {
			content: "hello world".to_string(),
			salt: "some pepper".to_string(),
		};

		let fx_res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

		// 执行
		let res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

		// 检查执行两次是否能得到相同的结果
		assert_eq!(res, fx_res);

		Ok(())
	}
}
// endregion: --- Tests
