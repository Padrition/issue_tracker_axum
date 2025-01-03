use auth::auth_middleware::authorization_middleware;
use axum::{middleware, routing::post, Router};
use mongodb::Client;

use crate::services::board_service::create_board;


pub fn board_routers(client: Client) -> Router{
    let collection = client.database(&std::env::var("DATABASE_NAME").expect("No DATABASE_NAME env present."))
        .collection("board");

        let collection_user = client.database(&std::env::var("DATABASE_NAME").expect("No DATABASE_NAME env present."))
        .collection("users");

    Router::new()
        .route("/create", post(create_board).layer(middleware::from_fn_with_state(collection_user.clone(), authorization_middleware)))
        .with_state(collection)
}
