pub mod node;
pub mod edge;
pub mod graph;
pub mod storage;
mod query_parser;

use graph::Graph;
use std::str::FromStr;

pub enum Query {
    PleaseDelete(u64),  // Holds the node ID for deletion
}

impl FromStr for Query {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.trim().split_whitespace().collect();
        if tokens.len() != 3 {
            return Err("Invalid command format".to_string());
        }

        match (tokens[0], tokens[1], tokens[2]) {
            ("PLEASE", "DELETE", node_id) => {
                let id = node_id.parse::<u64>()
                    .map_err(|_| "Invalid node ID".to_string())?;
                Ok(Query::PleaseDelete(id))
            },
            _ => Err("Unsupported command".to_string()),
        }
    }
}

pub fn execute_query(graph: &mut Graph, query: Query) -> Result<String, String> {
    match query {
        Query::PleaseDelete(node_id) => {
            graph.delete_node_please(node_id)
                .map(|_| format!("Node {} deleted successfully.", node_id))
                .map_err(|e| format!("Failed to delete node {}: {}", node_id, e))
        }
    }
}
