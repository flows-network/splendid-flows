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
    _qry: Vec<(String, String)>,
    _subpath: String,
    _headers: HashMap<String, Value>,
    _body: Vec<u8>,
) {
    logger::init();
    let notion = NotionApi::new(std::env::var("NOTION_INTERNAL_SECRET").unwrap()).unwrap();
    let page = new_page();
    let r = notion.create_page(page).await;
    log::debug!("{:?}", r);
    send_response(200, vec![], vec![]);
}

fn new_page() -> PageCreateRequest {
    let database_id = std::env::var("NOTION_PARENT_DATABASE_ID").unwrap();
    let database_id = DatabaseId::from_str(database_id.as_str()).unwrap();

    let title = RichText::Text {
        rich_text: RichTextCommon {
            plain_text: String::from("New"),
            href: None,
            annotations: None,
        },
        text: Text {
            content: String::from("New Content"),
            link: None,
        },
    };
    let pv = PropertyValue::Title {
        id: PropertyId::from_str("title").unwrap(),
        title: vec![title],
    };
    let mut properties = HashMap::new();
    properties.insert(String::from("title"), pv);
    let properties = Properties { properties };

    PageCreateRequest {
        parent: Parent::Database { database_id },
        properties,
        children: None,
    }
}
