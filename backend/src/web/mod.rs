// region:    --- Modules
mod error;
pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_login;
pub mod routes_static;

use tower_cookies::{Cookie, Cookies};

use crate::crypt::token::generate_web_token;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

// endregion: --- Modules

pub const AUTH_TOKEN: &str = "auth-token";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
	let token = generate_web_token(user, salt)?;

	let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());

	cookie.set_http_only(true);
	cookie.set_path("/");

	cookies.add(cookie);

	Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
	let mut cookie = Cookie::named(AUTH_TOKEN);
	cookie.set_path("/");

	cookies.remove(cookie);

	Ok(())
}
