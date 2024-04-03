// region:    --- Modules

mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

use crate::{
	ctx::Ctx,
	model::{
		self,
		task::{Task, TaskBmc, TaskForCreate},
		ModelManager,
	},
};

// endregion: --- Modules

// 初始化本地环境
pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();

	INIT.get_or_init(|| async {
		info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");
		dev_db::init_dev_db().await.unwrap();
	})
	.await;
}

// 初始化测试环境
pub async fn init_test() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			init_dev().await;
			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}

pub async fn seed_tasks(
	ctx: &Ctx,
	mm: &ModelManager,
	titles: &[&str],
) -> model::Result<Vec<Task>> {
	let mut tasks: Vec<Task> = Vec::new();

	// 按照传入的 titles 在数据库中创建记录
	for title in titles {
		let id = TaskBmc::create(
			ctx,
			mm,
			TaskForCreate {
				title: title.to_string(),
			},
		)
		.await?;

		// 创建完毕后再从数据库中取出来
		let task = TaskBmc::get(ctx, mm, id).await?;
		tasks.push(task);
	}

	Ok(tasks)
}
