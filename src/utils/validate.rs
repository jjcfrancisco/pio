use crate::Result;
use crate::utils::cli::Cli;

#[derive(Debug)]
pub struct Validated<'a> {
    pub uri: &'a str,
    pub path_osmpbf: &'a str,
}

impl<'a> Validated<'a> {
    pub fn new(uri: &'a str, path_osmpbf: &'a str) -> Self {
        Self { uri, path_osmpbf }
    }

    pub fn is_empty(&self) {
        if self.uri.is_empty() {
            eprintln!("URI is required");
            std::process::exit(1);
        }
    }

    // Validate path of the osm.pbf file
    pub fn is_valid(&self) {
        if !std::path::Path::new(&self.path_osmpbf).exists() {
            eprintln!("File path does not exist");
            std::process::exit(1);
        }
    }
}

pub fn validate_args(args: &Cli) -> Result<Validated> {
    let validated = Validated::new(&args.uri, &args.osmpbf);
    validated.is_empty();
    validated.is_valid();

    Ok(Validated {
        uri: &args.uri,
        path_osmpbf: &args.osmpbf,
    })
}
