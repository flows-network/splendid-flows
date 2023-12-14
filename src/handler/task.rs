use crate::typo::*;
use crate::util;
use airtable_api::{Airtable, Record};
use discord_flows::model::GuildChannel;

pub async fn task(tc: &GuildChannel) -> &'static str {
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

    match records.into_iter().next() {
        Some(mut r) => {
            // Update the existing record
            r.fields.title = tc.name.clone();
            airtable
                .update_records(table_name.as_str(), vec![r])
                .await
                .unwrap();
            "Task updated"
        }
        None => {
            // Create a new record
            let r = Record {
                id: String::new(),
                fields: Project {
                    thread: thread_link,
                    title: tc.name.clone(),
                    assignee: None,
                    status: Status::Todo,
                },
                created_time: None,
            };
            airtable
                .create_records(table_name.as_str(), vec![r])
                .await
                .unwrap();
            "Task created"
        }
    }
}
