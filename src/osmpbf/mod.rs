use crate::Result;
use geo::{coord, Coord, Geometry, LineString, Point, Polygon};
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Osm {
    pub id: i64,
    pub osm_type: String,
    pub properties: Vec<String>,
    pub geometry: Option<Geometry>,
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
                            properties.push(format!("{}: {}", key, value));
                        }
                        properties
                    },
                    geometry: to_point(n.lat(), n.lon()),
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
                            properties.push(format!("{}: {}", key, value));
                        }
                        properties
                    },
                    geometry: to_point(d.lat(), d.lon()),
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
            data.insert(
                w.id(),
                Osm {
                    id: w.id(),
                    osm_type: "node".to_string(),
                    properties: {
                        let mut properties = Vec::new();
                        for (key, value) in w.tags() {
                            properties.push(format!("{}: {}", key, value));
                        }
                        properties
                    },
                    geometry: {
                        let nodes = w.node_locations();
                        let geom = nodes
                            .map(|node| {
                                coord! {x: node.lat(), y: node.lon()}
                            })
                            .collect::<Vec<Coord>>();
                        if geom.get(0) == geom.last() {
                            Some(Geometry::Polygon(Polygon::new(LineString(geom), vec![])))
                        } else {
                            Some(Geometry::LineString(LineString(geom)))
                        }
                    },
                },
            );
        }
        Element::Relation(r) => (),
    })?;

    Ok(data)
}

pub fn to_point(lat: f64, lon: f64) -> Option<Geometry> {
    Some(Geometry::Point(Point::new(lat, lon)))
}

// pub fn to_line_or_poly(way: &Way) -> Option<Vec<Geometry>> {
//     let nodes = way.node_locations();
//     let geom = nodes
//         .map(|node| {
//             coord! {x: node.lat(), y: node.lon()}
//         })
//         .collect::<Vec<Coord>>();
//     println!("Way: {:?}", way.node_locations());
//     panic!("EXIT");
//     println!("{:?}", geom);
//     if geom.get(0) == geom.last() {
//         Some(Geometry::Polygon(Polygon::new(LineString(geom), vec![])))
//     } else {
//         Some(Geometry::LineString(LineString(geom)))
//     }
// }
