use axum::{
    Router, 
    routing::get,
    extract::{State, Query},
    http::StatusCode,
    Json,
};
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::postgres::{PgPool, PgPoolOptions};
use api::{get_response_from_node, NodeResponse, QueryParams};
use db::{update_node_database, retrieve_nodes_from_database};
use mempool_api::request_nodes;

mod api;
mod db;
mod mempool_api;

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

    // spawn task to pool external api periodically
    let db_clone = db_pool.clone();
    tokio::spawn(async { pool_nodes(nodes_endpoint, db_clone).await });

    let app = Router::new()
        .route("/nodes", get(get_nodes))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn order_param_to_enum(order_str: &str) -> OrderBy {
    match order_str {
        "capacity" => OrderBy::Capacity,
        _ => OrderBy::None,
    }
}

async fn get_nodes(State(db_pool): State<PgPool>, params: Query<QueryParams>) -> 
                                   Result<Json<Vec<NodeResponse>>, StatusCode> {
    let result = match params.order.as_deref() {
        None => get_nodes_as_json(db_pool, OrderBy::None).await,
        Some(order) => get_nodes_as_json(db_pool, order_param_to_enum(order)).await,
    };

    if let Ok(json) = result {
        Ok(json)
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

enum OrderBy {
    None,
    Capacity, 
}

async fn get_nodes_as_json(db_pool: PgPool, order_by: OrderBy) 
    -> Result<Json<Vec<NodeResponse>>, Box<dyn std::error::Error>> {
    let mut nodes = retrieve_nodes_from_database(&db_pool).await?;

    match order_by {
        OrderBy::Capacity => nodes.sort_by_key(|n| n.capacity),
        OrderBy::None => {},
    }

    let nodes = nodes.into_iter().map(get_response_from_node).collect();

    Ok(Json(nodes))
}

async fn pool_nodes(endpoint: String, db_pool: PgPool) {
    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let res = request_nodes(&endpoint).await;

        if let Err(msg) = res {
            tracing::warn!("Fetching list of nodes failed: {msg}");
        } else if let Ok(nodes) = res {
            let result = update_node_database(nodes, &db_pool);

            if let Err(msg) = result.await {
                tracing::warn!("Database update failed: {msg}");
            }
        }
    };
}
