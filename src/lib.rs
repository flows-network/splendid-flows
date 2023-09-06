use flowsnet_platform_sdk::logger;
use serde_json::Value;
use std::collections::HashMap;
use webhook_flows::{create_endpoint, request_handler, send_response};

mod auth;

const SUCCESS_HTML: &[u8] = include_bytes!("success.html");

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handle(
    _headers: Vec<(String, String)>,
    _subpath: String,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    logger::init();

    let code = qry
        .get("code")
        .unwrap_or(&Value::Null)
        .as_str()
        .unwrap_or("");
    match auth::auth(code.to_string()).await {
        Ok(token) => {
            send_response(
                200,
                vec![(String::from("Content-Type"), String::from("text/html"))],
                SUCCESS_HTML.to_vec(),
            );
        }
        Err(e) => {
            send_response(
                500,
                vec![(String::from("Content-Type"), String::from("text/html"))],
                e.as_bytes().to_vec(),
            );
        }
    }
}
