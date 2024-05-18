use serde::Serialize;

use crate::token;

use super::store;
use crate::pwd;
use derive_more::From;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    #[from]
    EntityNotFound { entity: &'static str, id: i64 },
    // Modules
    #[from]
    Store(store::Error),

    // Token
    #[from]
    Token(token::Error),

    // Pwd
    #[from]
    Pwd(pwd::Error),

    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
}

// region:    --- Froms

// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
