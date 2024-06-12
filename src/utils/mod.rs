use crate::schema::{PioCollection, PropertyValue};
use crate::Result;
pub mod config;

use geo::{Geometry, Point};
use geojson::{Feature, FeatureCollection, GeoJson, JsonObject, JsonValue};
use serde_json::to_string_pretty;

pub fn write_geojson(pc: PioCollection) -> Result<()> {
    let features: Vec<Feature> = pc
        .objects
        .iter()
        .map(|pio| Feature {
            bbox: None,
            geometry: Some((&pio.geometry.clone()).into()),
            id: Some(geojson::feature::Id::String(pio.osm_id.to_string())),
            properties: {
                // Iterate over properties to create JsonObject
                let mut properties = JsonObject::new();
                properties.insert(
                    "osm_type".to_string(),
                    JsonValue::from(pio.osm_type.clone()),
                );
                for (key, value) in &pio.properties {
                    match value {
                        PropertyValue::Integer(i) => {
                            // Insert as i64
                            properties.insert(key.clone(), JsonValue::from(*i));
                        }
                        PropertyValue::Float(f) => {
                            properties.insert(key.clone(), JsonValue::from(*f));
                        }
                        PropertyValue::Text(t) => {
                            properties.insert(key.clone(), JsonValue::from(t.clone()));
                        }
                        PropertyValue::Boolean(b) => {
                            properties.insert(key.clone(), JsonValue::from(*b));
                        }
                    }
                    // properties.insert(key.clone(), JsonValue::from({
                    //     match value {
                    //         PropertyValue::Integer(i) => i.to_string(),
                    //         PropertyValue::Float(f) => f.to_string(),
                    //         PropertyValue::Text(t) => t.clone(),
                    //         PropertyValue::Boolean(b) => b.to_string(),
                    //     }
                    // }));
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
    // Use PioCollection layer name as filename
    std::fs::write(format!("{}.geojson", pc.layer), pgj.unwrap())?;
    Ok(())
}

pub fn to_point(lat: f64, lon: f64) -> Option<Geometry> {
    Some(Geometry::Point(Point::new(lat, lon)))
}

