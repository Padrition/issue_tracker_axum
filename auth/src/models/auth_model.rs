use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message,
        }));
        (self.status_code, body).into_response()
    }
}
