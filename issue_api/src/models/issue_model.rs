use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Issue {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: Priority,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IssueCreate {
    pub board_id: String,
    pub title: String,
    pub description: String,
    pub status: Option<String>,
    pub priority: Option<Priority>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IssueUpdate {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<Priority>,
}
