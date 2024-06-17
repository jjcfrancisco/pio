use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    #[arg(short, long)]
    pub uri: String,
}

pub fn run() -> crate::Result<Cli> {
    let args = Cli::parse();
    if args.uri.is_empty() {
        eprintln!("URI is required");
        std::process::exit(1);
    }
    Ok(args)
}
