use crate::typo::*;
use airtable_api::{Airtable, Record};

pub async fn all() -> String {
    // Initialize the Airtable client.
    let airtable = Airtable::new_from_env();

    let table_name = std::env::var("AIRTABLE_TABLE_NAME").unwrap();

    // Get the existing records from a table.
    let records: Vec<Record<Project>> = airtable
        .filter_records(table_name.as_str(), None, FIELDS.to_vec(), Some("TRUE"))
        .await
        .unwrap();

    records
        .iter()
        .map(|r| {
            let emoji = match r.fields.status {
                Status::Todo => ":pj_todo:",
                Status::InProgress => ":pj_inprogress:",
                Status::Done => ":pj_done:",
            };
            format!("{} {}", emoji, r.fields.thread)
        })
        .collect::<Vec<String>>()
        .join("\n")
}
