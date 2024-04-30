use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Node {
    id: u64,
    label: Option<String>,
    properties: HashMap<String, String>,
}

impl Node {
    pub fn new(id: u64, label: Option<String>) -> Self {
        Node {
            id,
            label,
            properties: HashMap::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(1, Some("Label".to_string()));
        assert_eq!(node.id(), 1);
        assert_eq!(node.label(), Some(&"Label".to_string()));
        assert!(node.properties.is_empty());
    }

    #[test]
    fn test_node_properties() {
        let mut node = Node::new(2, None);
        assert!(node.get_property("key").is_none());

        node.set_property("key".to_string(), "value".to_string());
        assert_eq!(node.get_property("key"), Some(&"value".to_string()));
    }
}
