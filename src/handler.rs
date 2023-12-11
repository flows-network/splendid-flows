use discord_flows::{
    application_command_handler,
    model::prelude::application::interaction::application_command::ApplicationCommandInteraction,
    Bot, ProvidedBot,
};
use flowsnet_platform_sdk::logger;
use serde_json::Value;

#[application_command_handler]
async fn handler(ac: ApplicationCommandInteraction) {
    h(ac).await;
}

async fn h(ac: ApplicationCommandInteraction) {
    logger::init();
    let discord_token = std::env::var("DISCORD_TOKEN").unwrap();
    let bot = ProvidedBot::new(discord_token);
    let client = bot.get_client();

    client.set_application_id(ac.application_id.into());

    log::debug!("Receive application command: {}", ac.data.name);
}
