use crate::osmpbf::{Osm, Property};
use crate::Result;
use crate::schema::PioCollection;

use geojson::{Feature, FeatureCollection, GeoJson, JsonObject, JsonValue};
use polars::prelude::*;
use serde::Deserialize;
use serde_json::to_string_pretty;
use std::collections::HashMap;
use geo::Geometry;
use wkt::ToWkt;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub schema: String,
    pub geometry_types: Vec<String>,
    pub fields: Vec<Field>,
    pub class: Vec<Kvat>,
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: String,
    pub rename_to: Option<String>,
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct Kvat {
    pub key: String,
    pub values: Vec<String>,
    pub and: Option<Vec<Kv>>,
    pub then: String,
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct Kv {
    pub key: String,
    pub values: Vec<String>,
}

// Read file to YAML
pub fn read_yaml(path: &str) -> Result<Config> {
    let file = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&file)?;
    Ok(config)
}

pub fn write_geojson(pc: PioCollection) -> Result<()> {
    let features: Vec<Feature> = pc.objects
        .iter()
        .map(|object| Feature {
            bbox: None,
            geometry: Some((&object.geometry.clone()).into()),
            id: Some(geojson::feature::Id::String(object.osm_id.to_string())),
            properties: {
                // Iterate over properties to create JsonObject
                let mut properties = JsonObject::new();
                properties.insert("osm_type".to_string(), JsonValue::from(object.osm_type.clone()));
                properties.insert("class".to_string(), JsonValue::from(object.class.clone()));
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

#[allow(dead_code)]
pub fn to_polars(data: HashMap<i64, Osm>) -> Result<DataFrame> {
    let mut ids = Vec::new();
    let mut geometries: Vec<String> = Vec::new();
    let mut geometry_types = Vec::new();
    let mut properties: Vec<Property> = Vec::new();
    let mut osm_types = Vec::new();

    for (_, osm) in data.iter() {
        ids.push(osm.id);
        osm_types.push(osm.osm_type.clone());
        let geometry = &osm.geometry;
        match geometry {
            Some(Geometry::Point(p)) => {
                geometries.push(p.wkt_string());
                geometry_types.push("Point");
            }
            Some(Geometry::LineString(ls)) => {
                geometries.push(ls.wkt_string());
                geometry_types.push("LineString");
            }
            Some(Geometry::Polygon(p)) => {
                geometries.push(p.wkt_string());
                geometry_types.push("Polygon");
            }
            _ => panic!("Geometry not supported"),
        };
        for prop in &osm.properties {
            properties.push(Property {
                key: prop.key.clone(),
                value: prop.value.clone(),
            });
        }
    }

    // Create a dataframe
    let mut df = DataFrame::new(vec![
        Series::new("id", ids),
        Series::new("osm_type", osm_types),
        Series::new("geometry", geometries),
        Series::new("geometry_type", geometry_types),
    ])?;

    for prop in properties {
        let s = Series::new(&prop.key, vec![prop.value]);
        df.with_column(s)?;
    }

    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

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
                geometry_types: vec![
                    "Point".to_string(),
                    "LineString".to_string(),
                    "Polygon".to_string()
                ],
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
                        values: vec![
                            "halt".to_string(),
                            "tram_stop".to_string(),
                            "subway".to_string()
                        ],
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
