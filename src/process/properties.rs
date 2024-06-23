use crate::Result;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

use osmpbf::{DenseNode, Node, Relation, Way};

use crate::utils::config::Config;
use crate::utils::validate::{try_mapping, FieldType};

pub struct Properties {
    pub jsonb: Value,
}

impl Properties {
    pub fn new() -> Self {
        let data = json!({});
        Self { jsonb: data }
    }
    pub fn add(&mut self, key: &str, value: FieldType) {
        self.jsonb[key] = match value {
            FieldType::Integer(value) => json!(value),
            FieldType::Text(value) => json!(value),
            FieldType::Float(value) => json!(value),
            FieldType::Boolean(value) => json!(value),
        };
    }
}

#[warn(dead_code)]
pub enum OsmType<'a> {
    Node(&'a Node<'a>),
    DenseNode(&'a DenseNode<'a>),
    Way(&'a Way<'a>),
    Relation(&'a Relation<'a>),
}

pub fn sort_tags<'a>(element: OsmType, configs: &'a Vec<Config>) -> Value {
    let mut tags: HashMap<&str, &str> = HashMap::new();

    match element {
        OsmType::Node(node) => {
            for (key, value) in node.tags() {
                tags.insert(key, value);
            }
        }
        OsmType::DenseNode(dense_node) => {
            for (key, value) in dense_node.tags() {
                tags.insert(key, value);
            }
        }
        OsmType::Way(way) => {
            for (key, value) in way.tags() {
                tags.insert(key, value);
            }
        }
        OsmType::Relation(relation) => {
            for (key, value) in relation.tags() {
                tags.insert(key, value);
            }
        }
    };

    let properties = sort_fields(tags, configs);
    serde_json::to_value(properties.jsonb).expect("Failed to convert properties to json")
}

fn sort_fields<'a>(tags: HashMap<&str, &str>, configs: &'a Vec<Config>) -> Properties {
    let mut properties = Properties::new();
    // Iterate over all configs
    for config in configs.iter() {
        // Sort class based on yaml
        // if let Ok((key, value)) = sort_class(&config, &tags) {
        //     properties.add(key, value);
        // }
        // Sort other fields based on yaml
        for field in config.fields.iter() {
            if tags.get(field.name.as_str()).is_some() {
                if let Some(rename_to) = &field.rename_to {
                    // Cast to field_type
                    properties.add(rename_to, {
                        try_mapping(
                            tags.get(field.name.as_str()).expect("Failed to get field"),
                            &field.mapping,
                        )
                    });
                } else {
                    // Cast to field_type
                    properties.add(field.name.as_str(), {
                        try_mapping(
                            tags.get(field.name.as_str()).expect("Failed to get field"),
                            &field.mapping,
                        )
                    });
                }
            }
        }
    }
    properties
}

fn sort_class<'a>(
    config: &'a Config,
    tags: &'a HashMap<&'a str, &'a str>,
) -> Result<(&'a str, &'a str)> {
    Ok(("key", "value"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that Properties::new and Properties::add
    #[test]
    fn test_properties() {
        let mut properties = Properties::new();
        properties.add("key", FieldType::Text("value".to_string()));
        assert_eq!(properties.jsonb["key"], json!("value"));
    }
}
