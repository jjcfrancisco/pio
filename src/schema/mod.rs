use geo::Geometry;
use std::collections::HashMap;

use crate::utils::read_yaml;
use crate::utils::Config;
use crate::{osmpbf::Osm, Result};

pub struct Pio {
    pub osm_id: i64,
    pub osm_type: String,
    pub geometry: Geometry,
    pub class: String,
}

pub struct PioCollection {
    pub objects: Vec<Pio>,
}

impl PioCollection {
    pub fn new() -> Self {
        PioCollection {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Pio) {
        self.objects.push(object);
    }
}

pub fn with_config(config: &Config, osm: &Osm) -> Option<Pio> {
    let mut pio_class = String::new();

    let allowed_geom_types = &config.geometry_types;
    let props = &osm.properties;
    match &osm.geometry_type {
        Some(geom_type) => {
            // Checks if the geometry type is in the configuration
            if allowed_geom_types.contains(&geom_type) {
                for prop in props {
                    config.class.iter().for_each(|class| {
                        if class.key == prop.key && class.values.contains(&prop.value) {
                            pio_class = class.then.clone();
                        }
                    });
                }
            }
        }
        None => {}
    };

    if pio_class.is_empty() {
        return None;
    }
    return Some(Pio {
        osm_id: osm.id,
        osm_type: osm.osm_type.clone(),
        geometry: osm.geometry.clone().unwrap(),
        class: pio_class,
    });
}

pub fn apply(yaml_file: &str, data: HashMap<i64, Osm>) -> Result<PioCollection> {
    // Read YAML configuration
    let config = read_yaml(yaml_file)?;

    let mut pio_objects = PioCollection::new();

    // Iterate over data
    for (_, osm) in data.iter() {
        // Available: osm.id, osm.osm_type, osm.properties, osm.geometry
        let pio = with_config(&config, osm);
        match pio {
            Some(pio) => {
                pio_objects.add(pio);
            }
            None => {}
        }
    }

    Ok(pio_objects)
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
