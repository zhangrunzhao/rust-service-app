use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use sqlb::HasFields;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

use super::task::Task;

pub trait DbBmc {
	const TABLE: &'static str;
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();
	let fields = data.not_none_fields();
	let (id,) = sqlb::insert()
		.table(MC::TABLE)
		.data(fields)
		.returning(&["id"])
		.fetch_one::<_, (i64,)>(db)
		.await?;

	Ok(id)
}

// MC 代表 ModalController
// E 代表 Entity
pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
	E: HasFields,
{
	let db = mm.db();

	// 获取一个实体
	let entity: E = sqlb::select()
		.table(MC::TABLE)
		.columns(E::field_names())
		.and_where("id", "=", id)
		// 向输入的 db 中执行 sql 语句，返回一个 option，仅能检索单条数据
		.fetch_optional(db)
		.await?
		.ok_or(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})?;

	Ok(entity)
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
	E: HasFields,
{
	let db = mm.db();

	// 获取一个实体
	let entities: Vec<E> = sqlb::select()
		.table(MC::TABLE)
		.columns(E::field_names())
		.order_by("id") // 带感叹代表降序，不带则升序
		// 向输入的 db 中执行 sql 语句，返回一个 option，仅能检索单条数据
		.fetch_all(db)
		.await?;

	Ok(entities)
}

pub async fn update<MC, E>(
	_ctx: &Ctx,
	mm: &ModelManager,
	id: i64,
	data: E,
) -> Result<()>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();
	let fields = data.not_none_fields();
	let count = sqlb::update()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.data(fields)
		.exec(db)
		.await?;

	if count == 0 {
		Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})
	} else {
		Ok(())
	}
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
	MC: DbBmc,
{
	let db = mm.db();

	let count = sqlb::delete()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.exec(db)
		.await?;

	if count == 0 {
		Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})
	} else {
		Ok(())
	}
}
