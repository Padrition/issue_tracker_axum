use mongodb::Client;

pub async fn connect_to_mongo() -> Client {
    tracing::info!("Connecting to mongodb");
    let connection_str = std::env::var("MONGO_URL").unwrap_or_else(|_| {
        tracing::info!("failed to find MONGO_URL env connecting to fallback URL");
        "mongodb://localhost:27017/".to_string()
    });

    Client::with_uri_str(connection_str).await.unwrap()
}
