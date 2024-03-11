// region:    --- Modules
mod error;
pub mod routes_login;

pub use self::error::{Error, Result};

// endregion: --- Modules

pub const AUTO_TOKEN: &str = "auth-token";
