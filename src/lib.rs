use flowsnet_platform_sdk::logger;
use notion_flows::notion::{
    ids::{DatabaseId, PropertyId},
    models::{
        properties::PropertyValue,
        text::{RichText, RichTextCommon, Text},
        PageCreateRequest, Parent, Properties,
    },
    NotionApi,
};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use webhook_flows::{create_endpoint, request_handler, send_response};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler(
    _headers: Vec<(String, String)>,
    _subpath: String,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    logger::init();
    match qry.get("date") {
        Some(date) if date.is_string() => {
            let notion = NotionApi::new(std::env::var("NOTION_INTERNAL_SECRET").unwrap()).unwrap();
            let page = new_page(date.as_str().unwrap());
            _ = notion.create_page(page).await;
            send_response(200, vec![], b"okok".to_vec());
        }
        _ => {
            send_response(400, vec![], b"No date specified".to_vec());
        }
    }
}

fn new_page(date: &str) -> PageCreateRequest {
    let database_id = std::env::var("NOTION_PARENT_DATABASE_ID").unwrap();
    let database_id = DatabaseId::from_str(database_id.as_str()).unwrap();

    let title = RichText::Text {
        rich_text: RichTextCommon {
            plain_text: String::new(),
            href: None,
            annotations: None,
        },
        text: Text {
            content: String::from(date),
            link: None,
        },
    };
    let pd = PropertyValue::Title {
        id: PropertyId::from_str("Date").unwrap(),
        title: vec![title],
    };

    let phtml = PropertyValue::Url {
        id: PropertyId::from_str("Html").unwrap(),
        url: Some(format!(
            "https://flows-access-analysis.s3.us-west-2.amazonaws.com/{}.html",
            date
        )),
    };
    let pcsv = PropertyValue::Url {
        id: PropertyId::from_str("CSV").unwrap(),
        url: Some(format!(
            "https://flows-access-analysis.s3.us-west-2.amazonaws.com/{}.csv",
            date
        )),
    };

    let mut properties = HashMap::new();
    properties.insert(String::from("Date"), pd);
    properties.insert(String::from("Html"), phtml);
    properties.insert(String::from("CSV"), pcsv);
    let properties = Properties { properties };

    PageCreateRequest {
        parent: Parent::Database { database_id },
        properties,
        children: None,
    }
}
