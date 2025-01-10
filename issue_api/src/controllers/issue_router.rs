use auth::auth_middleware::authorization_middleware;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use mongodb::Client;

use crate::{
    service::issue_service::{create_issue, get_issue, get_issues},
    utils::app_state::AppState,
};

pub fn issue_router(client: Client) -> Router {
    let collection_user = client
        .database(&std::env::var("DATABASE_NAME").expect("No DATABASE_NAME env present."))
        .collection("users");

    let database =
        client.database(&std::env::var("DATABASE_NAME").expect("No DATABASE_NAME env present."));

    let state = AppState {
        board_collection: database.collection("board"),
        issue_collection: database.collection("issues"),
    };

    Router::new()
        .route(
            "/create",
            post(create_issue).layer(middleware::from_fn_with_state(
                collection_user.clone(),
                authorization_middleware,
            )),
        )
        .route(
            "/board/:id",
            get(get_issues).layer(middleware::from_fn_with_state(
                collection_user.clone(),
                authorization_middleware,
            )),
        )
        .route(
            "/:id",
            get(get_issue).layer(middleware::from_fn_with_state(
                collection_user.clone(),
                authorization_middleware,
            )),
        )
        .with_state(state)
}
