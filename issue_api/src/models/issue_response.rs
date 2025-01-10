use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct IssueError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for IssueError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message,
        }));
        (self.status_code, body).into_response()
    }
}
