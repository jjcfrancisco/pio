pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod pg;
mod utils;
mod process;

use utils::cli;
use crate::pg::{create_table, create_connection, create_binary_writer};

fn main() -> Result<()> {
    let args = cli::run()?;
    let mut client = create_connection(&args.uri)?;
    create_table(&mut client)?;
    let writer = create_binary_writer(&mut client, "COPY pio (id) FROM stdin BINARY")?;
    process::nodes("spain-latest.osm.pbf", writer)?;


    // Async
    // let client = create_connection(&args.uri).await?;
    // create_table(&client).await?;
    // let sink = create_sink(&client, "COPY pio (id) FROM STDIN BINARY").await?;
    // process::nodes("spain-latest.osm.pbf", sink).await?;

    Ok(())
}
