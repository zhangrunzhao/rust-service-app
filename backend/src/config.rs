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
    // Web 的静态文件目录路径
    pub WEB_FOLDER: String,
    // Web 的静态 html 文件
    pub WEB_FILE: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
            WEB_FILE: get_env("SERVICE_WEB_FILE")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissEnv(name))
}
