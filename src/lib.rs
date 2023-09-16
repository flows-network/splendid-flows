use discord_flows::{
    application_command_handler,
    http::HttpBuilder,
    message_handler,
    model::{
        application::interaction::InteractionResponseType,
        application_command::CommandDataOptionValue,
        prelude::application::interaction::application_command::ApplicationCommandInteraction,
        Message,
    },
    Bot, ProvidedBot,
};
use flowsnet_platform_sdk::logger;
use serde_json::Value;

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
    let client = bot.get_client();
    _ = client
        .send_message(
            msg.channel_id.into(),
            &serde_json::json!({
                "content": msg.content,
            }),
        )
        .await;
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
}

async fn register_commands() {
    let http_client = HttpBuilder::new(DISCORD_TOKEN.as_str())
        .application_id(APPLICATION_ID.parse().unwrap())
        .build();

    let command = serde_json::json!({
        "name": "Collect thread",
        "description": "Gather and collect all of the messages of thread"
    });

    match http_client
        .create_global_application_command(&command)
        .await
    {
        Ok(_) => log::info!("Successfully registered command"),
        Err(err) => log::error!("Error registering command: {}", err),
    }
}
