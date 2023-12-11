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

pub async fn assign(client: &Http, ac: &ApplicationCommandInteraction, tc: &GuildChannel) {
    let thread_link = util::compose_thread_link(tc);
    // Initialize the Airtable client.
    let airtable = Airtable::new_from_env();

    let table_name = std::env::var("AIRTABLE_TABLE_NAME").unwrap();

    // Get the existing records from a table.
    let records: Vec<Record<Project>> = airtable
        .filter_records(
            table_name.as_str(),
            None,
            FIELDS.to_vec(),
            Some(format!(r#"{{Thread}} = "{thread_link}""#).as_str()),
        )
        .await
        .unwrap();

    let msg = match records.into_iter().next() {
        Some(mut r) => {
            // Update the existing record
            let member_opt = ac.data.options.iter().find(|o| o.name == "member");
            let assignee = member_opt.unwrap().value.clone().unwrap();
            let assignee = assignee.as_str().unwrap();
            r.fields.assignee = Some(assignee.to_owned());
            airtable
                .update_records(table_name.as_str(), vec![r])
                .await
                .unwrap();
            "Task updated"
        }
        None => "This thread has not been made to a task",
    };

    _ = client
        .edit_original_interaction_response(
            &ac.token,
            &serde_json::json!({
                "content": msg
            }),
        )
        .await;
}
