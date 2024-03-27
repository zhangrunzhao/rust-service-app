use crate::{Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

pub fn config() -> &'static Config {
	// 单线程锁，它只允许一个线程访问一个共享变量，从而防止竞争
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct Config {
	// -- Crypt
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,

	// -- Db
	pub DB_URL: String,

	// -- Web
	pub WEB_FOLDER: String,
}

impl Config {
	fn load_from_env() -> Result<Config> {
		Ok(Config {
			// 安全
			PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,

			TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
			TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,

			// -- Db
			DB_URL: get_env("SERVICE_DB_URL")?,

			// -- Web
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}

fn get_env(name: &'static str) -> Result<String> {
	// env::var 的值部分来自于 .cargo 文件夹
	env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
	let val = get_env(name)?;
	val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
	base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}
