use crate::typo::*;
use crate::util;
use airtable_api::{Airtable, Record};
use discord_flows::{
    http::Http,
    model::{
        prelude::application::interaction::application_command::ApplicationCommandInteraction,
        GuildChannel,
    },
};

pub async fn task(client: &Http, ac: &ApplicationCommandInteraction, tc: &GuildChannel) {
    let thread_link = util::compose_thread_link(tc);
    // Initialize the Airtable client.
    let airtable = Airtable::new_from_env();

    // Get the current records from a table.
    let records: Vec<Record<Project>> = airtable
        .list_records(
            "Project",
            "Grid view",
            vec!["Thread", "Title", "Assignee", "Status"],
        )
        .await
        .unwrap();
    // Iterate over the records.
    for (i, record) in records.iter().enumerate() {
        println!("{} - {:?}", i, record);
    }

    _ = client
        .edit_original_interaction_response(
            &ac.token,
            &serde_json::json!({
                "content": "--"
            }),
        )
        .await;
}
