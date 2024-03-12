// region:    --- Modules
mod error;
pub mod routes_login;
pub mod routes_static;

pub use self::error::{Error, Result};

// endregion: --- Modules

pub const AUTO_TOKEN: &str = "auth-token";
