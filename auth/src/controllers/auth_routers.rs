use axum::{middleware, routing::{get, post}, Router};
use mongodb::Client;

use crate::{middlewares::auth_middleware::authorization_middleware, services::{auth_service::sign_in, hello_service::hello, user_service::create_user}};

pub fn auth_routers(client: Client) -> Router {
    let collection = client.database("dira").collection("users");

    Router::new()
        .route("/signin", get(sign_in))
        .route("/hello", get(hello).layer(middleware::from_fn_with_state(collection.clone(),authorization_middleware)))
        .route("/create", post(create_user))
        .with_state(collection)
}