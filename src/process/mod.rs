// #[derive(Debug)]
// pub struct Property {
//     pub key: &'static str,
//     pub value: &'static str,
// }
//
// #[derive(Debug)]
// pub struct Pio {
//     pub id: i64,
//     pub osm_type: String,
//     pub properties: Vec<Property>,
//     pub geometry: Option<Geometry>,
// }

// use crate::pg::Database;
// use crate::Result;
//
// use geo::Geometry;
// use osmpbf::{Element, ElementReader};
//
// pub fn nodes(path: &str, db: Database) -> Result<()> {
//     let mut db = db.client;
//     let nodes_reader = ElementReader::from_path(path)?;
//     nodes_reader.for_each(|element| match element {
//         Element::Node(n) => {
//             db.execute("INSERT INTO pio (id) VALUES ($1)", &[&n.id()])
//                 .expect("Failed to insert node into database");
//         }
//         Element::DenseNode(d) => {
//             db.execute("INSERT INTO pio (id) VALUES ($1)", &[&d.id()])
//                 .expect("Failed to insert dense node into database");
//         }
//         _ => {}
//     })?;
//
//     Ok(())
// }

