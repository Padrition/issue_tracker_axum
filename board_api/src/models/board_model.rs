use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::category::Category;

#[derive(Serialize,Deserialize,Clone)]
pub struct Board{
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    #[serde(rename="createdBy")]
    pub created_by: String,
    pub members: Vec<String>,
    pub categories: Vec<Category>,
    pub issues: Vec<ObjectId>
}

#[derive(Serialize,Deserialize,Clone)]
pub struct BoardCreate{
    pub name: String,
    pub description: String,
}