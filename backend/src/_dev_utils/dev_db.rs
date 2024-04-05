use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

use crate::{
    ctx::Ctx,
    model::{
        user::{User, UserBmc},
        ModelManager,
    },
};

type Db = Pool<Postgres>;

// 连接数据库
const PG_DEV_POSTGRES_URL: &str =
    "postgres://postgres:321chenjixink@localhost:5432/postgres?sslmode=disable";

const PG_DEV_APP_URL: &str =
    "postgres://postgres:321chenjixink@localhost:5432/app_db?sslmode=disable";

// sql 文件
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("->> {:<12} ", "FOR-DEV-ONLY");

    {
        // 拿到连接池
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;

        // 使用 postgres 默认用户将 app_db 数据库清除干净
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    // 读出文件夹下所有文件路径
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    // 根据文件名进行排序
    paths.sort();

    // 通过 app_db 的 url 获取数据库连接池
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;
    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/");
            // 往 app_db 中执行相关的 sql，比如建表跟插入 mock 数据
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?;
            }
        }
    }

    // 初始化 model layer
    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    let demo1_user: User = UserBmc::first_by_username(&ctx, &mm, "demo1")
        .await?
        .unwrap();

    UserBmc::update_pwd(&ctx, &mm, demo1_user.id, DEMO_PWD).await?;

    info!("->> {:<12} - init_dev_db - set demo1 pwd", "FOR-DEV-ONLY");

    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("->> {:<12} - pexec: {file}", "FOR-DEV-ONLY");

    let content = fs::read_to_string(file)?;

    // 读文件，将 sql 文件中的 sql 语句读出来后切个成多个 sql 语句
    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        // 往目标数据库执行相关 sql 语句
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

// 创建一个数据库的连接池
async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}
