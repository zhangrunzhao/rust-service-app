use crate::{Error, Result};
use std::{env, sync::OnceLock};

pub fn config() -> &'static Config {
    // 单线程锁，它只允许一个线程访问一个共享变量，从而防止竞争
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    // -- Web
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            // -- Web
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    // env::var 的值部分来自于 .cargo 文件夹
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}
