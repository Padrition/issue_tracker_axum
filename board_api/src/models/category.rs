use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Clone)]
pub struct Category{
    pub name: String,
    pub color: String,
}