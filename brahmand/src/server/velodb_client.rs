use sqlx::{MySqlPool, Row};
use std::env;

fn read_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} env variable should be set"))
}

pub async fn get_pool() -> MySqlPool {
    let host = read_env_var("VELODB_HOST");
    let port = read_env_var("VELODB_PORT");
    let user = read_env_var("VELODB_USER");
    let password = read_env_var("VELODB_PASSWORD");
    let database = read_env_var("VELODB_DATABASE");

    let connection_string = format!(
        "mysql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    );

    println!("\n VELODB_HOST: {}\n", host);

    MySqlPool::connect(&connection_string)
        .await
        .expect("Failed to connect to VeloDB")
}

pub async fn test_connection(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let row = sqlx::query("SELECT 1 as test")
        .fetch_one(pool)
        .await?;

    let test_val: i32 = row.try_get("test")?;
    println!("VeloDB connection test successful: {}", test_val);

    Ok(())
}

pub async fn get_tables(pool: &MySqlPool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let rows = sqlx::query("SHOW TABLES")
        .fetch_all(pool)
        .await?;

    let mut tables = Vec::new();
    for row in rows {
        // Get the first column value (table name)
        if let Ok(table_name) = row.try_get::<String, _>(0) {
            tables.push(table_name);
        }
    }

    Ok(tables)
}

pub async fn describe_table(pool: &MySqlPool, table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let query = format!("DESCRIBE {}", table_name);
    let rows = sqlx::query(&query)
        .fetch_all(pool)
        .await?;

    println!("\nTable structure for {}:", table_name);
    println!("--------------------------------");
    for row in rows {
        let field: String = row.try_get("Field")?;
        let field_type: String = row.try_get("Type")?;
        println!("  {} : {}", field, field_type);
    }

    Ok(())
}
