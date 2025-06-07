use axum::{
    Router, 
    routing::get,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/nodes", get(get_nodes));

    let handler = tokio::spawn(async { pool_nodes().await });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn pool_nodes() -> Result<(), reqwest::Error> {
    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let res = reqwest::get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
            .await?
            .text()
            .await?;
    };
}

async fn get_nodes() -> &'static str {
    "UNDER CONSTRUCTION"
}


