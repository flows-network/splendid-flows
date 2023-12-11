use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Todo,
    #[serde(rename(deserialize = "In progress"))]
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Project {
    thread: String,
    title: String,
    assignee: Option<String>,
    status: Status,
}
