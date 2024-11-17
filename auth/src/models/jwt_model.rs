use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Payload{
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}