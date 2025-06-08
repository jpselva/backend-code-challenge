use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Node {
    pub public_key: String,
    pub capacity: f64,
    pub alias: String,
    pub first_seen: String
}
