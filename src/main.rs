pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod osmpbf;
mod schema;
mod utils;

use std::collections::HashMap;

use osmpbf::{read_osmpbf, Osm};
use schema::omt;
use utils::write_geojson;

// Here for debugging purposes. Remove when not needed
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
    // Gets osm data
    let osm = read_osmpbf("melilla-latest.osm.pbf")?;
    // Process data
    let omt = omt::apply("poi.yaml", osm)?;

    // Save data to GeoJSON
    write_geojson(omt)?;
    Ok(())
}
