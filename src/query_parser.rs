use std::str::FromStr;
use crate::graph::Graph;
use crate::node::Node;

const INVALID_NODE_ID: &'static str = "Invalid node ID";
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
                let id = tokens[1].parse::<u64>().map_err(|_| INVALID_NODE_ID.to_string())?;
                Ok(Query::AddNode(id, tokens[2].to_string()))
            },
            "GET" if tokens.len() == 2 => {
                let id = tokens[1].parse::<u64>().map_err(|_| INVALID_NODE_ID.to_string())?;
                Ok(Query::GetNode(id))
            },
            "DELETE" if tokens.len() == 2 => {
                let id = tokens[1].parse::<u64>().map_err(|_| INVALID_NODE_ID.to_string())?;
                Ok(Query::DeleteNode(id))
            },
            "PLEASE" if tokens.len() == 3 && tokens[1] == "DELETE" => {
                let id = tokens[2].parse::<u64>().map_err(|_| INVALID_NODE_ID.to_string())?;
                Ok(Query::PleaseDeleteNode(id))
            },
            _ => Err("Unsupported command".to_string()),
        }
    }
}

pub fn execute_query(graph: &mut Graph, query: Query) -> Result<String, String> {
    match query {
        Query::AddNode(id, label) => {
            let node = Node::new(id, Some(label));
            graph.add_node(node);
            Ok(format!("Node {} added successfully.", id))
        },
        Query::GetNode(id) => {
            if let Some(node) = graph.get_node(id) {
                // Use the getter method for label with parentheses
                Ok(format!("Node {}: {}", id, node.label().unwrap_or(&"No label".to_string())))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test_query_parsing() {
        assert!(matches!(Query::from_str("ADD 1 Node1"), Ok(Query::AddNode(1, label)) if label == "Node1"));
        assert!(matches!(Query::from_str("GET 1"), Ok(Query::GetNode(1))));
        assert!(matches!(Query::from_str("DELETE 1"), Ok(Query::DeleteNode(1))));
        assert!(matches!(Query::from_str("PLEASE DELETE 1"), Ok(Query::PleaseDeleteNode(1))));
        assert!(matches!(Query::from_str("UNKNOWN 1"), Err(_)));
    }

    #[test]
    fn test_add_node_query_execution() {
        let mut graph = Graph::new();
        let node = Node::new(1, Some("TestNode".to_string())); // Create the node correctly
        graph.add_node(node); // Add the node to the graph
        let result = execute_query(&mut graph, Query::AddNode(1, "TestNode".to_string())).unwrap();
        assert_eq!(result, "Node 1 added successfully.");
        assert!(graph.get_node(1).is_some());
    }

    #[test]
    fn test_delete_node_query_execution() {
        let mut graph = Graph::new();
        let node = Node::new(1, Some("TestNode".to_string()));
        graph.add_node(node);
        assert!(graph.get_node(1).is_some());

        let result = execute_query(&mut graph, Query::DeleteNode(1)).unwrap();
        assert_eq!(result, "Node 1 deleted.");
        assert!(graph.get_node(1).is_none());
    }

    #[test]
    fn test_please_delete_node_query_execution() {
        let mut graph = Graph::new();
        let node = Node::new(1, Some("TestNode".to_string()));
        graph.add_node(node);
        assert!(graph.get_node(1).is_some());

        let result = execute_query(&mut graph, Query::PleaseDeleteNode(1));
        assert!(result.is_ok(), "Should handle conditional deletes properly");
    }
}
