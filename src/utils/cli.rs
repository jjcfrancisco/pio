use crate::Result;
use crate::utils::validate::validate_args;

use clap::Parser;

use crate::pg::{create_binary_writer, create_connection, create_table, infer_geom_type};
use crate::process::nodes::process_nodes;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    #[arg(short, long)]
    pub uri: String,

    #[arg(short, long)]
    pub osmpbf: String,
}

pub fn run() -> Result<Cli> {
    let args = Cli::parse();
    let validated = validate_args(&args)?;

    let mut client = create_connection(&args.uri)?;
    let stmt = create_table(&mut client)?;
    let geom_type = infer_geom_type(stmt)?;
    let writer = create_binary_writer(&mut client)?;
    process_nodes(validated.path_osmpbf, writer, geom_type)?;

    Ok(args)
}
