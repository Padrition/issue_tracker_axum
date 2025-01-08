use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serialize};

use super::category::Category;

#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    pub members: Vec<String>,
    pub categories: Vec<Category>,
    pub issues: Vec<ObjectId>,
}

#[derive(Serialize)]
pub struct BoardResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "isCreator")]
    pub is_creator: bool,
    pub members: Vec<String>,
    pub categories: Vec<Category>,
    pub issues: Vec<ObjectId>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BoardCreate {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BoardUpdate {
    #[serde(deserialize_with = "deserialize_object_id")]
    pub id: ObjectId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub members: Option<Vec<String>>,
    pub categories: Option<Vec<Category>>,
}

fn deserialize_object_id<'de, D>(deserializer: D) -> Result<ObjectId, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    ObjectId::parse_str(&s).map_err(serde::de::Error::custom)
}
