pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use crate::utils::database::Database;
use crate::osmpbf::process_nodes;

mod osmpbf;
mod schema;
mod utils;

fn main() -> Result<()> {
    let args = utils::cli::run()?;
    let mut db = Database::new(&args.uri)?;
    if db.check_health().is_err() {
        eprintln!("Database is not healthy");
        std::process::exit(1);
    } else {
        println!("Database is healthy");
    }
    db.create_table()?;
    process_nodes("spain-latest.osm.pbf", db)?;
    
    // read_osmpbf_test("spain-latest.osm.pbf")?;
    Ok(())
}
