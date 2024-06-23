use crate::Result;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

use osmpbf::{DenseNode, Node, Relation, Way};

use crate::utils::config::{Config, Kvat};
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
        if let Ok((key, value)) = sort_class(&config, &tags) {
            println!("key: {}, value: {}", key, value);
            properties.add(key.as_str(), FieldType::Text(value));
        }
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
) -> Result<(String, String)> {
    // Iterates over all yaml files
    for class in config.class.iter() {
        // Iterates over all kvat (key, values, and, then)
        // in the 'class' object
        for kvat in class.iter() {
            // Get the key matching the kvat key
            if let Some(values) = tags.get(kvat.key.as_str()) {
                // Check if the value is in the kvat values
                if kvat.values.contains(&values.to_string()) {
                    // Also checks if there is an 'and' object
                    if let Some(and) = &kvat.and {
                        // If there is an 'and' objects associated to the kvat
                        // then iterate over all kvs (key, values) within the 'and' object
                        for kvs in and.iter() {
                            // Check if the and.key is in the tags
                            if let Some(value) = tags.get(kvs.key.as_str()) {
                                // Check if the value is in the kvs values
                                if kvs.values.contains(&value.to_string()) {
                                    // If the value is in the kvs values
                                    return Ok(("class".into(), kvat.then.clone()));
                                }
                            }
                        }
                    } else {
                        // If there is no 'and' object associated to the kvat
                        return Ok(("class".into(), kvat.then.clone()));
                    }
                }
            }
        }
    }

    Err("Failed to sort class".into())
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

    // Test that sort_class sorts the class based on yaml
    #[test]
    fn test_sort_class() {
        let mut tags: HashMap<&str, &str> = HashMap::new();
        tags.insert("key", "value");

        let kvat = Kvat {
            key: "key".to_string(),
            values: vec!["value".to_string()],
            and: None,
            then: "class".to_string(),
        };

        let class = vec![kvat];

        let config = Config {
            schema: "omt".to_string(),
            layer: "test".to_string(),
            geometry_types: vec![
                "Point".to_string(),
                "LineString".to_string(),
                "Polygon".to_string(),
            ],
            fields: vec![],
            class: Some(class),
        };

        let result = sort_class(&config, &tags).unwrap();
        assert_eq!(result, ("class".to_string(), "class".to_string()));

    }
}
