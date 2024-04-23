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

#[derive(serde::Deserialize, Clone, FromRow, Fields, Debug)]
pub struct UserForCreate {
    pub username: String,
    pub pwd: String,
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
impl UserBy for UserForAuth {}
impl UserBy for UserForCreate {}

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

    pub async fn create<E>(ctx: &Ctx, mm: &ModelManager, user_c: UserForCreate) -> Result<i64>
    where
        E: UserBy,
    {
        let id = base::create::<Self, _>(ctx, mm, user_c.clone()).await?;

        // 给新增的账号进行密码加密
        Self::update_pwd(ctx, mm, id, &user_c.pwd).await?;

        Ok(id)
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

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use crate::_dev_utils;
    use anyhow::{Context, Result};
    use serial_test::serial;

    #[tokio::test]
    async fn test_first_by_username_ok_demo1() -> Result<()> {
        // 初始化
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        // 执行
        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .context("Should have user 'demo1'")?;

        // 检查
        assert_eq!(user.username, fx_username);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_user_ok_demo12() -> Result<()> {
        // 初始化
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo12";
        let fx_pwd = "welcome";

        // 执行
        // 先创建一个角色
        let _ = UserBmc::create::<UserForCreate>(
            &ctx,
            &mm,
            UserForCreate {
                username: fx_username.to_string(),
                pwd: fx_pwd.to_string(),
            },
        )
        .await?;

        // 创建完毕后再进行登录
        let user: UserForLogin = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .context("Should have user 'demo12'")?;

        // 注册完毕后在数据库中的 pwd 是已经被加密过的字段，我们校验的时候需要把明文加密一遍
        let pwd = pwd::encrypt_pwd(&EncryptContent {
            content: fx_pwd.to_string(),
            salt: user.pwd_salt.to_string(),
        })?;

        // 校验账号密码是否对得上
        assert_eq!(user.username, fx_username);
        assert_eq!(user.pwd, Some(pwd));

        Ok(())
    }
}

// endregion: --- Tests
