// use crate::osmpbf;
// use crate::schema;
// use crate::schema::{Pio, PioCollection, PropertyValue};
// use crate::utils::config::{read_yaml, Config};
// use crate::{osmpbf::Osm, Result};
// use std::collections::HashMap;
//
//; fn apply_fields(config: &Config, osm: &Osm) -> Option<(String, PropertyValue)> {
//     for field in &config.fields {
//         if let Some(prop) = osm.properties.get(&field.name) {
//             if let Some(mapping) = &field.mapping {
//                 for m in mapping {
//                     if m.key == prop.value {
//                         let field_name = field.rename_to.clone().unwrap_or(field.name.clone());
//                         let value = match field.field_type.as_str() {
//                             "boolean" => PropertyValue::Boolean(m.value.parse().unwrap()),
//                             "integer" => PropertyValue::Integer(m.value.parse().unwrap()),
//                             "float" => PropertyValue::Float(m.value.parse().unwrap()),
//                             _ => PropertyValue::Text(m.value.clone()),
//                         };
//                         return Some((field_name, value));
//                     }
//                 }
//             }
//         }
//     }
//
//     None
// }
//
// fn apply_class(config: &Config, osm: &Osm) -> Option<PropertyValue> {
//     if config.class.is_some() {
//         let class_objects = config.class.as_ref().unwrap();
//         for class_object in class_objects {
//             let prop = osm.properties.get(&class_object.key);
//             if class_object.and.is_some() {
//                 let and = class_object.and.as_ref().unwrap();
//                 if prop.is_some() {
//                     let prop = prop.unwrap();
//                     if class_object.key == prop.key
//                         && class_object.values.contains(&prop.value)
//                         && and.iter().all(|a| {
//                             let prop = osm.properties.get(&a.key);
//                             if prop.is_some() {
//                                 let prop = prop.unwrap();
//                                 a.values.contains(&prop.value)
//                             } else {
//                                 false
//                             }
//                         })
//                     {
//                         return Some(PropertyValue::Text(class_object.then.clone()));
//                     }
//                 }
//             } else {
//                 // Process 'class' members without AND
//                 if prop.is_some() {
//                     let prop = prop.unwrap();
//                     if class_object.key == prop.key && class_object.values.contains(&prop.value) {
//                         return Some(PropertyValue::Text(class_object.then.clone()));
//                     }
//                 }
//             }
//         }
//     }
//
//     None
// }
//
// fn to_omt(config: &Config, osm: &Osm) -> Option<Pio> {
//
//     let allowed_geom_types = &config.geometry_types;
//     let pio = match &osm.geometry_type {
//         Some(geom_type) => {
//             // Checks if the geometry type is in the configuration
//             if allowed_geom_types.contains(&geom_type) {
//                 // Checks if provided YAML has class
//                 if config.class.is_some() {
//                     // If class, apply class
//                     let mut properties:Vec<schema::Property> = Vec::new();
//                     // if let Some(class) = config.class.as_ref().and_then(|_| apply_class(config, osm)) {
//                     //     properties.push(schema::Property {
//                     //         key: "class".to_string(),
//                     //         value: class,
//                     //     });
//                     // }
//                     // if let Some((field_name, value)) = apply_fields(config, osm) {
//                     //     properties.push(Property {
//                     //         key: field_name,
//                     //         value,
//                     //     });
//                     // }
//                     Some(Pio {
//                         osm_id: osm.id,
//                         osm_type: osm.osm_type.to_string(),
//                         geometry: None,  // Assuming geometry is always Some when geometry_type is Some
//                         properties,
//                     })
//
//
//
//
//
//
//                     // let class = apply_class(config, osm);
//                     // if class.is_some() {
//                     //     properties.push(Property {
//                     //         key: "class".to_string(),
//                     //         value: class.unwrap(),
//                     //     });
//                     // }
//                     // Apply fields
//                     // let field = apply_fields(config, osm);
//                     // if field.is_some() {
//                     //     let (field_name, value) = field.unwrap();
//                     //     properties.insert(field_name, value);
//                     // }
//                     // Some(Pio {
//                     //     osm_id: osm.id,
//                     //     osm_type: osm.osm_type.clone(),
//                     //     geometry: osm.geometry.clone().unwrap(),
//                     //     properties,
//                     // })
//                 } else {
//                     // If no class, apply fields
//                     // let mut properties = Vec::new();
//                     // let field = apply_fields(config, osm);
//                     // if field.is_some() {
//                     //     let (field_name, value) = field.unwrap();
//                     //     properties.push(Property {
//                     //         key: field_name,
//                     //         value,
//                     //     });
//                     // }
//
//                     // Some(Pio {
//                     //     osm_id: osm.id,
//                     //     osm_type: osm.osm_type.clone(),
//                     //     geometry: osm.geometry.clone().unwrap(),
//                     //     properties,
//                     // })
//                     None
//                 }
//             } else {
//                 // If the geometry type is not in the configuration, ignore the object altogether
//                 return None;
//             }
//         }
//         // If the geometry type is not provided, ignore the object altogether as well
//         None => None
//     };
//
//     pio
//
// }
//
// pub fn apply(yaml_file: &str, data: osmpbf::OsmCollection) -> Result<PioCollection> {
//     // Read YAML configuration
//     let config = read_yaml(yaml_file)?;
//
//     let mut pc = PioCollection::new();
//     pc.layer = config.layer.clone();
//
//     // Iterate over data
//     for (_, osm) in data.objects.iter() {
//         // Available: osm.id, osm.osm_type, osm.properties, osm.geometry
//         if let Some(pio) = to_omt(&config, osm) {
//             pc.add(pio);
//         }
//         // let pio = to_omt(&config, osm);
//         // match pio {
//         //     Some(pio) => {
//         //         pc.add(pio);
//         //     }
//         //     None => {}
//         // }
//     }
//
//     Ok(pc)
//
// }
//
// // Unit tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use geo::{Point, Geometry};
//     use crate::osmpbf::Property;
//
//     #[test]
//     fn test_apply_fields() {
//         let yaml = "
//         schema: omt
//         layer: test
//         geometry_types:
//           - Point
//           - LineString
//           - Polygon
//         fields:
//           - name: name:en
//             field_type: boolean
//             rename_to: name_en
//             mapping:
//                 - key: yes
//                   value: true
//                 - key: no
//                   value: false
//         class:
//           - key: amenity
//             values: ['bus_stop', 'bus_station']
//             then: bus
//           - key: railway
//             values: ['halt', 'tram_stop', 'subway']
//             and:
//               - key: railway
//                 values: ['station']
//             then: railway
//         ";
//
//         let deser: Config = serde_yaml::from_str(&yaml).unwrap();
//         let osm = Osm {
//             id: 1,
//             osm_type: "node".to_string(),
//             properties: {
//                 let mut properties = HashMap::new();
//                 properties.insert("name:en".to_string(), Property {
//                     key: "name:en".to_string(),
//                     value: "yes".to_string(),
//                 });
//                 properties
//             },
//             geometry: None,
//             geometry_type: None,
//         };
//
//         let field = apply_fields(&deser, &osm);
//         assert_eq!(field, Some(("name_en".to_string(), PropertyValue::Boolean(true))));
//     }
//
//     #[test]
//     fn test_apply_class() {
//         let yaml = "
//         schema: omt
//         layer: test
//         geometry_types:
//           - Point
//           - LineString
//           - Polygon
//         fields:
//           - name: name:en
//             field_type: boolean
//             rename_to: name_en
//             mapping:
//                 - key: yes
//                   value: true
//                 - key: no
//                   value: false
//         class:
//           - key: amenity
//             values: ['bus_stop', 'bus_station']
//             then: bus
//           - key: railway
//             values: ['halt', 'tram_stop', 'subway']
//             and:
//               - key: railway
//                 values: ['station']
//             then: railway
//         ";
//
//         let deser: Config = serde_yaml::from_str(&yaml).unwrap();
//         let osm = Osm {
//             id: 1,
//             osm_type: "node".to_string(),
//             properties: {
//                 let mut properties = HashMap::new();
//                 properties.insert("amenity".to_string(), Property {
//                     key: "amenity".to_string(),
//                     value: "bus_stop".to_string(),
//                 });
//                 properties
//             },
//             geometry: None,
//             geometry_type: None,
//         };
//
//         let class = apply_class(&deser, &osm);
//         assert_eq!(class, Some(PropertyValue::Text("bus".to_string())));
//     }
//
//     #[test]
//     fn test_to_omt() {
//         let yaml = "
//         schema: omt
//         layer: test
//         geometry_types:
//           - Point
//           - LineString
//           - Polygon
//         fields:
//           - name: name:en
//             field_type: boolean
//             rename_to: name_en
//             mapping:
//                 - key: yes
//                   value: true
//                 - key: no
//                   value: false
//         class:
//           - key: amenity
//             values: ['bus_stop', 'bus_station']
//             then: bus
//           - key: railway
//             values: ['halt', 'tram_stop', 'subway']
//             and:
//               - key: railway
//                 values: ['station']
//             then: railway
//         ";
//
//         let deser: Config = serde_yaml::from_str(&yaml).unwrap();
//         let osm = Osm {
//             id: 1,
//             osm_type: "node".to_string(),
//             properties: {
//                 let mut properties = HashMap::new();
//                 properties.insert("amenity".to_string(), Property {
//                     key: "amenity".to_string(),
//                     value: "bus_stop".to_string(),
//                 });
//                 properties.insert("name:en".to_string(), Property {
//                     key: "name:en".to_string(),
//                     value: "yes".to_string(),
//                 });
//                 properties
//             },
//             geometry: Some(Geometry::Point(Point::new(1.0, 2.0))),
//             geometry_type: Some("Point".to_string()),
//         };
//
//         let pio = to_omt(&deser, &osm);
//         assert_eq!(pio, Some(Pio {
//             osm_id: 1,
//             osm_type: "node".to_string(),
//             geometry: Geometry::Point(Point::new(1.0, 2.0)),
//             properties: {
//                 let mut properties = HashMap::new();
//                 properties.insert("class".to_string(), PropertyValue::Text("bus".to_string()));
//                 properties.insert("name_en".to_string(), PropertyValue::Boolean(true));
//                 properties
//             },
//         }));
//     }
// }
