use serde::Deserialize;
use code_challenge::Node;
use std::time::{UNIX_EPOCH, Duration};
use chrono::{Utc, prelude::DateTime};

#[derive(Debug, Deserialize)]
struct MemPoolNodeResponse {
    #[serde(rename = "publicKey")]
    public_key: String,
    alias: String,
    #[serde(rename = "firstSeen")]
    first_seen: u64,
    capacity: i64,
}

pub async fn request_nodes(endpoint: &str) -> Result<Vec<Node>, reqwest::Error> {
    let resp: Vec<MemPoolNodeResponse> = reqwest::get(endpoint)
        .await?
        .json()
        .await?;

    Ok(resp
        .into_iter()
        .map(convert_response_to_node)
        .collect())
}

fn convert_response_to_node(resp: MemPoolNodeResponse) -> Node {
    let first_seen = UNIX_EPOCH + Duration::from_secs(resp.first_seen);
    let time_str = DateTime::<Utc>::from(first_seen);

    Node {
        public_key: resp.public_key,
        capacity: resp.capacity,
        alias: resp.alias,
        first_seen: time_str,
    }
}
