use axum::{
    Router, 
    routing::get,
    extract::State,
};
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::{postgres::{PgPool, PgPoolOptions}, Postgres, QueryBuilder, Execute};

mod node_api;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = dotenvy::var("DATABASE_URL")
        .expect("DATABASE_URL not defined");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Can't connect to database");

    let nodes_endpoint = dotenvy::var("NODES_ENDPOINT")
        .expect("NODES_ENDPOINT not defined");

    let db_clone = db_pool.clone();
    tokio::spawn(async { pool_nodes(nodes_endpoint, db_clone).await });

    let app = Router::new()
        .route("/nodes", get(get_nodes))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_nodes(State(db_pool): State<PgPool>) -> &'static str {
    "UNDER CONSTRUCTION"
}

async fn pool_nodes(endpoint: String, db_pool: PgPool) {
    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let res = node_api::request_nodes(&endpoint).await;
        if let Err(msg) = res {
            tracing::warn!("Fetching list of nodes failed: {msg}");
        } else if let Ok(nodes) = res {
            // TODO
        }
    };
}

