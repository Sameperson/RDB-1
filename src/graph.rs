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
