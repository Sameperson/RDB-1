use std::str::FromStr;
use crate::graph::Graph;

pub enum Query {
    AddNode(u64, String),   // ID and Label
    GetNode(u64),           // ID
    DeleteNode(u64),        // ID
    PleaseDeleteNode(u64),  // ID
}

impl FromStr for Query {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.trim().split_whitespace().collect();
        match tokens[0] {
            "ADD" if tokens.len() == 3 => {
                let id = tokens[1].parse::<u64>().map_err(|_| "Invalid node ID".to_string())?;
                Ok(Query::AddNode(id, tokens[2].to_string()))
            },
            "GET" if tokens.len() == 2 => {
                let id = tokens[1].parse::<u64>().map_err(|_| "Invalid node ID".to_string())?;
                Ok(Query::GetNode(id))
            },
            "DELETE" if tokens.len() == 2 => {
                let id = tokens[1].parse::<u64>().map_err(|_| "Invalid node ID".to_string())?;
                Ok(Query::DeleteNode(id))
            },
            "PLEASE" if tokens.len() == 3 && tokens[1] == "DELETE" => {
                let id = tokens[2].parse::<u64>().map_err(|_| "Invalid node ID".to_string())?;
                Ok(Query::PleaseDeleteNode(id))
            },
            _ => Err("Unsupported command".to_string()),
        }
    }
}

pub fn execute_query(graph: &mut Graph, query: Query) -> Result<String, String> {
    match query {
        Query::AddNode(id, label) => {
            graph.add_node(id, label);
            Ok(format!("Node {} added successfully.", id))
        },
        Query::GetNode(id) => {
            if let Some(node) = graph.get_node(id) {
                Ok(format!("Node {}: {}", id, node.label))
            } else {
                Err(format!("Node {} not found.", id))
            }
        },
        Query::DeleteNode(id) => {
            graph.delete_node(id);
            Ok(format!("Node {} deleted.", id))
        },
        Query::PleaseDeleteNode(id) => {
            graph.delete_node_please(id)
                .map(|_| format!("Node {} deleted successfully.", id))
                .map_err(|e| format!("Failed to delete node {}: {}", id, e))
        }
    }
}
