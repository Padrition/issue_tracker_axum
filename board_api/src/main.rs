use std::{env, time::Duration};
use auth::{shutdown_signal, utils::db};
use axum::{http::Request,response::Response, Router};
use controllers::{board_routers::board_routers, fallback_controller::fallback_route};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::TcpListener;

mod models;
mod controllers;
mod services;

#[tokio::main]
async fn main(){
    dotenvy::dotenv().expect("Could not load .env file");

    let addr = env::var("MICROSERVICE_ADDRESS")
        .expect("Could not interpret key MICROSERVICE_ADDRESS");

    let port = env::var("BOARD_API_PORT")
        .expect("Could not interpret key BOARD_API_PORT");

    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_|{
            format!("{}=debug,tower-http=debug", env!("CARGO_CRATE_NAME")).into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let client = db::connect_to_mongo().await;

    let microservice_address = format!("{}:{}", addr, port);

    let listener = TcpListener::bind(&microservice_address)
        .await
        .unwrap();

    let app = Router::new()
        .merge(board_routers(client))
        .fallback(fallback_route)
        .layer((
            TraceLayer::new_for_http()
            .on_request(|_request: &Request<_>, _span: &Span|{
                tracing::info!("Received request: {} {}", _request.method(),_request.uri())
            })
            .on_response(|_response: &Response, _latency: Duration, _span: &Span|{
                tracing::info!("Response generated in {:?} with status {}", _latency, _response.status())
            }),
            TimeoutLayer::new(Duration::from_secs(10)),
        ));

    tracing::info!("Listening on {}", &microservice_address);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Error serving application.");
}