use crate::Result;
use geo::Geometry;
use osmpbf::{Element, ElementReader};

use postgres::CopyInWriter;
use postgres::binary_copy::BinaryCopyInWriter;
use postgres::types::Type;

#[derive(Debug)]
pub struct Property {
    pub key: &'static str,
    pub value: &'static str,
}

#[derive(Debug)]
pub struct Pio {
    pub id: i64,
    pub osm_type: String,
    pub properties: Vec<Property>,
    pub geometry: Option<Geometry>,
}

pub fn nodes<'a>(path: &str, writer: CopyInWriter<'a>) -> Result<()> {

    let mut writer = BinaryCopyInWriter::new(writer, &[Type::INT4]);
    let nodes_reader = ElementReader::from_path(path)?;
    nodes_reader.for_each(|element| match element {
        Element::Node(n) => {
            let id = n.id() as i32;
            writer.write(&[&id]).expect("Failed to insert node into database");
        }
        Element::DenseNode(d) => {
            let id = d.id() as i32;
            writer.write(&[&id]).expect("Failed to insert node into database");
        }
        _ => {}
    })?;
    writer.finish()?;

    Ok(())
}

