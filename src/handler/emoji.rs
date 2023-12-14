use discord_flows::{http::Http, model::GuildChannel};

pub async fn emoji(client: &Http, tc: &GuildChannel) {
    let guild_id = tc.guild_id.as_u64().to_owned();

    let emojis = client.get_emojis(guild_id).await.unwrap();
    for emoji in emojis.iter() {
        if emoji.name.starts_with("Todo") {
            client
                .delete_emoji(guild_id, emoji.id.into())
                .await
                .unwrap();
        }
    }

    let todo = include_str!("../emoji/todo");
    let body = serde_json::json!({
        "name": "pj_todo",
        "image": todo,
        "roles": [guild_id]
    });
    client.create_emoji(guild_id, &body, None).await.unwrap();
}
