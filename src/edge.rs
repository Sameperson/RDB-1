use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Edge {
    id: u64,
    pub(crate) from: u64,
    pub(crate) to: u64,
    pub(crate) weight: f64,
    label: Option<String>,
    properties: HashMap<String, String>,
}

impl Edge {
    pub fn new(id: u64, from: u64, to: u64, weight: f64, label: Option<String>) -> Self {
        Edge {
            id,
            from,
            to,
            weight,
            label,
            properties: HashMap::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn from(&self) -> u64 {
        self.from
    }
    pub fn to(&self) -> u64 {
        self.to
    }
    pub fn weight(&self) -> f64 {
        self.weight
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
    fn test_edge_creation() {
        let edge = Edge::new(1, 2, 3, 1.5, Some("RelatedTo".to_string()));
        assert_eq!(edge.id(), 1);
        assert_eq!(edge.from(), 2);
        assert_eq!(edge.to(), 3);
        assert_eq!(edge.weight(), 1.5);
        assert_eq!(edge.label(), Some(&"RelatedTo".to_string()));
        assert!(edge.properties.is_empty());
    }

    #[test]
    fn test_edge_properties() {
        let mut edge = Edge::new(2, 3, 4, 2.0, None);
        assert!(edge.get_property("key").is_none());

        edge.set_property("key".to_string(), "value".to_string());
        assert_eq!(edge.get_property("key"), Some(&"value".to_string()));
    }
}
