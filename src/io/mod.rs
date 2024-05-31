use crate::osmpbf::Osm;
use geojson::{Feature, FeatureCollection, GeoJson, JsonObject, JsonValue};
use serde_json::to_string_pretty;
use std::collections::HashMap;

use crate::Result;

pub fn write_geojson(objects: HashMap<i64, Osm>) -> Result<()> {
    let features: Vec<Feature> = objects
        .iter()
        .map(|(_, object)| Feature {
            bbox: None,
            geometry: object.geometry.as_ref().map(|g| g.into()),
            id: Some(geojson::feature::Id::String(object.id.to_string())),
            properties: {
                // Iterate over properties to create JsonObject
                let mut properties = JsonObject::new();
                for prop in &object.properties {
                    let key_value: Vec<&str> = prop.split(": ").collect();
                    properties.insert(String::from(key_value[0]), JsonValue::from(key_value[1]));
                }
                Some(properties)
            },
            foreign_members: None,
        })
        .collect();

    let fc = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };

    let gj = GeoJson::from(fc);
    let pgj = to_string_pretty(&gj);
    if pgj.is_err() {
        return Err("Cannot format GeoJSON feature collection".into());
    }
    std::fs::write("melilla.geojson", pgj.unwrap())?;

    // let features: Vec<Feature> = objects
    //     .iter()
    //     .map(|object| Feature {
    //         bbox: None,
    //         geometry: object.geometry.as_ref().map(|g| g.into()),
    //         id: Some(geojson::feature::Id::String(object.id.to_string())),
    //         properties: {
    //             // Iterate over properties to create JsonObject
    //             let mut properties = JsonObject::new();
    //             for prop in &object.properties {
    //                 let key_value: Vec<&str> = prop.split(": ").collect();
    //                 properties.insert(String::from(key_value[0]), JsonValue::from(key_value[1]));
    //             }
    //             Some(properties)
    //         },
    //         foreign_members: None,
    //     })
    //     .collect();
    // let fc = FeatureCollection {
    //     bbox: None,
    //     features,
    //     foreign_members: None,
    // };
    //
    // let gj = GeoJson::from(fc);
    // let pgj = to_string_pretty(&gj);
    // if pgj.is_err() {
    //     return Err("Cannot format GeoJSON feature collection".into());
    // }
    // std::fs::write("melilla.geojson", pgj.unwrap())?;
    Ok(())
}
