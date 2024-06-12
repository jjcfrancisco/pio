use std::{collections::HashMap, i64};

use geo::Geometry;

pub mod omt;

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pio {
    pub osm_id: i64,
    pub osm_type: String,
    pub geometry: Geometry,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PioCollection {
    pub objects: Vec<Pio>,
    pub layer: String,
}

impl PioCollection {
    pub fn new() -> Self {
        PioCollection {
            layer: "default".to_string(),
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Pio) {
        self.objects.push(object);
    }
}
