use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct CurrentUser{
    pub email: String,
    pub login: String,
    pub password_hash: String,
}

#[derive(Serialize,Deserialize)]
pub struct User{
    #[serde(rename="_id")]
    pub id: u32,
    pub email: String,
    pub login: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserCreate{
    pub email: String,
    pub login: String,
    pub password: String
}

#[derive(Serialize,Deserialize)]
pub struct UserInsert{
    pub email: String,
    pub login: String,
    pub password_hash: String,
}