use axum::{
    Router, 
    routing::get,
};
use std::time::Duration;
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/nodes", get(get_nodes));

    tokio::spawn(async { pool_nodes().await });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct LightningNode {
    #[serde(rename = "publicKey")]
    public_key: String,
    alias: String,
    channels: u32,
    capacity: u64,
}

async fn pool_nodes() {
    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let res = request_nodes().await;
        if let Err(msg) = res {
            tracing::warn!("Fetching list of nodes failed: {msg}");
        }
    };
}

async fn request_nodes() -> Result<Vec<LightningNode>, reqwest::Error> {
    let res: Vec<LightningNode> = reqwest::get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
        .await?
        .json()
        .await?;
    Ok(res)
}

async fn get_nodes() -> &'static str {
    "UNDER CONSTRUCTION"
}


