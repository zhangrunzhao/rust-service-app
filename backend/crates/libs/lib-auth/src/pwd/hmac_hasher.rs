use crate::pwd::{ContentToHash, Error, Result};
use hmac::{Hmac, Mac};
use lib_utils::b64::b64u_encode;
use sha2::Sha512;

pub fn hmac_sha512_hash(key: &[u8], to_hash: &ContentToHash) -> Result<String> {
    let ContentToHash { content, salt } = to_hash;

    // -- Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFail)?;

    // -- Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // -- Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();

    let result = b64u_encode(hmac_result.into_bytes());

    Ok(result)
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
    use rand::RngCore;
    use uuid::Uuid;

    #[test]
    fn test_hmac_sha512_hash_ok() -> Result<()> {
        // 初始化
        let mut fx_key = [0u8; 64];
        // 往 fx_key 里塞满随机值
        rand::thread_rng().fill_bytes(&mut fx_key);

        let fx_enc_content = ContentToHash {
            content: "hello world".to_string(),
            salt: Uuid::new_v4(),
        };

        let fx_res = hmac_sha512_hash(&fx_key, &fx_enc_content)?;

        // 执行
        let res = hmac_sha512_hash(&fx_key, &fx_enc_content)?;

        // 检查执行两次是否能得到相同的结果
        assert_eq!(res, fx_res);

        Ok(())
    }
}

// endregion: --- Tests
