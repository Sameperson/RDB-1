use crate::graph::Graph;
use std::{fs, io};

pub struct Storage {
    path: String,
}

impl Storage {
    pub fn new(path: String) -> Self {
        Storage { path }
    }

    pub fn save_graph(&self, graph: &Graph) -> io::Result<()> {
        let json = serde_json::to_string_pretty(graph)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn load_graph(&self) -> io::Result<Graph> {
        let json = fs::read_to_string(&self.path)?;
        let graph = serde_json::from_str(&json)?;
        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;
    use crate::graph::Graph;
    use tempfile::NamedTempFile;

    #[test]
    fn test_save_and_load_graph() {
        let mut graph = Graph::new();
        graph.add_node(Node::new(1, Some("Node1".to_string()))); // Correctly adding nodes
        graph.add_node(Node::new(2, Some("Node2".to_string())));

        // Adding edge directly with the parameters expected by `add_edge`
        graph.add_edge(1, 2, 1.0, Some("connects".to_string())); // assuming weight is required as per your setup

        let mut temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();

        let storage = Storage::new(file_path.clone());

        assert!(storage.save_graph(&graph).is_ok());

        let loaded_graph = storage.load_graph();
        assert!(loaded_graph.is_ok());
        let loaded_graph = loaded_graph.unwrap();

        assert_eq!(loaded_graph.nodes.len(), graph.nodes.len());
        assert_eq!(loaded_graph.edges.len(), graph.edges.len());
    }
}
