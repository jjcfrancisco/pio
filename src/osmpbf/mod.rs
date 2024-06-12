use crate::utils::to_point;
use crate::Result;
use geo::{coord, Coord, CoordsIter, Geometry, LineString, Polygon};
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Osm {
    pub id: i64,
    pub osm_type: String,
    pub properties: HashMap<String, Property>,
    pub geometry: Option<Geometry>,
    pub geometry_type: Option<String>,
}

pub struct OsmCollection {
    pub objects: HashMap<i64, Osm>,
}

impl Osm {
    // Get a value from a key from the properties
    pub fn get(&self, key: &str) -> Option<String> {
        let prop = self.properties.get(key);
        match prop {
            Some(prop) => Some(prop.value.clone()),
            None => None,
        }
    }
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
                    let mut properties = HashMap::new();
                    for (key, value) in n.tags() {
                        properties.insert(
                            key.to_string(),
                            Property {
                                key: key.to_string(),
                                value: value.to_string(),
                            },
                        );
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
                    let mut properties = HashMap::new();
                    for (key, value) in d.tags() {
                        properties.insert(
                            key.to_string(),
                            Property {
                                key: key.to_string(),
                                value: value.to_string(),
                            },
                        );
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
                    let mut properties = HashMap::new();
                    for (key, value) in w.tags() {
                        properties.insert(
                            key.to_string(),
                            Property {
                                key: key.to_string(),
                                value: value.to_string(),
                            },
                        );
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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_osmpbf() {
        let osm =
            read_osmpbf("tests/melilla-latest.osm.pbf").expect("Failed to read test OSM PBF file");
        assert_eq!(osm.objects.len(), 79114);
    }

    #[test]
    fn test_process_lines_and_polygons() {
        let nodes = read_nodes_from_osmpbf("tests/melilla-latest.osm.pbf")
            .expect("Failed to read nodes from OSM PBF file");
        let nodes_ways_relations =
            process_lines_and_polygons("tests/melilla-latest.osm.pbf", nodes)
                .expect("Failed to process lines and polygons");
        // Assert per type
        let mut points = 0;
        let mut lines = 0;
        let mut polygons = 0;
        for (_, osm) in nodes_ways_relations.objects.iter() {
            match osm.geometry_type.as_ref().unwrap().as_str() {
                "Point" => points += 1,
                "LineString" => lines += 1,
                "Polygon" => polygons += 1,
                _ => (),
            }
        }
        assert_eq!(points, 68673);
        assert_eq!(lines, 4614);
        assert_eq!(polygons, 5827);
    }
}
