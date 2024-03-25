//! Model Layer
//!
//! Design:
//!
//! - 对应用程序的数据类型结构和访问做模型层规范化。
//! - 所有的应用程序代码数据访问都必须经过 Model 层。
//! - ModelManager 保存着 ModelController 访问数据所需的内部状态/资源。
//! - ModelController (TaskBmc, ProjectBmc) 是一个提供实现了 CRUD 和其他数据访问方法的实体。
//! (Bmc 是 Backend Model Controller 的缩写)
//! - 在像 Axum，Tauri 这样的框架中，"ModelManager" 通常用作 App State。
//! - ModelManager 被设计为作为参数传递者给所有的 ModelController 函数传递参数
//!

// region:    --- Modules

mod base;
mod error;
mod store;
pub mod task;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;

		Ok(ModelManager { db })
	}

	// 返回 sqlx 的数据库连接池引用 （仅在 model 文件夹下使用）
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
