use discord_flows::{
    application_command_handler,
    http::{Http, HttpBuilder},
    message_handler,
    model::{
        application::interaction::InteractionResponseType,
        application_command::CommandDataOptionValue,
        prelude::{
            application::interaction::application_command::ApplicationCommandInteraction, Channel,
            Message,
        },
    },
    Bot, ProvidedBot,
};
use flowsnet_platform_sdk::logger;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

use notion_flows::notion::{
    ids::{DatabaseId, PropertyId},
    models::{
        block::{CreateBlock, Text as BlockText, TextAndChildren},
        properties::PropertyValue,
        text::{RichText, RichTextCommon, Text, TextColor},
        PageCreateRequest, Parent, Properties,
    },
    NotionApi,
};

use lazy_static::lazy_static;

use webhook_flows::{create_endpoint, request_handler, send_response};

mod auth;

use auth::NotionAuth;

const SUCCESS_HTML: &[u8] = include_bytes!("success.html");

lazy_static! {
    static ref DISCORD_TOKEN: String =
        std::env::var("DISCORD_TOKEN").expect("No discord token configure");
    static ref APPLICATION_ID: String =
        std::env::var("APPLICATION_ID").expect("No application_id configure");
    static ref NOTION_AUTH_URL: String =
        std::env::var("NOTION_AUTH_URL").expect("No notion_auth_url configure");
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    logger::init();

    create_endpoint().await;

    let bot = ProvidedBot::new(DISCORD_TOKEN.as_str());

    register_commands().await;

    bot.listen_to_messages().await;

    bot.listen_to_application_commands().await;
}

#[message_handler]
async fn discord_message_handler(msg: Message) {
    logger::init();
    let bot = ProvidedBot::new(DISCORD_TOKEN.as_str());

    if msg.author.bot {
        return;
    }
    // let client = bot.get_client();
    // _ = client
    //     .send_message(
    //         msg.channel_id.into(),
    //         &serde_json::json!({
    //             "content": msg.content,
    //         }),
    //     )
    //     .await;
}

