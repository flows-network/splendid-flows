use discord_flows::http::HttpBuilder;

pub async fn register_commands() {
    let discord_token = std::env::var("DISCORD_TOKEN").unwrap();
    let app_id = std::env::var("DISCORD_APP_ID").unwrap();
    let guild_id = std::env::var("DISCORD_GUILD_ID").unwrap();

    let commands = serde_json::json!({
        "name": "task",
        "description": "Make this thread as a task",
    });

    let http_client = HttpBuilder::new(discord_token)
        .application_id(app_id.parse().unwrap())
        .build();

    match http_client
        .create_guild_application_commands(guild_id.parse().unwrap(), &commands)
        .await
    {
        Ok(_) => log::info!("Successfully registered command"),
        Err(err) => log::error!("Error registering command: {}", err),
    }
}
