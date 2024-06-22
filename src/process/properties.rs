use crate::Result;
use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;

use osmpbf::{Node, Way, Relation, DenseNode};

pub struct Properties {
    pub jsonb: Value
}

impl Properties {
    pub fn new() -> Self {
        let data = json!({
        });
        Self {
            jsonb: data
        } 
    }
    pub fn add(&mut self, key: &str, value: &str) {
        if let Value::Object(ref mut map) = self.jsonb {
            map.insert(key.to_string(), json!(value));
        }
    }
}

pub enum OsmType<'a> {
    Node(&'a Node<'a>),
    DenseNode(&'a DenseNode<'a>),
    Way(&'a Way<'a>),
    Relation(&'a Relation<'a>),
}


pub fn sort_tags(element: OsmType) -> Result<Properties> {

    let tags = match element {
        OsmType::Node(node) => {
            for (key, value) in node.tags() {
                println!("{}: {}", key, value);
            } 
        },
        OsmType::DenseNode(dense_node) => {
            for (key, value) in dense_node.tags() {
                println!("{}: {}", key, value);
            }
        },
        OsmType::Way(way) => {
            for (key, value) in way.tags() {
                println!("{}: {}", key, value);
            }
        },
        OsmType::Relation(relation) => {
            for (key, value) in relation.tags() {
                println!("{}: {}", key, value);
            }
        },
    };

    // Get tags into a hashmap
    let mut properties = Properties::new();
    Ok(properties)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that Properties::new and Properties::add
    #[test]
    fn test_properties() {
        let mut properties = Properties::new();
        properties.add("email", "hello@world.com");
        properties.add("is_student", "true");
        assert_eq!(properties.jsonb, json!({"email": "hello@world.com", "is_student": "true"}));
    }

}
