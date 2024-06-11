use crate::osmpbf::{OsmCollection, Property};
use crate::schema::PioCollection;
use crate::Result;
pub mod config;

use geo::{Geometry, Point};
use geojson::{Feature, FeatureCollection, GeoJson, JsonObject, JsonValue};
use polars::prelude::*;
use serde_json::to_string_pretty;
use wkt::ToWkt;

pub fn write_geojson(pc: PioCollection) -> Result<()> {
    let features: Vec<Feature> = pc
        .objects
        .iter()
        .map(|object| Feature {
            bbox: None,
            geometry: Some((&object.geometry.clone()).into()),
            id: Some(geojson::feature::Id::String(object.osm_id.to_string())),
            properties: {
                // Iterate over properties to create JsonObject
                let mut properties = JsonObject::new();
                properties.insert(
                    "osm_type".to_string(),
                    JsonValue::from(object.osm_type.clone()),
                );
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
    // Use PioCollection layer name as filename
    std::fs::write(format!("{}.geojson", pc.layer), pgj.unwrap())?;
    Ok(())
}

pub fn to_point(lat: f64, lon: f64) -> Option<Geometry> {
    Some(Geometry::Point(Point::new(lat, lon)))
}


#[allow(dead_code)]
pub fn to_polars(data: OsmCollection) -> Result<DataFrame> {
    let mut ids = Vec::new();
    let mut geometries: Vec<String> = Vec::new();
    let mut geometry_types = Vec::new();
    let mut properties: Vec<Property> = Vec::new();
    let mut osm_types = Vec::new();

    for (_, osm) in data.objects.iter() {
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
