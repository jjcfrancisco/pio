use geo::{Geometry, Point};
use wkt::ToWkt;

pub fn to_point(lat: f64, lon: f64) -> Option<Geometry> {
    Some(Geometry::Point(Point::new(lat, lon)))
}

