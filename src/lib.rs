use discord_flows::{message_handler, model::Message, Bot, ProvidedBot};
use flowsnet_platform_sdk::logger;

mod commands;
mod handler;

pub use handler::*;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    let discord_token = std::env::var("DISCORD_TOKEN").unwrap();
    let bot = ProvidedBot::new(&discord_token);

    commands::register_commands().await;

    bot.listen_to_messages().await;

    bot.listen_to_application_commands().await;
}

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

    /*
    _ = client
        .send_message(
            channel_id.into(),
            &serde_json::json!({
                "content": content,
            }),
        )
        .await;
    */
}
