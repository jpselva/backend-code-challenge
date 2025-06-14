use code_challenge::Node;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct NodeResponse {
    pub public_key: String,
    pub capacity: String,
    pub alias: String,
    pub first_seen: String,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub order: Option<String>,
}

pub fn get_response_from_node(node: Node) -> NodeResponse {
    // convert from satoshis to btc
    let capacity = (node.capacity as f64 / 100000000.0).to_string();

    // format date string
    let first_seen = node.first_seen.format("%Y-%m-%dT%H:%M:%S%Z").to_string();

    NodeResponse {
        public_key: node.public_key,
        capacity,
        alias: node.alias,
        first_seen,
    }
}
