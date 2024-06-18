use crate::utils::database::Database;
use crate::Result;

use geo::Geometry;
use osmpbf::{Element, ElementReader};

#[derive(Debug)]
pub struct Property {
    pub key: &'static str,
    pub value: &'static str,
}

#[derive(Debug)]
pub struct Osm {
    pub id: i64,
    pub osm_type: String,
    pub properties: Vec<Property>,
    pub geometry: Option<Geometry>,
}

pub fn process_nodes(path: &str, db: Database) -> Result<()> {
    let mut db = db.client;
    let nodes_reader = ElementReader::from_path(path)?;
    nodes_reader.for_each(|element| match element {
        Element::Node(n) => {
            db.execute("INSERT INTO pio (id) VALUES ($1)", &[&n.id()])
                .expect("Failed to insert node into database");
        }
        Element::DenseNode(d) => {
            db.execute("INSERT INTO pio (id) VALUES ($1)", &[&d.id()])
                .expect("Failed to insert dense node into database");
        }
        _ => {}
    })?;

    Ok(())
}

pub fn process_ways() {

    // let ways_reader = ElementReader::from_path(path)?;
    // let ways = ways_reader.par_map_reduce(
    //     |element| {
    //         let mut osmc: HashMap<i64, OsmT> = HashMap::new();
    //         let result: HashMap<i64, OsmT> = match element {
    //             Element::Way(w) => {
    //                 let mut osm = OsmT {
    //                     id: w.id(),
    //                     osm_type: "way".to_string(),
    //                     properties: {
    //                         let mut properties = Vec::new();
    //                         for (key, value) in w.tags() {
    //                             properties.push(Property {
    //                                 key: key.to_string(),
    //                                 value: value.to_string(),
    //                             });
    //                         }
    //                         properties
    //                     },
    //                     geometry: None,
    //                 };
    //                 let refs = w.refs();
    //                 let mut geom: Vec<Coord> = Vec::new();
    //                 for ref_id in refs {
    //                     let node = nodes.get(&ref_id);
    //                     if node.is_some() {
    //                         let node = node.unwrap();
    //                         match &node.geometry {
    //                             Some(Geometry::Point(p)) => geom.push(coord! {x: p.x(), y: p.y()}),
    //                             Some(Geometry::LineString(ls)) => {
    //                                 geom.extend(ls.coords_iter().map(|c| c).collect::<Vec<Coord>>())
    //                             }
    //                             _ => (),
    //                         }
    //                     }
    //                 }
    //                 if geom.get(0) == geom.last() {
    //                     osm.geometry = Some(Geometry::Polygon(Polygon::new(LineString(geom), vec![]))
    //                     );
    //                 } else {
    //                     osm.geometry = Some(Geometry::LineString(LineString(geom)));
    //                 };
    //                 osmc.insert(w.id(), osm);
    //                 osmc
    //             },
    //             _ => HashMap::<i64, OsmT>::new(),
    //         };
    //         result
    //     },
    //     || HashMap::<i64, OsmT>::new(),
    //     |mut a, b| {
    //         a.extend(b);
    //         a
    //     }
    // )?;

    // Ok(())
}
