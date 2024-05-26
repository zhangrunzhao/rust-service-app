use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

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

// sql files
const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("->> {:<12} ", "FOR-DEV-ONLY");

    let current_dir = std::env::current_dir().unwrap();
    let v: Vec<_> = current_dir.components().collect();
    let path_comp = v.get(v.len().wrapping_sub(3));
    let base_dir = if Some(true) == path_comp.map(|c| c.as_os_str() == "crates") {
        v[..v.len() - 3].iter().collect::<PathBuf>()
    } else {
        current_dir.clone()
    };
    let sql_dir = base_dir.join(SQL_DIR);

    {
        // 拿到连接池
        let sql_recreate_db_file = sql_dir.join(SQL_RECREATE_DB_FILE_NAME);
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, &sql_recreate_db_file).await?;
    }

    // 读出文件夹下所有文件路径
    let mut paths: Vec<PathBuf> = fs::read_dir(sql_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    // 根据文件名进行排序
    paths.sort();

    // 通过 app_db 的 url 获取数据库连接池
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;

    for path in paths {
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".sql") && !path_str.ends_with(SQL_RECREATE_DB_FILE_NAME) {
            pexec(&app_db, &path).await?;
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

async fn pexec(db: &Db, file: &Path) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file:?}", "FOR-DEV-ONLY");

    // -- Read the file.
    let content = fs::read_to_string(file)?;

    // 读文件，将 sql 文件中的 sql 语句读出来后切个成多个 sql 语句
    let sqls: Vec<&str> = content.split(';').collect();

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
