use flowsnet_platform_sdk::logger;
use serde_json::Value;
use std::collections::HashMap;
use webhook_flows::{create_endpoint, request_handler, send_response};

mod auth;

use auth::NotionAuth;

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

    let code = qry.get("code").unwrap_or(&Value::Null).as_str();
    let user = qry.get("state").unwrap_or(&Value::Null).as_str();
    match (code, user) {
        (Some(code), Some(user)) if code != "" && user != "" => match auth::auth(code).await {
            Ok(author) => {
                match store_flows::get(user) {
                    Some(authors) => {
                        let authors: Vec<NotionAuth> = serde_json::from_value(authors).unwrap();
                        let new_authors = authors
                            .into_iter()
                            .filter(|t| t.bot_id == author.bot_id)
                            .collect::<Vec<NotionAuth>>()
                            .push(author);
                        store_flows::set(user, serde_json::to_value(new_authors).unwrap(), None);
                    }
                    None => {
                        store_flows::set(user, serde_json::json!([author]), None);
                    }
                }

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
        },
        _ => {
            send_response(
                400,
                vec![(String::from("Content-Type"), String::from("text/html"))],
                "Invalid params".as_bytes().to_vec(),
            );
        }
    }
}
