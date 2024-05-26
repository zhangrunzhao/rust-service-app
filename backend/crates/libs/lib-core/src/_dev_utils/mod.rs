// region:    --- Modules

use tokio::sync::OnceCell;
use tracing::info;

use crate::model::ModelManager;

mod dev_db;

// endregion: --- Modules

// 初始化本地开发环境
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

// 初始化测试环境，返回一个用于测试的 ModelManager
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
