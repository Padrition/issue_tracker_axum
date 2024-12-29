use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn fallback_route() -> impl IntoResponse{
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "Not Found".to_string()
        }))
    )
}