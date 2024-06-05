use crate::Result;
use geo::{coord, Coord, Geometry, LineString, Point, Polygon};
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;

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
                    geometry_type: Some("Point".to_string())
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
                    geometry_type: Some("Point".to_string())
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
            let geom = refs
                .map(|ref_id| {
                    let node = data.get(&ref_id).expect("Node not found");
                    if let Some(geom) = &node.geometry {
                        match geom {
                            Geometry::Point(p) => coord! {x: p.x(), y: p.y()},
                            _ => panic!("Node is not a point"),
                        }
                    } else {
                        panic!("Node has no geometry");
                    }
                })
                .collect::<Vec<Coord>>();
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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_point() {
        let point = to_point(1.0, 2.0);
        assert_eq!(point, Some(Geometry::Point(Point::new(1.0, 2.0))));
    }
}
