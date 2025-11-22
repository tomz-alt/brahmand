// Graph algorithms module for Brahmand
//
// This module provides implementations of common graph algorithms
// that can be executed on graph data stored in ClickHouse.

pub mod pagerank;

pub use pagerank::PageRankConfig;
