use crate::Result;
use geo::{coord, Coord, CoordsIter, Geometry, LineString, Point, Polygon};
use osmpbf::{Element, ElementReader};
use polars::prelude::*;
use std::collections::HashMap;
use wkt::ToWkt;

#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub value: String,
}

pub struct Osm {
    pub id: i64,
    pub osm_type: String,
    pub properties: Vec<Property>,
    pub geometry: Option<Geometry>,
    pub geometry_type: Option<String>,
}

// Create a dataframe from the osmpbf data
#[allow(dead_code)]
pub fn read_nodes_from_osmpbf_polars(path: &str) -> Result<DataFrame> {
    let reader = ElementReader::from_path(path)?;
    let mut ids = Vec::new();
    let mut geometries = Vec::new();
    let mut geometry_types = Vec::new();
    let mut properties: Vec<Property> = Vec::new();
    reader.for_each(|element| match element {
        Element::Node(n) => {
            ids.push(n.id());
            geometries.push(Point::new(n.lat(), n.lon()).wkt_string());
            geometry_types.push("Point");
            for (key, value) in n.tags() {
                properties.push(Property {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            }
        }
        Element::DenseNode(d) => {
            ids.push(d.id());
            geometries.push(Point::new(d.lat(), d.lon()).wkt_string());
            geometry_types.push("Point");
            for (key, value) in d.tags() {
                properties.push(Property {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            }
        }
        Element::Way(_) => {}
        Element::Relation(_) => {}
    })?;

    // Create a dataframe
    let mut df = DataFrame::new(vec![
        Series::new("id", ids),
        Series::new("osm_type", vec!["node"]),
        Series::new("geometry", geometries),
        Series::new("geometry_type", geometry_types),
    ])?;

    for prop in properties {
        let s = Series::new(&prop.key, vec![prop.value]);
        df.with_column(s)?;
    }

    println!("DataFrame: {:?}", df);

    Ok(df)
}

pub fn read_nodes_from_osmpbf(path: &str) -> Result<HashMap<i64, Osm>> {
    let reader = ElementReader::from_path(path)?;
    let mut nodes: HashMap<i64, Osm> = HashMap::new();
    reader.for_each(|element| match element {
        Element::Node(n) => {
            nodes.insert(
                n.id(),
                Osm {
                    id: n.id(),
                    osm_type: "node".to_string(),
                    properties: {
                        let mut properties = Vec::new();
                        for (key, value) in n.tags() {
                            properties.push(Property {
                                key: key.to_string(),
                                value: value.to_string(),
                            });
                        }
                        properties
                    },
                    geometry: to_point(n.lat(), n.lon()),
                    geometry_type: Some("Point".to_string()),
                },
            );
        }
        Element::DenseNode(d) => {
            nodes.insert(
                d.id(),
                Osm {
                    id: d.id(),
                    osm_type: "node".to_string(),
                    properties: {
                        let mut properties = Vec::new();
                        for (key, value) in d.tags() {
                            properties.push(Property {
                                key: key.to_string(),
                                value: value.to_string(),
                            });
                        }
                        properties
                    },
                    geometry: to_point(d.lon(), d.lat()),
                    geometry_type: Some("Point".to_string()),
                },
            );
        }
        // Empty match arm to satisfy the compiler
        Element::Way(_) => (),
        Element::Relation(_) => (),
    })?;

    return Ok(nodes);
}

pub fn process_lines_and_polygons(
    path: &str,
    data: HashMap<i64, Osm>,
) -> Result<HashMap<i64, Osm>> {
    let mut data = data;
    let reader = ElementReader::from_path(path)?;
    reader.for_each(|element| match element {
        Element::Node(_) => (),
        Element::DenseNode(_) => (),
        Element::Way(w) => {
            let mut osm = Osm {
                id: w.id(),
                osm_type: "way".to_string(),
                properties: {
                    let mut properties = Vec::new();
                    for (key, value) in w.tags() {
                        properties.push(Property {
                            key: key.to_string(),
                            value: value.to_string(),
                        });
                    }
                    properties
                },
                geometry: None,
                geometry_type: None,
            };
            let refs = w.refs();
            let mut geom: Vec<Coord> = Vec::new();
            for ref_id in refs {
                let node = data.get(&ref_id);
                if node.is_some() {
                    let node = node.unwrap();
                    match &node.geometry {
                        Some(Geometry::Point(p)) => {
                            geom.push(coord! {x: p.x(), y: p.y()})
                        }
                        Some(Geometry::LineString(ls)) => {
                            geom.extend(ls.coords_iter().map(|c| c).collect::<Vec<Coord>>())
                        }
                        _ => (),
                    }
                }
            }
            if geom.get(0) == geom.last() {
                osm.geometry_type = Some("Polygon".to_string());
                osm.geometry = Some(Geometry::Polygon(Polygon::new(LineString(geom), vec![])))
            } else {
                osm.geometry_type = Some("LineString".to_string());
                osm.geometry = Some(Geometry::LineString(LineString(geom)))
            };
            data.insert(w.id(), osm);
        }
        Element::Relation(_) => (),
    })?;

    Ok(data)
}

fn to_point(lat: f64, lon: f64) -> Option<Geometry> {
    Some(Geometry::Point(Point::new(lat, lon)))
}

#[allow(dead_code)]
fn to_polars(data: HashMap<i64, Osm>) -> Result<DataFrame> {
    let mut ids = Vec::new();
    let mut geometries: Vec<String> = Vec::new();
    let mut geometry_types = Vec::new();
    let mut properties: Vec<Property> = Vec::new();
    let mut osm_types = Vec::new();

    for (_, osm) in data.iter() {
        ids.push(osm.id);
        osm_types.push(osm.osm_type.clone());
        let geometry = &osm.geometry;
        match geometry {
            Some(Geometry::Point(p)) => {
                geometries.push(p.wkt_string());
                geometry_types.push("Point");
            }
            Some(Geometry::LineString(ls)) => {
                geometries.push(ls.wkt_string());
                geometry_types.push("LineString");
            }
            Some(Geometry::Polygon(p)) => {
                geometries.push(p.wkt_string());
                geometry_types.push("Polygon");
            }
            _ => panic!("Geometry not supported"),
        };
        for prop in &osm.properties {
            properties.push(Property {
                key: prop.key.clone(),
                value: prop.value.clone(),
            });
        }
    }

    // Create a dataframe
    let mut df = DataFrame::new(vec![
        Series::new("id", ids),
        Series::new("osm_type", vec!["node"]),
        Series::new("geometry", geometries),
        Series::new("geometry_type", geometry_types),
    ])?;

    for prop in properties {
        let s = Series::new(&prop.key, vec![prop.value]);
        df.with_column(s)?;
    }

    Ok(df)
}

// Unit tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
// }
