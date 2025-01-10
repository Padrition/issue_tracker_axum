use axum::Router;
use mongodb::Client;

pub fn issue_router(client: Client) -> Router {
    Router::new()
}
