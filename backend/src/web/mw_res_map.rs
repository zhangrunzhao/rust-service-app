use crate::{log::log_request, web::rpc::RpcInfo};
use axum::{
	http::{Method, Uri},
	response::{IntoResponse, Response},
	Json,
};
use serde_json::{json, to_value};
use tracing::debug;
use uuid::Uuid;

use crate::{ctx::Ctx, web};

pub async fn mw_response_map(
	ctx: Option<Ctx>, // 满足 FromRequestParts 特征的结构体，被放入到提取器中了
	uri: Uri,         // 被提取器提取出来的字段
	req_method: Method, // 被提取器提取出来的字段
	res: Response,
) -> Response {
	println!("->> {:<12} - mw_response_map ", "RES_MAPPER");
	let uuid = Uuid::new_v4();

	let rpc_info = res.extensions().get::<RpcInfo>();

	// 获取 web 错误 和 错误码
	let web_error = res.extensions().get::<web::Error>();
	// 从 client_status_and_error 中获取错误信息
	let client_status_error = web_error.map(|se| se.client_status_and_error());

	// 生成错误信息
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				// 解析元组
				let client_error = to_value(client_error).ok();
				let message = client_error.as_ref().and_then(|v| v.get("message"));
				let detail = client_error.as_ref().and_then(|v| v.get("detail"));

				let client_error_body = json!({
				"id": rpc_info.as_ref().map(|rpc| rpc.id.clone()),
					  "error": {
								"message": message, // Variant name
								"data": {
							"req_uuid": uuid.to_string(),
							"detail": detail
						  },
							}
						});

				println!("->> CLIENT ERROR BODY:\n{client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});

	// 解开元祖，获取到客户端错误信息
	let client_error = client_status_error.unzip().1;
	let _ = log_request(
		uuid,
		req_method,
		uri,
		rpc_info,
		ctx,
		web_error,
		client_error,
	)
	.await;

	debug!("\n");

	error_response.unwrap_or(res)
}
