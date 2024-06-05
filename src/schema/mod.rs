use geo::Geometry;
use serde_yaml::Value;
use std::collections::HashMap;

use crate::utils::read_yaml;
use crate::{osmpbf::Osm, Result};

pub fn is_geometry_type(key: &Value, value: &Value, osm: &Osm) -> bool {
    let k_str = key.as_str().unwrap();
    if k_str == "geometry_types" {
        let v_seq = value.as_sequence().unwrap();
        // Check if geometry type in Osm is in values
        if osm.geometry_type.is_some() {
            let geom_type = osm.geometry_type.as_ref().unwrap();
            if v_seq.iter().any(|x| x.as_str().unwrap() == geom_type) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
}

#[derive(Default)]
pub struct And {
    pub key: String,
    pub values: Vec<String>,
}

// Struct of key, value and then
#[derive(Default)]
pub struct Class {
    pub key: String,
    pub values: Vec<String>,
    pub then: String,
    pub and: And,
}

pub fn apply_schema(config: &Value, osm: &Osm) -> Result<()> {
    // Go over the configuration
    let mut class: Class = Default::default();
    let config_map = config.as_mapping();
    if config_map.is_some() {
        let config_map = config_map.unwrap();
        // println!("\n\nConfiguration found: {:?}\n\n", config_map);
        config_map.iter().for_each(|(k, v)| {
            if k.is_string() {
                // If osm geometry type is in YAML allowed geometry types
                if k.as_str().unwrap() == "geometry_types" && is_geometry_type(k, v, osm) {
                    // Geometry allowed
                }
                if k.as_str().unwrap() == "class" && v.is_sequence() {
                    let class_map = v.as_sequence().unwrap();
                    class_map.iter().for_each(|map_parent_key| {
                        if map_parent_key.is_mapping() {
                            let class_map = map_parent_key.as_mapping().unwrap();
                            class_map.iter().for_each(|(map_child_key, map_child_value)| {
                                if map_child_key.is_string() {
                                    let key = map_child_key.as_str().unwrap();
                                    if key == "key" {
                                        class.key = map_child_value.as_str().unwrap().to_string();
                                    } else if key == "values" {
                                        let values = map_child_value.as_sequence().unwrap();
                                        values.iter().for_each(|value| {
                                            class.values.push(value.as_str().unwrap().to_string());
                                        });
                                    } else if key == "then" {
                                        class.then = map_child_value.as_str().unwrap().to_string();
                                    } else if key == "and" {
                                        let and_parent = map_child_value.as_sequence().unwrap();
                                        and_parent.iter().for_each(|and_child| {
                                            if and_child.is_mapping() {
                                                let and_map = and_child.as_mapping().unwrap();
                                                and_map.iter().for_each(|(and_key, and_value)| {
                                                    class.and.key = and_key.as_str().unwrap().to_string();
                                                    if and_value.is_sequence() {
                                                        let values = and_value.as_sequence().unwrap();
                                                        values.iter().for_each(|value| {
                                                            class.and.values.push(value.as_str().unwrap().to_string());
                                                        });
                                                    }
                                                });
                                            }
                                        });
                                    }
                                }
                            });
                        }
                    });
                }
            }
        });
    } else {
        println!("No mapping found");
    }

    if osm.properties.iter().any(|x| x.key == class.key) {
        let property = osm.properties.iter().find(|x| x.key == class.key).unwrap();
        if class.values.iter().any(|x| x == &property.value) {
            if class.and.values.len() > 0 {
                if class.and.values.iter().any(|x| x == &class.and.key) {
                    // Also AND
                }
            } else {
                // Without AND
            }
        }
    }

    Ok(())
}

// Construct a struct that allows to save final results from apply_schema 
pub struct Pio {
    pub id: i64,
    pub osm_type: String,

}

pub fn process_data(yaml_file: &str, data: HashMap<i64, Osm>) -> Result<()> {
    // Read YAML configuration
    let config = read_yaml(yaml_file)?;

    // Iterate over data
    for (_, osm) in data.iter() {
        // Available: osm.id, osm.osm_type, osm.properties, osm.geometry
        let pio = apply_schema(&config, osm)?;
    }

    Ok(())
}

// Unit tests
#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    //
    // let yaml = "
    //     service:
    //       key: highway
    //       values: service
    // ";

    // fn test_to_point() {
    //     let point = to_point(1.0, 2.0);
    //     assert_eq!(point, Some(Geometry::Point(Point::new(1.0, 2.0))));
    // }
}
