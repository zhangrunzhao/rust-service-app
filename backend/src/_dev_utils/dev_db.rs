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
	info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

	// 这里用代码块只是想要弃用掉 root_db
	{
		let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
		pexec(&root_db, SQL_RECREATE_DB).await?; // 重建数据库
	}

	// -- 先读出文件夹下所有文件路径
	let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
		.filter_map(|entry| entry.ok().map(|e| e.path()))
		.collect();

	paths.sort();

	let app_db = new_db_pool(PG_DEV_APP_URL).await?;
	for path in paths {
		if let Some(path) = path.to_str() {
			let path = path.replace('\\', "/");

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
	info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

	let content = fs::read_to_string(file)?;

	let sqls: Vec<&str> = content.split(";").collect();

	for sql in sqls {
		sqlx::query(sql).execute(db).await?;
	}

	Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(1)
		.acquire_timeout(Duration::from_millis(500))
		.connect(db_con_url)
		.await
}