#[request_handler]
async fn webhook_handler(
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
                        let mut new_authors = authors
                            .into_iter()
                            .filter(|t| t.bot_id.ne(&author.bot_id))
                            .collect::<Vec<NotionAuth>>();
                        new_authors.push(author);
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

#[application_command_handler]
async fn discord_slash_command_handler(ac: ApplicationCommandInteraction) {
    logger::init();
    let bot = ProvidedBot::new(DISCORD_TOKEN.as_str());
    let client = bot.get_client();

    client.set_application_id(ac.application_id.into());

    _ = client
        .create_interaction_response(
            ac.id.into(),
            &ac.token,
            &serde_json::json!({
                "type": InteractionResponseType::DeferredChannelMessageWithSource as u8,
            }),
        )
        .await;

    match ac.data.name.as_str() {
        "collect_thread" => {
            let options = ac
                .data
                .options
                .iter()
                .filter_map(|o| match o.name.as_str() {
                    "notion_workspace" => match o.resolved.as_ref() {
                        Some(s) => match s {
                            CommandDataOptionValue::String(s) => Some(s.clone()),
                            _ => Some(String::new()),
                        },
                        _ => Some(String::new()),
                    },
                    "notion_database_id" => match o.resolved.as_ref() {
                        Some(s) => match s {
                            CommandDataOptionValue::String(s) => Some(s.clone()),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                })
                .collect();
            collect_gather(client, &ac, options).await;
        }
        "auth_gathering_notion" => {}
        _ => {
            _ = client
                .edit_original_interaction_response(
                    &ac.token,
                    &serde_json::json!({
                        "content":
                            format!(
                                "Click following link to authorized the Notion App\n{}",
                                NOTION_AUTH_URL.as_str()
                            )
                    }),
                )
                .await;
        }
    }
}

async fn collect_gather(client: Http, ac: &ApplicationCommandInteraction, options: Vec<String>) {
    if options.len() != 2 {
        log::error!("Not enough options for Notion");
        return;
    }

    if let Ok(c) = client.get_channel(ac.channel_id.into()).await {
        if let Channel::Guild(gc) = c {
            if let Some(_) = gc.thread_metadata {
                match client
                    .get_messages(ac.channel_id.into(), format!("?limit=100").as_str())
                    .await
                {
                    Ok(messages) if messages.len() > 0 => {
                        let notion =
                            NotionApi::new(std::env::var("NOTION_INTERNAL_SECRET").unwrap())
                                .unwrap();
                        let page = new_page(gc.name.as_str(), messages);
                        match notion.create_page(page).await {
                            Ok(_) => {
                                _ = client
                                    .edit_original_interaction_response(
                                        &ac.token,
                                        &serde_json::json!({
                                            "content": "Messages has been saved to Notion"
                                        }),
                                    )
                                    .await;
                            }
                            Err(_) => {
                                _ = client
                                    .edit_original_interaction_response(
                                        &ac.token,
                                        &serde_json::json!({
                                            "content": "Failed save messages to Notion"
                                        }),
                                    )
                                    .await;
                            }
                        }
                    }
                    _ => {
                        _ = client
                            .edit_original_interaction_response(
                                &ac.token,
                                &serde_json::json!({
                                    "content": "Not find any message from channel"
                                }),
                            )
                            .await;
                    }
                }
                return;
            }
        }
    }
    _ = client
        .edit_original_interaction_response(
            &ac.token,
            &serde_json::json!({
                "content": "Only work in a thread."
            }),
        )
        .await;
}

async fn register_commands() {
    let http_client = HttpBuilder::new(DISCORD_TOKEN.as_str())
        .application_id(APPLICATION_ID.parse().unwrap())
        .build();

    if let Ok(commands) = http_client.get_global_application_commands().await {
        for c in commands.iter() {
            match http_client
                .delete_global_application_command(c.id.into())
                .await
            {
                Err(e) => {
                    log::error!("Failed delete old command '{}': {:?}", c.name, e);
                }
                Ok(_) => {}
            }
        }
    }

    let command = serde_json::json!({
        "name": "auth_gathering_notion",
        "description": "Authorize Notion for gathering messages",
        "options": []
    });

    match http_client
        .create_global_application_command(&command)
        .await
    {
        Ok(_) => log::info!("Successfully registered command 'auth_gathering_notion'"),
        Err(err) => log::error!("Error registering command 'auth_gathering_notion': {}", err),
    }

    let command = serde_json::json!({
        "name": "collect_thread",
        "description": "Gather and collect all of the messages of thread",
        "options": [
            {
                "name": "notion_workspace",
                "description": "The name of the workspace",
                "type": 3,
                "required": false
            },
            {
                "name": "notion_database_id",
                "description": "The database where messages will be gathered into",
                "type": 3,
                "required": true
            }
        ]
    });

    match http_client
        .create_global_application_command(&command)
        .await
    {
        Ok(_) => log::info!("Successfully registered command 'collect_thread'"),
        Err(err) => log::error!("Error registering command 'collect_thread': {}", err),
    }
}

fn new_page(thread_name: &str, messages: Vec<Message>) -> PageCreateRequest {
    let database_id = std::env::var("NOTION_PARENT_DATABASE_ID").unwrap();
    let database_id = DatabaseId::from_str(database_id.as_str()).unwrap();

    let title = RichText::Text {
        rich_text: RichTextCommon {
            plain_text: String::new(),
            href: None,
            annotations: None,
        },
        text: Text {
            content: String::from(thread_name),
            link: None,
        },
    };
    let pt = PropertyValue::Title {
        id: PropertyId::from_str("Thread").unwrap(),
        title: vec![title],
    };

    let mut properties = HashMap::new();
    properties.insert(String::from("Thread"), pt);
    let properties = Properties { properties };

    let mut blocks = vec![];
    for m in messages.into_iter() {
        let header = RichText::Text {
            rich_text: RichTextCommon {
                plain_text: String::new(),
                href: None,
                annotations: None,
            },
            text: Text {
                content: String::from(m.author.name),
                link: None,
            },
        };
        let header_block = CreateBlock::Heading3 {
            heading_3: BlockText {
                rich_text: vec![header],
            },
        };

        blocks.push(header_block);

        let para = RichText::Text {
            rich_text: RichTextCommon {
                plain_text: String::new(),
                href: None,
                annotations: None,
            },
            text: Text {
                content: String::from(m.content),
                link: None,
            },
        };
        let para_block = CreateBlock::Paragraph {
            paragraph: TextAndChildren {
                rich_text: vec![para],
                children: None,
                color: TextColor::Default,
            },
        };

        blocks.push(para_block);
    }

    PageCreateRequest {
        parent: Parent::Database { database_id },
        properties,
        children: Some(blocks),
    }
}
