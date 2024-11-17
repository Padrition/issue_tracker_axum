use axum::{middleware, routing::{get, post}, Router};

use crate::{middlewares::auth_middleware::authorization_middleware, services::{auth_service::sign_in, hello_service::hello}};

pub fn auth_routers() -> Router {
    Router::new()
        .route("/signin", post(sign_in))
        .route("/hello", get(hello).layer(middleware::from_fn(authorization_middleware)))
}