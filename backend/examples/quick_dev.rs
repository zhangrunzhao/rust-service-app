#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ResponseBody<T> {
    result: T,
}

#[derive(Debug, Deserialize)]
struct ResponseResult {
    success: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/").await?.print().await?;

    let req_register = hc.do_post(
        "/api/register",
        json!({
          "username":"demo131",
          "pwd": "welcome"
        }),
    );

    req_register.await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
          "username": "demo131",
          "pwd": "welcome"
        }),
    );

    req_login.await?.print().await?;
    hc.do_get("/hello").await?.print().await?;

    // let req_logoff = hc.do_post(
    //     "/api/logoff",
    //     json!({
    //       "logoff": true
    //     }),
    // );
    // req_logoff.await?.print().await?;

    // hc.do_get("/hello").await?.print().await?;
    Ok(())
}
