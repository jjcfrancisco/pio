pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod pg;
mod process;
mod utils;

use crate::pg::{create_binary_writer, create_connection, create_table, get_geom_type};
use crate::process::nodes::process_nodes;
use utils::cli;

fn main() -> Result<()> {
    let args = cli::run()?;
    let mut client = create_connection(&args.uri)?;
    let stmt = create_table(&mut client)?;
    let geom_type = infer_geom_type(stmt)?;
    let writer = create_binary_writer(&mut client)?;
    process_nodes("melilla-latest.osm.pbf", writer, geom_type)?;

    Ok(())
}
