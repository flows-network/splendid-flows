use discord_flows::{http::Http, model::GuildChannel};

const NAMES: &[(&str, &str)] = &[
    ("pj_todo", include_str!("../emoji/todo")),
    ("pj_inprogress", include_str!("../emoji/inprogress")),
    ("pj_done", include_str!("../emoji/done")),
];
pub async fn emoji(client: &Http, tc: &GuildChannel) -> &'static str {
    let guild_id = tc.guild_id.as_u64().to_owned();

    let emojis = client.get_emojis(guild_id).await.unwrap();
    for emoji in emojis.iter() {
        for n in NAMES.iter() {
            if emoji.name.starts_with(n.0) {
                client
                    .delete_emoji(guild_id, emoji.id.into())
                    .await
                    .unwrap();
                let body = serde_json::json!({
                    "name": n.0,
                    "image": n.1,
                    "roles": [guild_id]
                });
                client.create_emoji(guild_id, &body, None).await.unwrap();
            }
        }
    }

    "Emojis have been created"
}
