use crate::Result;
use wkb::geom_to_wkb;
use geo::{Geometry, Point};
use osmpbf::{Element, ElementReader};
use postgres::binary_copy::BinaryCopyInWriter;
use postgres::types::Type;
use postgres::CopyInWriter;

pub mod bin_geom;

pub fn nodes<'a>(path: &str, writer: CopyInWriter<'a>, geom_type: Type) -> Result<()> {
    let mut writer = BinaryCopyInWriter::new(writer, &[Type::INT4, geom_type]);
    let nodes_reader = ElementReader::from_path(path)?;
    nodes_reader.for_each(|element| match element {
        Element::Node(n) => {
            let id = n.id() as i32;
            let p = Geometry::Point(Point::new(n.lon(), n.lat()));
            let t = geom_to_wkb(&p).expect("Failed to insert node into database");
            let b = bin_geom::Wkb { geometry: t };
            writer
                .write(&[&id, &b])
                .expect("Failed to insert node into database");
        }
        Element::DenseNode(d) => {
            let id = d.id() as i32;
            let p = Geometry::Point(Point::new(d.lon(), d.lat()));
            let t = geom_to_wkb(&p).expect("Failed to insert node into database");
            let b = bin_geom::Wkb { geometry: t };
            writer
                .write(&[&id, &b])
                .expect("Failed to insert node into database");
        }
        _ => {}
    })?;
    writer.finish()?;

    Ok(())
}
