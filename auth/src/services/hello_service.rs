use axum::{response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::models::user_model::CurrentUser;

pub async fn hello(Extension(current_user): Extension<CurrentUser>) -> impl IntoResponse{
    Json(json!({
        "message" : format!("Hello, {}",current_user.email)
    }))
}
