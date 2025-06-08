use sqlx::FromRow;
use chrono::prelude::{DateTime, Utc};

#[derive(FromRow, Debug)]
pub struct Node {
    pub public_key: String,
    pub capacity: i64,
    pub alias: String,
    pub first_seen: DateTime<Utc>,
}
