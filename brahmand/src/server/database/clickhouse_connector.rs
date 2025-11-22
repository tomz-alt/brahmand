use async_trait::async_trait;
use clickhouse::Client;
use serde_json::Value;
use std::error::Error;
use tokio::io::AsyncBufReadExt;

use super::{DatabaseConnector, DatabaseType};

#[derive(Clone)]
pub struct ClickHouseConnector {
    client: Client,
}

impl ClickHouseConnector {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl DatabaseConnector for ClickHouseConnector {
    async fn execute_query(
        &self,
        query: &str,
        format: &str,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let mut lines = self
            .client
            .clone()
            .query(query)
            .fetch_bytes(format)
            .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })?
            .lines();

        let mut rows: Vec<String> = vec![];
        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })?
        {
            rows.push(line);
        }

        Ok(rows)
    }

    async fn execute_query_json(
        &self,
        query: &str,
    ) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>> {
        let mut lines = self
            .client
            .clone()
            .query(query)
            .fetch_bytes("JSONEachRow")
            .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })?
            .lines();

        let mut rows: Vec<Value> = vec![];
        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })?
        {
            let value: Value = serde_json::from_str(&line)
                .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })?;
            rows.push(value);
        }

        Ok(rows)
    }

    async fn execute_ddl(&self, query: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.client
            .clone()
            .with_option("wait_end_of_query", "1")
            .query(query)
            .execute()
            .await
            .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })?;

        Ok(())
    }

    async fn insert_catalog(
        &self,
        graph_name: &str,
        element_type: &str,
        element_name: &str,
        properties: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let insert_query = format!(
            "INSERT INTO graph_catalog (graph_name, element_type, element_name, properties) VALUES ('{}', '{}', '{}', '{}')",
            graph_name, element_type, element_name, properties
        );

        self.execute_ddl(&insert_query).await
    }

    async fn query_catalog(&self) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>> {
        self.execute_query_json("SELECT * FROM graph_catalog").await
    }

    fn get_type(&self) -> DatabaseType {
        DatabaseType::ClickHouse
    }

    fn clone_box(&self) -> Box<dyn DatabaseConnector> {
        Box::new(self.clone())
    }
}
