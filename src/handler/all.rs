use crate::typo::*;
use airtable_api::{Airtable, Record};

const EMOJIS: &[&str] = &["pj_todo", "pj_inprogress", "pj_done"];

pub async fn all() -> String {
    let emojis = EMOJIS
        .iter()
        .map(|e| {
            let eid = store_flows::get(*e).unwrap();
            format!("<:{}:{}>", e, eid)
        })
        .collect::<Vec<String>>();

    // Initialize the Airtable client.
    let airtable = Airtable::new_from_env();

    let table_name = std::env::var("AIRTABLE_TABLE_NAME").unwrap();

    // Get the existing records from a table.
    let records: Vec<Record<Project>> = airtable
        .filter_records(table_name.as_str(), None, FIELDS.to_vec(), Some("1=1"))
        .await
        .unwrap();

    records
        .iter()
        .map(|r| {
            let emoji = match r.fields.status {
                Status::Todo => &emojis[0],
                Status::InProgress => &emojis[1],
                Status::Done => &emojis[2],
            };
            format!("{} {}", emoji, r.fields.thread)
        })
        .collect::<Vec<String>>()
        .join("\n")
}
