// region:    --- Modules
mod error;
use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub use self::error::{Error, Result};

use crate::config::core_config;

// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&core_config().DB_URL)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
