use async_trait::async_trait;
use serde_json::Value;
use std::error::Error;

pub mod clickhouse_connector;
pub mod velodb_connector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    ClickHouse,
    VeloDB,
}

impl DatabaseType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "clickhouse" => Ok(DatabaseType::ClickHouse),
            "velodb" => Ok(DatabaseType::VeloDB),
            _ => Err(format!("Unsupported database type: {}", s)),
        }
    }
}

#[async_trait]
pub trait DatabaseConnector: Send + Sync {
    /// Execute a query and return results as JSON rows
    async fn execute_query(
        &self,
        query: &str,
        format: &str,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>>;

    /// Execute a query and return results as JSON values
    async fn execute_query_json(
        &self,
        query: &str,
    ) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>>;

    /// Execute a DDL statement (CREATE, ALTER, DROP)
    async fn execute_ddl(&self, query: &str) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// Insert a row into the graph catalog
    async fn insert_catalog(
        &self,
        graph_name: &str,
        element_type: &str,
        element_name: &str,
        properties: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// Query the graph catalog
    async fn query_catalog(&self) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>>;

    /// Get the database type
    fn get_type(&self) -> DatabaseType;

    /// Clone the connector as a trait object
    fn clone_box(&self) -> Box<dyn DatabaseConnector>;
}

impl Clone for Box<dyn DatabaseConnector> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
