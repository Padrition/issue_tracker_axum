use auth::auth_middleware::authorization_middleware;
use axum::{middleware, response::IntoResponse, routing::get, Json, Router};
use tokio::net::TcpListener;
use serde_json::json;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not load .env file");
    
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    // let app = Router::new()
    //     // .route("/hello", get(hello).layer(middleware::from_fn(authorization_middleware)));

    // axum::serve(listener, app)
    //     .await
    //     .unwrap();
}

async fn hello()-> impl IntoResponse{
    Json(json!({
        "message" : "Hello, authorized!"
    }))
}
