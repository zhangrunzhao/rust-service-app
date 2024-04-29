// region:    --- Modules
mod error;
pub mod token;

use hmac::{Hmac, Mac};
use sha2::Sha512;

pub use self::error::{Error, Result};

// endregion: --- Modules

pub struct EncryptContent {
    pub content: String,
    pub salt: String,
}

// 该方法将通过传入的秘钥，使用 hmac 中 sha512 算法将需要加密的内容加密并返回 base64 字符串
pub fn encrypt_into_b64u(key: &[u8], enc_content: &EncryptContent) -> Result<String> {
    let EncryptContent { content, salt } = enc_content;

    // 创建基于秘密密钥的 HMAC-SHA-512 对象
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

    // update 方法用于添加额外的数据到 HMAC 计算中
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // 返回一个 HMAC 对象中的哈希值。
    let hmac_result = hmac_sha512.finalize();
    // 这个方法将哈希值转化为字节数组，可以将其使用作为消息认证的签名。
    let result_bytes = hmac_result.into_bytes();

    // 将字符串数组转成 base64 编码
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
        let mut fx_key = [0u8; 64];
        // 往 fx_key 里塞满随机值
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
