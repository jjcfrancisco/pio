use super::binary_geom::Wkb;
use super::properties::{sort_tags, OsmType};
use crate::Result;
use geo::{Geometry, Point};
use osmpbf::{Element, ElementReader};
use postgres::binary_copy::BinaryCopyInWriter;
use postgres::types::Type;
use postgres::CopyInWriter;
use wkb::geom_to_wkb;

pub fn process_nodes<'a>(path: &str, writer: CopyInWriter<'a>, geom_type: Type) -> Result<()> {
    let mut writer = BinaryCopyInWriter::new(writer, &[Type::INT4, geom_type]);
    let nodes_reader = ElementReader::from_path(path)?;
    nodes_reader.for_each(|element| match element {
        Element::Node(n) => {
            let id = n.id() as i32;
            let properties = sort_tags(OsmType::Node(&n));
            let geom = Geometry::Point(Point::new(n.lon(), n.lat()));
            let wkb = geom_to_wkb(&geom).expect("Failed to insert node into database");
            writer
                .write(&[&id, &Wkb { geometry: wkb }])
                .expect("Failed to insert node into database");
        }
        Element::DenseNode(d) => {
            let id = d.id() as i32;
            let properties = sort_tags(OsmType::DenseNode(&d));
            let geom = Geometry::Point(Point::new(d.lon(), d.lat()));
            let wkb = geom_to_wkb(&geom).expect("Failed to insert node into database");
            writer
                .write(&[&id, &Wkb { geometry: wkb }])
                .expect("Failed to insert node into database");
        }
        _ => {}
    })?;
    writer.finish()?;

    Ok(())
}
