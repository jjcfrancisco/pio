pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod pg;
mod utils;
mod process;

use utils::cli;
use crate::pg::{create_table, create_connection, create_binary_writer, get_geom_type};

fn main() -> Result<()> {
    let args = cli::run()?;
    let mut client = create_connection(&args.uri)?;
    let stmt = create_table(&mut client)?;
    let geom_type = get_geom_type(stmt)?;
    let writer = create_binary_writer(&mut client)?;
    process::nodes("melilla-latest.osm.pbf", writer, geom_type)?;

    Ok(())
}
