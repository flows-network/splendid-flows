use discord_flows::http::HttpBuilder;
use flowsnet_platform_sdk::logger;

pub async fn register_commands() {
    logger::init();
    let discord_token = std::env::var("DISCORD_TOKEN").unwrap();
    let app_id = std::env::var("DISCORD_APP_ID").unwrap();
    let guild_id = std::env::var("DISCORD_GUILD_ID").unwrap().parse().unwrap();

    let http_client = HttpBuilder::new(discord_token)
        .application_id(app_id.parse().unwrap())
        .build();

    // Delete all the old commands
    if let Ok(cs) = http_client.get_guild_application_commands(guild_id).await {
        for c in cs.iter() {
            _ = http_client
                .delete_guild_application_command(guild_id, c.id.into())
                .await;
        }
    }

    // Refer https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    let commands = serde_json::json!([
        {
            "name": "pj_make_task",
            "description": "Make this thread as a task",
        },{
            "name": "pj_assign",
            "description": "Assign this task to user",
            "options": [
                {
                    "name": "member",
                    "description": "The member to assign",
                    "type": 6,
                    "required": true
                }
            ]
        },{
            "name": "pj_evolve",
            "description": "Set the status of this task",
            "options": [
                {
                    "name": "status",
                    "description": "The status select",
                    "type": 3,
                    "required": true,
                    "choices": [
                        {
                            "name": "Todo",
                            "value": "Todo"
                        },
                        {
                            "name": "In progress",
                            "value": "In progress"
                        },
                        {
                            "name": "Done",
                            "value": "Done"
                        },
                    ]
                }
            ]
        }
    ]);

    match http_client
        .create_guild_application_commands(guild_id, &commands)
        .await
    {
        Ok(_) => log::info!("Successfully registered commands"),
        Err(err) => log::error!("Error registering commands: {}", err),
    }
}
