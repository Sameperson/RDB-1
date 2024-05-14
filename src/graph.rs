use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::edge::Edge;
use crate::node::Node;

#[derive(Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<u64, Node>,
    pub edges: HashMap<u64, Edge>,
    pub adjacency_list: HashMap<u64, Vec<u64>>,
    pub core_nodes: HashSet<u64>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            adjacency_list: HashMap::new(),
            core_nodes: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id(), node);
    }

    pub fn add_edge(&mut self, from: u64, to: u64, weight: f64, label: Option<String>) {
        let edge_id = self.generate_edge_id();
        let edge = Edge::new(edge_id, from, to, weight, label);
        self.edges.insert(edge_id, edge);
        self.adjacency_list.entry(from).or_insert_with(Vec::new).push(edge_id);
    }

    fn generate_edge_id(&self) -> u64 {
        self.edges.len() as u64 + 1
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

    pub fn add_core_node(&mut self, node_id: u64) -> Result<(), String> {
        if self.nodes.contains_key(&node_id) {
            self.core_nodes.insert(node_id);
            Ok(())
        } else {
            Err("Node does not exist".to_string())
        }
    }

    pub fn is_core_node(&self, node_id: u64) -> bool {
        self.core_nodes.contains(&node_id)
    }

    pub fn remove_core_node(&mut self, node_id: u64) -> Result<(), String> {
        if self.core_nodes.contains(&node_id) {
            self.core_nodes.remove(&node_id);
            Ok(())
        } else {
            Err("Node is not a core node".to_string())
        }
    }

    pub fn dijkstra(&self, start_node: u64) -> HashMap<u64, f64> {
        let mut distances = HashMap::new();
        let mut priority_queue = BinaryHeap::new();

        // Initialize distances to INFINITY, except for the start node
        for &node in self.nodes.keys() {
            distances.insert(node, f64::INFINITY);
        }
        distances.insert(start_node, 0.0);  // Start node has a distance of 0
        priority_queue.push((FloatOrd(0.0), start_node));  // Push the start node onto the priority queue

        while let Some((FloatOrd(cost), u)) = priority_queue.pop() {
            if cost > distances[&u] {
                continue;  // Skip this node if we've already found a cheaper path
            }
            if let Some(edges) = self.adjacency_list.get(&u) {
                for &edge_id in edges {
                    let edge = &self.edges[&edge_id];
                    let next = edge.to;
                    let next_cost = cost + edge.weight();
                    if next_cost < distances[&next] {
                        distances.insert(next, next_cost);  // Update the shortest path to this node
                        priority_queue.push((FloatOrd(-next_cost), next));  // Push the next node to the priority queue
                    }
                }
            }
        }

        distances  // Return the map of shortest distances from the start node to all other nodes
    }

}

#[derive(PartialEq, PartialOrd)]
struct FloatOrd(f64);

impl Eq for FloatOrd {}

impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
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
        graph.add_node(Node::new(1, Some("Label".to_string())));
        graph.add_node(Node::new(2, Some("Label".to_string())));

        // Adjust the parameters to match the add_edge signature
        graph.add_edge(1, 2, 1.0, Some("RelatedTo".to_string()));

        // Assuming you want to test if the edge was added correctly
        assert!(graph.get_edge(1).is_some());
        assert_eq!(graph.adjacency_list.get(&1).unwrap().len(), 1);
    }
}
