use brahmand::server::velodb_client;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("=== VeloDB Connection Test ===\n");

    // Get the VeloDB connection pool
    let pool = velodb_client::get_pool().await;

    // Test the connection
    velodb_client::test_connection(&pool).await?;

    // Get list of tables
    println!("\nFetching tables...");
    let tables = velodb_client::get_tables(&pool).await?;

    if tables.is_empty() {
        println!("No tables found in the database.");
    } else {
        println!("\nFound {} table(s):", tables.len());
        for (i, table) in tables.iter().enumerate() {
            println!("  {}. {}", i + 1, table);
        }

        // Describe each table
        println!("\n=== Table Structures ===");
        for table in &tables {
            velodb_client::describe_table(&pool, table).await?;
        }
    }

    // Run a sample query
    println!("\n=== Running Sample Query ===");
    let rows = sqlx::query("SELECT DATABASE() as current_db")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        use sqlx::Row;
        let db: String = row.try_get("current_db")?;
        println!("Current database: {}", db);
    }

    println!("\n=== Test Complete ===");

    Ok(())
}
