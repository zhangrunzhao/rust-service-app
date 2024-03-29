// region:    --- Modules

mod task_rpc;

use axum::{
	extract::State,
	response::{IntoResponse, Response},
	routing::post,
	Json, Router,
};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;

use crate::{
	ctx::Ctx,
	model::ModelManager,
	web::{
		rpc::task_rpc::{create_task, delete_task, list_tasks, update_task},
		Error, Result,
	},
};

// endregion: --- Modules

// region:    --- RPC Types

#[derive(Deserialize)]
struct RpcRequest {
	id: Option<Value>,
	method: String,
	params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
	id: i64,
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
	id: i64,
}

// endregion: --- RPC Types

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/rpc", post(rpc_handler))
		.with_state(mm)
}

async fn rpc_handler(
	State(mm): State<ModelManager>,
	ctx: Ctx,
	Json(rpc_req): Json<RpcRequest>,
) -> Response {
	// 创建 rpc info 用于响应
	let rpc_info = RpcInfo {
		id: rpc_req.id.clone(),
		method: rpc_req.method.clone(),
	};

	let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();

	res.extensions_mut().insert(rpc_info);

	res
}

#[derive(Debug)]
pub struct RpcInfo {
	pub id: Option<Value>,
	pub method: String,
}

macro_rules! exec_rpc_fn {
	// With Params
	($rpc_fn: expr, $ctx: expr, $mm: expr, $rpc_params: expr) => {{
		let rpc_fn_name = stringify!($rpc_fn);

		let params = $rpc_params.ok_or(Error::RpcMissingParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;

		let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;

		$rpc_fn($ctx, $mm, params).await.map(to_value)??
	}};

	// Without Params
	($rpc_fn: expr, $ctx: expr, $mm: expr) => {
		$rpc_fn($ctx, $mm).await.map(to_value)??
	};
}

async fn _rpc_handler(
	ctx: Ctx,
	mm: ModelManager,
	rpc_req: RpcRequest,
) -> Result<Json<Value>> {
	let RpcRequest {
		id: rpc_id,
		method: rpc_method,
		params: rpc_params,
	} = rpc_req;

	debug!("{:<12} - _rpc_handle - method: {rpc_method}", "HANDLER");

	let result_json: Value = match rpc_method.as_str() {
		//  -- Task RPC methods
		"create_task" => exec_rpc_fn!(create_task, ctx, mm, rpc_params),
		"list_tasks" => exec_rpc_fn!(list_tasks, ctx, mm),
		"update_task" => exec_rpc_fn!(update_task, ctx, mm, rpc_params),
		"delete_task" => exec_rpc_fn!(delete_task, ctx, mm, rpc_params),

		_ => return Err(Error::RpcMethodUnknown(rpc_method)),
	};

	let body_response = json!({
	  "id": rpc_id,
	  "result": result_json
	});

	Ok(Json(body_response))
}
