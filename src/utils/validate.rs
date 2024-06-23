use crate::utils::cli::Cli;
use crate::utils::config::Kv;
use crate::Result;

// Validate all args are not empty
fn is_empty(args: &Cli) {
    if args.uri.is_empty() || args.osmpbf.is_empty() || args.schema_yamls.is_empty() {
        eprintln!("All arguments are required");
        std::process::exit(1);
    }
}

// Validate osmpbf file exists
fn osmpbf_is_valid(osmpbf: &str) {
    if !std::path::Path::new(osmpbf).exists() {
        eprintln!("File path does not exist");
        std::process::exit(1);
    }
}

// Validate yamls dir exists & contains at least one yaml file
fn yamls_is_valid(yamls: &str) {
    if !std::path::Path::new(yamls).exists() {
        eprintln!("Directory path does not exist");
        std::process::exit(1);
    }
    let dir = std::fs::read_dir(yamls).expect("Failed to read directory");
    let count = dir.count();
    if count == 0 {
        eprintln!("Directory does not contain any yaml files");
        std::process::exit(1);
    }
}

pub fn validate_args(args: &Cli) -> Result<()> {
    is_empty(&args);
    osmpbf_is_valid(&args.osmpbf);
    yamls_is_valid(&args.schema_yamls);
    Ok(())
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Integer(i32),
    Text(String),
    Float(f64),
    Boolean(bool),
}

fn try_parse(value: &str) -> FieldType {
    if let Ok(value) = value.parse::<i32>() {
        return FieldType::Integer(value);
    }
    if let Ok(value) = value.parse::<f64>() {
        return FieldType::Float(value);
    }
    if let Ok(value) = value.parse::<bool>() {
        return FieldType::Boolean(value);
    }
    FieldType::Text(value.to_string())
}

pub fn try_mapping(value: &str, mapping: &Option<Vec<Kv>>) -> FieldType {
    if let Some(mapping) = mapping {
        for map in mapping {
            if map.key == value {
                return try_parse(&map.value);
            }
        }
    }
    try_parse(value)
}

