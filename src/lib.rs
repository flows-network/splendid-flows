use discord_flows::{
    application_command_handler,
    http::HttpBuilder,
    message_handler,
    model::{
        application::interaction::InteractionResponseType,
        // application_command::CommandDataOptionValue,
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

lazy_static! {
    static ref DISCORD_TOKEN: String =
        std::env::var("DISCORD_TOKEN").expect("No discord token configure");
    static ref APPLICATION_ID: String =
        std::env::var("APPLICATION_ID").expect("No application_id configure");
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    logger::init();
    let bot = ProvidedBot::new(DISCORD_TOKEN.as_str());

    register_commands().await;

    bot.listen_to_messages().await;

    bot.listen_to_application_commands().await;
}

#[message_handler]
async fn handle(msg: Message) {
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

#[application_command_handler]
async fn handler(ac: ApplicationCommandInteraction) {
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

    let command = serde_json::json!({
        "name": "collect_thread",
        "description": "Gather and collect all of the messages of thread",
        "options": []
    });

    match http_client
        .create_global_application_command(&command)
        .await
    {
        Ok(_) => log::info!("Successfully registered command"),
        Err(err) => log::error!("Error registering command: {}", err),
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
        children: None,
    }
}
