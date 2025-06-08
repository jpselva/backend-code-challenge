use code_challenge::Node;
use sqlx::postgres::PgPool;

pub async fn update_node_database(nodes: Vec<Node>, db_pool: &PgPool) -> 
    Result<(), Box<dyn std::error::Error>> {
    // Perform entire update inside a transaction so it is atomic
    let mut transaction = db_pool.begin().await?;
    sqlx::query!("DELETE FROM Nodes").execute(&mut *transaction).await?;

    for node in nodes {
        sqlx::query!("INSERT INTO Nodes (public_key, alias, first_seen, capacity) VALUES ($1, $2, $3, $4)", 
            node.public_key, 
            node.alias, 
            node.first_seen, 
            node.capacity
        ).execute(&mut *transaction).await?;
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn retrieve_nodes_from_database(db_pool: &PgPool) -> 
    Result<Vec<Node>, Box<dyn std::error::Error>> {
    let result = sqlx::query_as!(
        Node,
        "SELECT public_key, alias, first_seen, capacity FROM Nodes"
    ).fetch_all(db_pool).await?;

    // Trim spaces from alias string
    Ok(result.into_iter().map(|n| Node {
        public_key: n.public_key,
        alias: n.alias.trim_end().to_string(),
        first_seen: n.first_seen,
        capacity: n.capacity,
    }).collect())
}
