use crate::{
    crypt::{pwd, EncryptContent},
    ctx::Ctx,
    model::{Error, Result},
};
use serde::{Deserialize, Serialize};
use sqlb::{Field, Fields, HasFields};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

use super::{
    base::{self, DbBmc},
    ModelManager,
};

// region:    --- User Types
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(serde::Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

#[derive(Fields)]
pub struct UserForInsert {
    username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,

    // pwd 和 token 信息
    pub pwd: Option<String>,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,

    // -- token info
    pub token_salt: Uuid,
}

pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}

// endregion: --- User Types

pub struct UserBmc {}

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, E>(ctx, mm, id).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.db();

        let user = sqlb::select()
            .table(Self::TABLE)
            .and_where("username", "=", username)
            .fetch_optional::<_, E>(db)
            .await?;

        Ok(user)
    }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: i64, pwd_clear: &str) -> Result<()> {
        let db = mm.db();

        let user: UserForLogin = Self::get(ctx, mm, id).await?;

        let pwd = pwd::encrypt_pwd(&EncryptContent {
            content: pwd_clear.to_string(),
            salt: user.pwd_salt.to_string(),
        })?;

        sqlb::update()
            .table(Self::TABLE)
            .and_where("id", "=", id)
            .data(vec![("pwd", pwd.to_string()).into()])
            .exec(db)
            .await?;

        Ok(())
    }
}
