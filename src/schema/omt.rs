use crate::utils::config::{Config, read_yaml};
use crate::{osmpbf::Osm, Result};
use crate::osmpbf::OsmCollection;
use crate::schema::{Pio, PioCollection};

fn to_omt(config: &Config, osm: &Osm) -> Option<Pio> {
    let mut pio_class = String::new();

    let allowed_geom_types = &config.geometry_types;
    let props = &osm.properties;
    match &osm.geometry_type {
        Some(geom_type) => {
            // Checks if the geometry type is in the configuration
            if allowed_geom_types.contains(&geom_type) {
                for prop in props {
                    config.class.iter().for_each(|class| {
                        if class.and.is_some() {
                            let and = class.and.as_ref().unwrap();
                            if class.key == prop
                                .key
                                && class.values.contains(&prop.value)
                                && and.iter().all(|a| {
                                    props.iter().any(|p| a.key == p.key && a.values.contains(&p.value))
                                })
                            {
                                pio_class = class.then.clone();
                            }
                        } else if class.key == prop.key && class.values.contains(&prop.value) {
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

pub fn apply(yaml_file: &str, data: OsmCollection) -> Result<PioCollection> {
    // Read YAML configuration
    let config = read_yaml(yaml_file)?;

    let mut pc = PioCollection::new();
    pc.layer = config.layer.clone();

    // Iterate over data
    for (_, osm) in data.objects.iter() {
        // Available: osm.id, osm.osm_type, osm.properties, osm.geometry
        let pio = to_omt(&config, osm);
        match pio {
            Some(pio) => {
                pc.add(pio);
            }
            None => {}
        }
    }

    Ok(pc)
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

