pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod osmpbf;
mod utils;
mod schema;

use std::collections::HashMap;

use osmpbf::{process_lines_and_polygons, read_nodes_from_osmpbf, Osm};
use utils::write_geojson;
use schema::{process_data, apply_schema};

#[allow(dead_code)]
fn print_data(data: &HashMap<i64, Osm>) {
    for (id, osm) in data {
        println!("ID: {}", id);
        println!("Type: {}", osm.osm_type);
        println!("Properties: {:?}", osm.properties);
        println!("Geometry: {:?}", osm.geometry);
    }
}

fn main() -> Result<()> {

    let osmpbf_file = "melilla-latest.osm.pbf";
    // Reads read nodes
    let raw_data = read_nodes_from_osmpbf(osmpbf_file)?;
    // Reads lines and polygons
    let data = process_lines_and_polygons(osmpbf_file, raw_data)?;
    // Process data
    let data = process_data("poi.yaml", data)?;

    // Save data to GeoJSON
    // write_geojson(data)?;
    Ok(())
}

