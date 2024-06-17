use crate::schema::{PioCollection, PropertyValue};
use crate::Result;

use geojson::{Feature, FeatureCollection, GeoJson, JsonObject, JsonValue};
use serde_json::to_string_pretty;

pub fn write_geojson(pc: PioCollection) -> Result<()> {
    let features: Vec<Feature> = pc
        .objects
        .iter()
        .map(|pio| Feature {
            bbox: None,
            geometry: None,
            // geometry: Some((&pio.geometry.clone()).into()),
            id: Some(geojson::feature::Id::String(pio.osm_id.to_string())),
            properties: {
                // Iterate over properties to create JsonObject
                let mut properties = JsonObject::new();
                properties.insert(
                    "osm_type".to_string(),
                    JsonValue::from(pio.osm_type.clone()),
                );
                for prop in &pio.properties {
                    match &prop.value {
                        PropertyValue::Integer(i) => {
                            // Insert as i64
                            properties.insert(prop.key.clone(), JsonValue::from(*i));
                        }
                        PropertyValue::Float(f) => {
                            properties.insert(prop.key.clone(), JsonValue::from(*f));
                        }
                        PropertyValue::Text(t) => {
                            properties.insert(prop.key.clone(), JsonValue::from(t.clone()));
                        }
                        PropertyValue::Boolean(b) => {
                            properties.insert(prop.key.clone(), JsonValue::from(*b));
                        }
                    }
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
