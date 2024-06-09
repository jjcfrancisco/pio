use crate::osmpbf::Osm;
use geojson::{Feature, FeatureCollection, GeoJson, JsonObject, JsonValue};
use serde_json::to_string_pretty;
use std::collections::HashMap;

use crate::Result;

// Read file to YAML
pub fn read_yaml(path: &str) -> Result<serde_yaml::Value> {
    let file = std::fs::read_to_string(path)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&file)?;
    Ok(yaml)
}

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
                    properties.insert(prop.key.clone(), JsonValue::from(prop.value.clone()));
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
    Ok(())
}


#[cfg(test)]
mod tests {
    use serde_yaml;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Config {
        schema: String,
        geometry_types: Vec<String>,
        fields: Vec<Field>,
        class: Vec<Kvat>,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Field {
        name: String,
        field_type: String,
        rename_to: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Kvat {
        key: String,
        values: Vec<String>,
        and: Option<Vec<Kv>>,
        then: String,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Kv {
        key: String,
        values: Vec<String>,
    }

    #[test]
    fn test_read_yaml() {
        let yaml = "
        schema: omt
        geometry_types:
          - Point
          - LineString
          - Polygon
        fields:
          - name: name:en
            field_type: string
            rename_to: name_en
        class:
          - key: amenity
            values: ['bus_stop', 'bus_station']
            then: bus
          - key: railway
            values: ['halt', 'tram_stop', 'subway']
            and:
              - key: railway
                values: ['station']
            then: railway
        ";

        let deser: Config = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(
            deser,
            Config {
                schema: "omt".to_string(),
                geometry_types: vec!["Point".to_string(), "LineString".to_string(), "Polygon".to_string()],
                fields: vec![Field {
                    name: "name:en".to_string(),
                    field_type: "string".to_string(),
                    rename_to: Some("name_en".to_string()),
                }],
                class: vec![
                    Kvat {
                        key: "amenity".to_string(),
                        values: vec!["bus_stop".to_string(), "bus_station".to_string()],
                        and: None,
                        then: "bus".to_string(),
                    },
                    Kvat {
                        key: "railway".to_string(),
                        values: vec!["halt".to_string(), "tram_stop".to_string(), "subway".to_string()],
                        and: Some(vec![Kv {
                            key: "railway".to_string(),
                            values: vec!["station".to_string()],
                        }]),
                        then: "railway".to_string(),
                    }
                ]
            }
        );
    }
}

