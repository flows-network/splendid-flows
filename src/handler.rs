use discord_flows::{
    application_command_handler,
    model::{
        application::interaction::InteractionResponseType,
        prelude::application::interaction::application_command::ApplicationCommandInteraction,
    },
    Bot, ProvidedBot,
};
use flowsnet_platform_sdk::logger;
use serde_json::Value;

#[application_command_handler]
pub async fn handler(ac: ApplicationCommandInteraction) {
    h(ac).await;
}

async fn h(ac: ApplicationCommandInteraction) {
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

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    client.set_application_id(ac.application_id.into());
    _ = client
        .edit_original_interaction_response(
            &ac.token,
            &serde_json::json!({
                "content": "Pong"
            }),
        )
        .await;

    log::debug!("Receive application command: {}", ac.data.name);
}
