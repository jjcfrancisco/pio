use crate::utils::validate::validate_args;
use crate::Result;

use clap::Parser;

use crate::pg::{create_binary_writer, create_connection, create_table, infer_geom_type};
use crate::process::elements::process_nodes;
use crate::utils::config::read_schema_yamls;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    #[arg(short, long)]
    pub uri: String,

    #[arg(short, long)]
    pub osmpbf: String,

    #[arg(long = "schema-yamls")]
    pub schema_yamls: String,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    validate_args(&args)?;

    let mut client = create_connection(&args.uri)?;
    let stmt = create_table(&mut client)?;
    let geom_type = infer_geom_type(stmt)?;
    let writer = create_binary_writer(&mut client)?;
    let configs = read_schema_yamls(&args.schema_yamls)?;
    process_nodes(&args.osmpbf, configs, writer, geom_type)?;

    Ok(())
}
