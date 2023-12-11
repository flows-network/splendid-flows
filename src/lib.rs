use discord_flows::{
    application_command_handler,
    model::{
        application::interaction::InteractionResponseType,
        prelude::application::interaction::application_command::ApplicationCommandInteraction,
        Channel,
    },
    Bot, ProvidedBot,
};
use flowsnet_platform_sdk::logger;
use serde_json::Value;

mod commands;
mod handler;
mod typo;
mod util;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    let discord_token = std::env::var("DISCORD_TOKEN").unwrap();
    let bot = ProvidedBot::new(&discord_token);

    commands::register_commands().await;
    bot.listen_to_application_commands().await;

    // bot.listen_to_messages().await;
}

#[application_command_handler]
pub async fn handler(ac: ApplicationCommandInteraction) {
    logger::init();
    let discord_token = std::env::var("DISCORD_TOKEN").unwrap();
    let bot = ProvidedBot::new(discord_token);
    let client = bot.get_client();

    _ = client
        .create_interaction_response(
            ac.id.into(),
            &ac.token,
            &serde_json::json!({
                "type": InteractionResponseType::DeferredChannelMessageWithSource as u8,
            }),
        )
        .await;

    // Necessary for doing following action
    client.set_application_id(ac.application_id.into());

    let channel = client.get_channel(ac.channel_id.into()).await.unwrap();

    let mut error_msg = String::new();

    // Ensure it is a GuildChannel
    match channel {
        Channel::Guild(gc) => {
            // Ensure message is sent from a thread
            match gc.parent_id {
                Some(pc) => {
                    let parent_channel_id = std::env::var("DISCORD_PROJECT_CHANNEL_ID").unwrap();
                    match parent_channel_id.parse::<u64>().unwrap() == pc.as_u64().to_owned() {
                        true => match ac.data.name.as_str() {
                            "pj_make_task" => {
                                handler::task(&client, &ac, &gc).await;
                            }
                            "pj_assign" => {
                                handler::assign(&client, &ac, &gc).await;
                            }
                            "pj_evolve" => {
                                handler::evolve(&client, &ac, &gc).await;
                            }
                            _ => {}
                        },
                        false => {
                            error_msg = format!("Only work in channel `{}`", parent_channel_id);
                        }
                    }
                }
                None => error_msg = String::from("Not in a thread"),
            }
        }
        _ => {
            error_msg = String::from("Not in a thread");
        }
    }

    if error_msg != "" {
        _ = client
            .edit_original_interaction_response(
                &ac.token,
                &serde_json::json!({
                    "content": error_msg
                }),
            )
            .await;
    }
}

/*
#[message_handler]
async fn handle(msg: Message) {
    logger::init();
    let token = std::env::var("DISCORD_TOKEN").unwrap();

    let bot = ProvidedBot::new(token);
    let client = bot.get_client();
    let channel_id = msg.channel_id;
    let content = msg.content;

    if msg.author.bot {
        log::debug!("message from bot");
        return;
    }

    _ = client
        .send_message(
            channel_id.into(),
            &serde_json::json!({
                "content": content,
            }),
        )
        .await;
}
*/
