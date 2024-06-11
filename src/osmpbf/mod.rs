use crate::Result;
use geo::{coord, Coord, CoordsIter, Geometry, LineString, Point, Polygon};
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

pub struct OsmCollection {
    pub objects: HashMap<i64, Osm>,
}

impl OsmCollection {
    pub fn new() -> Self {
        OsmCollection {
            objects: HashMap::new(),
        }
    }

    pub fn add(&mut self, object: Osm) {
        self.objects.insert(object.id, object);
    }
}

pub fn read_osmpbf(path: &str) -> Result<OsmCollection> {
    let nodes = read_nodes_from_osmpbf(path)?;
    let nodes_ways_relations = process_lines_and_polygons(path, nodes)?;

    Ok(nodes_ways_relations)
}

fn read_nodes_from_osmpbf(path: &str) -> Result<OsmCollection> {
    let reader = ElementReader::from_path(path)?;
    let mut nodes = OsmCollection::new();
    reader.for_each(|element| match element {
        Element::Node(n) => {
            nodes.add(Osm {
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
            });
        }
        Element::DenseNode(d) => {
            nodes.add(Osm {
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
                geometry: to_point(d.lat(), d.lon()),
                geometry_type: Some("Point".to_string()),
            });
        }
        // Empty match arm to satisfy the compiler
        Element::Way(_) => (),
        Element::Relation(_) => (),
    })?;

    return Ok(nodes);
}

fn process_lines_and_polygons(path: &str, oc: OsmCollection) -> Result<OsmCollection> {
    let mut data = oc;
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
                let node = data.objects.get(&ref_id);
                if node.is_some() {
                    let node = node.unwrap();
                    match &node.geometry {
                        Some(Geometry::Point(p)) => geom.push(coord! {x: p.x(), y: p.y()}),
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
            data.add(osm);
        }
        Element::Relation(_) => (),
    })?;

    Ok(data)
}

fn to_point(lat: f64, lon: f64) -> Option<Geometry> {
    Some(Geometry::Point(Point::new(lat, lon)))
}

// Unit tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
// }
