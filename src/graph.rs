use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::node::Node;
use crate::edge::Edge;

#[derive(Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<u64, Node>,

    pub edges: HashMap<u64, Edge>,

    pub adjacency_list: HashMap<u64, Vec<u64>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id(), node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        let from = edge.from();
        let to = edge.to();

        self.adjacency_list.entry(from).or_insert_with(Vec::new).push(edge.id());

        self.adjacency_list.entry(to).or_insert_with(Vec::new).push(edge.id());

        self.edges.insert(edge.id(), edge);
    }

    pub fn get_node(&self, id: u64) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_edge(&self, id: u64) -> Option<&Edge> {
        self.edges.get(&id)
    }

    pub fn delete_node(&mut self, id: u64) {
        if self.nodes.contains_key(&id) {
            self.nodes.remove(&id);
            self.adjacency_list.remove(&id);
            self.edges.retain(|_, e| e.from != id && e.to != id);
            self.adjacency_list.values_mut().for_each(|edges| {
                edges.retain(|&e_id| {
                    let e = self.edges.get(&e_id).unwrap();
                    e.from != id && e.to != id
                });
            });
        }
    }

    pub fn delete_node_please(&mut self, id: u64) -> Result<(), String> {
        if self.can_safely_delete(id) {
            self.delete_node(id);
            Ok(())
        } else {
            Err(format!("Node {} cannot be safely deleted.", id))
        }
    }

    // Helper function to determine if a node can be safely deleted.
    // TODO: Implement this function in more detail
    fn can_safely_delete(&self, id: u64) -> bool {
        !self.adjacency_list.get(&id).map_or(false, |edges| !edges.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_retrieve_node() {
        let mut graph = Graph::new();
        let node = Node::new(1, Some("Label".to_string()));
        graph.add_node(node);

        assert!(graph.get_node(1).is_some());
    }

    #[test]
    fn test_add_and_retrieve_edge() {
        let mut graph = Graph::new();
        let node1 = Node::new(1, Some("Label".to_string()));
        let node2 = Node::new(2, Some("Label".to_string()));
        graph.add_node(node1);
        graph.add_node(node2);

        let edge = Edge::new(1, 1, 2, Some("RelatedTo".to_string()));
        graph.add_edge(edge);

        assert!(graph.get_edge(1).is_some());
        assert_eq!(graph.adjacency_list.get(&1).unwrap().len(), 1);
    }
}
