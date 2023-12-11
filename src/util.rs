use discord_flows::model::GuildChannel;

// Something like below:
// https://discord.com/channels/938591043560017950/1183582341751373824/threads/1183591424852250674
pub fn compose_thread_link(tc: &GuildChannel) -> String {
    // tc has been ensured to be a thread
    // so it has parent_id
    format!(
        "https://discord.com/channels/{}/{}/threads/{}",
        tc.guild_id,
        tc.parent_id.unwrap(),
        tc.id
    )
}
