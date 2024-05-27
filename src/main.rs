pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod fs;

use crate::fs::{read_osmpbf, write_geojson};

fn main() -> Result<()> {
    let data = read_osmpbf("melilla-latest.osm.pbf")?;
    write_geojson(data)
}

