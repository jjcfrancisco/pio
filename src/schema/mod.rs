use std::collections::HashMap;

use crate::utils::read_yaml;
use crate::{osmpbf::Osm, Result};

pub fn filter(data: &HashMap<i64, Osm>) -> Result<()> {
    // Read YAML configuration
    let config = read_yaml("examples.yaml")?;

    // config.as_sequence().unwrap().iter().for_each(|v| {
    //     println!("Value: {:?}", v);
    // });

    config.as_mapping().unwrap().iter().for_each(|(k, v)| {
        println!("Key: {:?}, Value: {:?}", k, v);
    });

    Ok(())
}
