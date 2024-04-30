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
