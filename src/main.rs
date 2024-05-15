mod node;
mod edge;
mod query_parser;
mod graph;

use graph::Graph;
use crate::node::Node;
use crate::query_parser::{execute_query, Query};

fn main() {
    // Assuming Graph, Node, and add_node are implemented as described
    let mut graph = Graph::new();

    // Add nodes
    graph.add_node(Node::new(1, Some("Node 1".to_string())));
    graph.add_node(Node::new(2, Some("Node 2".to_string())));
    graph.add_node(Node::new(3, Some("Node 3".to_string())));
    graph.add_node(Node::new(4, Some("Node 4".to_string())));
    graph.add_node(Node::new(5, Some("Node 5".to_string())));

    // Add edges with weights
    // Connect node 1 to nodes 2 and 3
    graph.add_edge(1, 2, 6.0, Some("Edge 1-2".to_string())); // weight 6
    graph.add_edge(1, 3, 1.0, Some("Edge 1-3".to_string())); // weight 1

    // Connect node 2 to node 3 and node 4
    graph.add_edge(2, 3, 2.0, Some("Edge 2-3".to_string())); // weight 2
    graph.add_edge(2, 4, 2.0, Some("Edge 2-4".to_string())); // weight 2

    // Connect node 3 to node 4 and node 5
    graph.add_edge(3, 4, 1.0, Some("Edge 3-4".to_string())); // weight 1
    graph.add_edge(3, 5, 4.0, Some("Edge 3-5".to_string())); // weight 4

    // Connect node 4 to node 5
    graph.add_edge(4, 5, 2.0, Some("Edge 4-5".to_string())); // weight 2

    // Running Dijkstra's algorithm from node 1
    let distances = graph.dijkstra(1);

    // Print shortest paths from node 1 to all other nodes
    for (node, distance) in distances {
        println!("Shortest path from Node 1 to Node {}: {}", node, distance);
    }

    graph.write_dot("graph.dot").expect("Failed to write DOT file");
}
