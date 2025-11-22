use async_trait::async_trait;
use serde_json::Value;
use sqlx::{Column, MySql, MySqlPool, Pool, Row};
use std::error::Error;

use super::{DatabaseConnector, DatabaseType};

#[derive(Clone)]
pub struct VeloDBConnector {
    pool: Pool<MySql>,
}

impl VeloDBConnector {
    pub async fn new(connection_string: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let pool = MySqlPool::connect(connection_string)
            .await
            .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })?;

        Ok(Self { pool })
    }

    pub fn from_pool(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabaseConnector for VeloDBConnector {
    async fn execute_query(
        &self,
        query: &str,
        format: &str,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })?;

        let mut result = Vec::new();

        if format == "Pretty" || format == "PrettyCompact" {
            // Pretty print format
            if !rows.is_empty() {
                // Get column names
                let columns = rows[0].columns();
                let col_names: Vec<&str> = columns.iter().map(|c| c.name()).collect();

                // Create header
                let header = col_names.join(" | ");
                let header_len = header.len();
                result.push(header);
                result.push("-".repeat(header_len));

                // Add rows
                for row in &rows {
                    let mut row_values = Vec::new();
                    for (i, _col) in columns.iter().enumerate() {
                        let value: Option<String> = row.try_get(i).ok();
                        row_values.push(value.unwrap_or_else(|| "NULL".to_string()));
                    }
                    result.push(row_values.join(" | "));
                }
            }
        } else if format == "Csv" || format == "CSVWithNames" {
            // CSV format
            if !rows.is_empty() {
                let columns = rows[0].columns();

                if format == "CSVWithNames" {
                    let col_names: Vec<&str> = columns.iter().map(|c| c.name()).collect();
                    result.push(col_names.join(","));
                }

                for row in &rows {
                    let mut row_values = Vec::new();
                    for (i, _col) in columns.iter().enumerate() {
                        let value: Option<String> = row.try_get(i).ok();
                        row_values.push(value.unwrap_or_else(|| "".to_string()));
                    }
                    result.push(row_values.join(","));
                }
            }
        } else {
            // JSONEachRow format (default)
            for row in &rows {
                let columns = row.columns();
                let mut json_obj = serde_json::Map::new();

                for col in columns {
                    let col_name = col.name();
                    let value: Option<String> = row.try_get(col_name).ok();

                    if let Some(v) = value {
                        json_obj.insert(col_name.to_string(), Value::String(v));
                    } else {
                        json_obj.insert(col_name.to_string(), Value::Null);
                    }
                }

                result.push(serde_json::to_string(&json_obj).unwrap());
            }
        }

        Ok(result)
    }

    async fn execute_query_json(
        &self,
        query: &str,
    ) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })?;

        let mut result = Vec::new();

        for row in &rows {
            let columns = row.columns();
            let mut json_obj = serde_json::Map::new();

            for col in columns {
                let col_name = col.name();

                // Try different types
                if let Ok(v) = row.try_get::<String, _>(col_name) {
                    json_obj.insert(col_name.to_string(), Value::String(v));
                } else if let Ok(v) = row.try_get::<i64, _>(col_name) {
                    json_obj.insert(col_name.to_string(), Value::Number(v.into()));
                } else if let Ok(v) = row.try_get::<i32, _>(col_name) {
                    json_obj.insert(col_name.to_string(), Value::Number(v.into()));
                } else if let Ok(v) = row.try_get::<f64, _>(col_name) {
                    if let Some(num) = serde_json::Number::from_f64(v) {
                        json_obj.insert(col_name.to_string(), Value::Number(num));
                    }
                } else if let Ok(v) = row.try_get::<bool, _>(col_name) {
                    json_obj.insert(col_name.to_string(), Value::Bool(v));
                } else {
                    json_obj.insert(col_name.to_string(), Value::Null);
                }
            }

            result.push(Value::Object(json_obj));
        }

        Ok(result)
    }

    async fn execute_ddl(&self, query: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query(query)
            .execute(&self.pool)
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
        DatabaseType::VeloDB
    }

    fn clone_box(&self) -> Box<dyn DatabaseConnector> {
        Box::new(self.clone())
    }
}
