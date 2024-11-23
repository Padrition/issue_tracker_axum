use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Payload{
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse{
    pub access_token: String,
    pub refresh_token: String,
}