use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use mongodb::Client;

use crate::{
    middlewares::auth_middleware::authorization_middleware,
    services::{
        auth_service::{refresh, sign_in},
        hello_service::hello,
        user_service::create_user,
    },
};

pub fn auth_routers(client: Client) -> Router {
    let collection = client
        .database(&std::env::var("DATABASE_NAME").expect("No DATABASE_NAME env present."))
        .collection("users");

    Router::new()
        .route("/signin", post(sign_in))
        .route("/refresh", post(refresh))
        .route(
            "/hello",
            get(hello).layer(middleware::from_fn_with_state(
                collection.clone(),
                authorization_middleware,
            )),
        )
        .route("/create", post(create_user))
        .with_state(collection)
}
