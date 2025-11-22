// PageRank algorithm implementation for graph databases
//
// PageRank is an algorithm that measures the importance of nodes in a graph
// based on the structure of incoming links.

/// Configuration for PageRank algorithm
#[derive(Debug, Clone)]
pub struct PageRankConfig {
    /// Damping factor (typically 0.85)
    pub damping_factor: f64,
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Convergence tolerance
    pub tolerance: f64,
}

impl Default for PageRankConfig {
    fn default() -> Self {
        PageRankConfig {
            damping_factor: 0.85,
            max_iterations: 20,
            tolerance: 0.0001,
        }
    }
}

impl PageRankConfig {
    /// Create a new PageRank configuration
    pub fn new(damping_factor: f64, max_iterations: usize, tolerance: f64) -> Self {
        PageRankConfig {
            damping_factor,
            max_iterations,
            tolerance,
        }
    }

    /// Generate ClickHouse SQL for PageRank computation
    ///
    /// This generates a SQL query that computes PageRank scores using
    /// recursive CTEs and window functions in ClickHouse.
    ///
    /// The algorithm works as follows:
    /// 1. Initialize all nodes with score 1/N
    /// 2. For each iteration:
    ///    - Each node distributes its score equally to all outgoing edges
    ///    - Each node receives the sum of incoming scores
    ///    - Apply damping factor: (1-d)/N + d * (sum of incoming scores)
    ///
    /// Parameters:
    /// - node_table: Name of the node table
    /// - edge_table: Name of the edge table
    /// - node_id_col: Column name for node ID
    /// - source_col: Column name for edge source
    /// - target_col: Column name for edge target
    pub fn generate_sql(
        &self,
        node_table: &str,
        edge_table: &str,
        node_id_col: &str,
        source_col: &str,
        target_col: &str,
    ) -> String {
        format!(
            r#"
WITH
-- Initialize: Count total nodes
node_count AS (
    SELECT count(*) as total_nodes
    FROM {node_table}
),
-- Initialize: All nodes start with score 1/N
init_scores AS (
    SELECT
        {node_id_col} as node_id,
        1.0 / (SELECT total_nodes FROM node_count) as score
    FROM {node_table}
),
-- Count outgoing edges for each node
out_degrees AS (
    SELECT
        {source_col} as node_id,
        count(*) as out_degree
    FROM {edge_table}
    GROUP BY {source_col}
),
-- PageRank iteration (simplified single iteration for MVP)
-- In production, this would need to iterate {max_iterations} times
score_distribution AS (
    SELECT
        e.{target_col} as node_id,
        sum(s.score / COALESCE(d.out_degree, 1)) as incoming_score
    FROM {edge_table} e
    JOIN init_scores s ON e.{source_col} = s.node_id
    LEFT JOIN out_degrees d ON e.{source_col} = d.node_id
    GROUP BY e.{target_col}
),
-- Apply damping factor
final_scores AS (
    SELECT
        n.{node_id_col} as node_id,
        ((1 - {damping}) / (SELECT total_nodes FROM node_count)) +
        ({damping} * COALESCE(d.incoming_score, 0)) as pagerank_score
    FROM {node_table} n
    LEFT JOIN score_distribution d ON n.{node_id_col} = d.node_id
)
SELECT * FROM final_scores
ORDER BY pagerank_score DESC
"#,
            node_table = node_table,
            edge_table = edge_table,
            node_id_col = node_id_col,
            source_col = source_col,
            target_col = target_col,
            damping = self.damping_factor,
            max_iterations = self.max_iterations,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_config_default() {
        let config = PageRankConfig::default();
        assert_eq!(config.damping_factor, 0.85);
        assert_eq!(config.max_iterations, 20);
        assert_eq!(config.tolerance, 0.0001);
    }

    #[test]
    fn test_pagerank_sql_generation() {
        let config = PageRankConfig::default();
        let sql = config.generate_sql("nodes", "edges", "id", "src", "dst");

        // Verify SQL contains expected keywords
        assert!(sql.contains("WITH"));
        assert!(sql.contains("node_count"));
        assert!(sql.contains("init_scores"));
        assert!(sql.contains("pagerank_score"));
        assert!(sql.contains("0.85")); // damping factor
    }
}
