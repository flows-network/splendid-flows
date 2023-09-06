use base64::prelude::BASE64_STANDARD;
use base64::write::EncoderWriter;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use serde::Deserialize;
use std::io::Write;

#[derive(Deserialize)]
pub struct NotionAuth {
    pub access_token: String,
    pub bot_id: String,
    pub workspace_id: String,
    pub workspace_name: Option<String>,
}

fn basic_auth<U>(username: U, password: U) -> String
where
    U: std::fmt::Display,
{
    let mut buf = b"Basic ".to_vec();
    {
        let mut encoder = EncoderWriter::new(&mut buf, &BASE64_STANDARD);
        let _ = write!(encoder, "{}:{}", username, password);
    }

    String::from_utf8_lossy(&buf).into_owned()
}

pub(crate) async fn auth(code: String) -> Result<NotionAuth, String> {
    let client_id = std::env::var("NOTION_CLIENT_ID").unwrap();
    let client_secret = std::env::var("NOTION_CLIENT_SECRET").unwrap();
    let redirect_uri = std::env::var("NOTION_REDIRECT_URI").unwrap();

    let basic = basic_auth(client_id, client_secret);

    let addr = Uri::try_from("https://api.notion.com/v1/oauth/token").unwrap();
    let mut writer = Vec::new();
    let body = serde_json::to_vec(&serde_json::json!({
        "grant_type": "authorization_code",
        "code": code,
        "redirect_uri": redirect_uri.as_str(),
    }))
    .unwrap();
    match Request::new(&addr)
        .method(Method::POST)
        .header("Connection", "Close")
        .header("Authorization", basic.as_str())
        .header("Content-Length", &body.len())
        .header("Content-Type", "application/json")
        .body(&body)
        .send(&mut writer)
    {
        Ok(response) => match response.status_code().is_success() {
            true => {
                let r = String::from_utf8_lossy(&writer).into_owned();
                log::debug!("{}", r);
                serde_json::from_slice(&writer).map_err(|e| e.to_string())
            }
            false => Err(String::from_utf8_lossy(&writer).into_owned()),
        },
        Err(e) => Err(e.to_string()),
    }
}
