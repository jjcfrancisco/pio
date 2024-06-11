use geo::Geometry;

pub mod omt;

pub struct Pio {
    pub osm_id: i64,
    pub osm_type: String,
    pub geometry: Geometry,
    pub class: String,
}

pub struct PioCollection {
    pub objects: Vec<Pio>,
    pub layer: String,
}

impl PioCollection {
    pub fn new() -> Self {
        PioCollection {
            layer: "default".to_string(),
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Pio) {
        self.objects.push(object);
    }
}
