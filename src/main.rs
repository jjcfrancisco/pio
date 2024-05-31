pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use geo::Geometry;

mod osmpbf;
mod io;

use osmpbf::{read_nodes_from_osmpbf, process_lines_and_polygons};
use io::write_geojson;

fn main() -> Result<()> {
    let osmpbf_file = "melilla-latest.osm.pbf";
    // Reads read nodes
    let raw_data = read_nodes_from_osmpbf(osmpbf_file)?;
    // Reads lines and polygons
    let data = process_lines_and_polygons(osmpbf_file, raw_data)?;
    for o in &data {
        // Get geometry
        let geom = o.1.geometry.as_ref().expect("No geometry found");
        // Check geometry is polygon or line
        if let Geometry::Polygon(p) = geom {
            println!("Polygon: {:?}", p);
        } else if let Geometry::LineString(l) = geom {
            println!("LineString: {:?}", l);
        }
    }
    write_geojson(data);
    Ok(())
}

