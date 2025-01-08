use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub login: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserCreate {
    pub email: String,
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInsert {
    pub email: String,
    pub login: String,
    pub password_hash: String,
}
