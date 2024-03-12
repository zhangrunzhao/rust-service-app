use crate::log::log_request;
use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

use crate::{ctx::Ctx, web};

pub async fn mw_response_map(
    ctx: Option<Ctx>,   // 满足 FromRequestParts 特征的结构体，被放入到提取器中了
    uri: Uri,           // 被提取器提取出来的字段
    req_method: Method, // 被提取器提取出来的字段
    res: Response,
) -> Response {
    println!("->> {:<12} - mw_response_map ", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // 获取 web 错误 和 错误码
    let web_error = res.extensions().get::<web::Error>();
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    // 生成错误信息
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("->> CLIENT ERROR BODY:\n{client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // 解开元祖，获取到客户端错误信息
    let client_error = client_status_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, ctx, web_error, client_error).await;

    debug!("\n");

    error_response.unwrap_or(res)
}
