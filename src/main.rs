pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod pg;
mod utils;
mod process;

use utils::cli;
use crate::pg::pool::{create_pool, run_job};

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::run()?;
    let pool = create_pool(&args.uri).await?;
    run_job(&pool).await?;
    // process::nodes("spain-latest.osm.pbf", db)?;

    Ok(())
}
