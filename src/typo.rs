use serde::{Deserialize, Serialize};

pub const FIELDS: &[&str] = &["Thread", "Title", "Assignee", "Status"];

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Todo,
    #[serde(rename = "In progress")]
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Project {
    pub thread: String,
    pub title: String,
    pub assignee: Option<String>,
    pub status: Status,
}
