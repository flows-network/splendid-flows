use serde_json::Value;
use std::collections::HashMap;
use webhook_flows::{create_endpoint, request_handler, send_response};

mod auth;

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
    let code = qry
        .get("code")
        .unwrap_or(&Value::Null)
        .as_str()
        .unwrap_or("");
    auth::auth(code.to_string()).await;
    send_response(200, vec![], vec![]);
}
