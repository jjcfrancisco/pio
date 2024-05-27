use osmpbf::{ElementReader, Element};
use geojson::{Feature, FeatureCollection, GeoJson, JsonObject, JsonValue};
use serde_json::to_string_pretty;
use geo::{Geometry, Point};
use crate::Result;


#[allow(dead_code)]
pub struct Osm {
    pub id: i64,
    pub properties: Vec<String>,
    pub geometry: Geometry,
}

pub fn read_osmpbf(path: &str) -> Result<Vec<Osm>> {

    let reader = ElementReader::from_path(path)?;

    // Create new Vector to store Osm objects
    let mut osm_vec: Vec<Osm> = Vec::new();
    let _ = reader.for_each(|element| {
        match element { 
            Element::Node(n) => {
                let mut osm = Osm {
                    id: n.id(),
                    properties: Vec::new(),
                    geometry: Geometry::Point(Point::new(n.lon(), n.lat()))
                };
                for (key, value) in n.tags() {
                    osm.properties.push(format!("{}: {}", key, value));
                }
                osm_vec.push(osm);
            },
            Element::Way(_) => {
                // w.node_locations()
                //     .for_each(|node| {
                //         println!("Node: {:?}", node.lat());
                //         println!("Node: {:?}", node.lon());
                //     });
                //
            },
            Element::DenseNode(d) => {
                let mut osm = Osm {
                    id: d.id(),
                    properties: Vec::new(),
                    geometry: Geometry::Point(Point::new(d.lon(), d.lat()))
                };
                for (key, value) in d.tags() {
                    osm.properties.push(format!("{}: {}", key, value));
                }
                osm_vec.push(osm);
            },
            Element::Relation(_) => (),
        }
    });

    return Ok(osm_vec);

}

pub fn write_geojson(objects: Vec<Osm>) -> Result<()> {

    let features: Vec<Feature> = objects
        .iter()
        .map(|object| Feature {
            bbox: None,
            geometry: None,
            id: Some(geojson::feature::Id::String(object.id.to_string())),
            properties: {
                // Iterate over properties to create JsonObject
                let mut properties = JsonObject::new();
                for prop in &object.properties {
                    let key_value: Vec<&str> = prop.split(": ").collect();
                    properties.insert(
                        String::from(key_value[0]),
                        JsonValue::from(key_value[1]),
                    );
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

