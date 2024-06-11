use crate::Result;
use serde::Deserialize;

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
