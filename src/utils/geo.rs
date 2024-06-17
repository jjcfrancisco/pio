use geo::{Geometry, Point};
use wkt::ToWkt;

pub fn to_point(lat: f64, lon: f64) -> Option<Geometry> {
    Some(Geometry::Point(Point::new(lat, lon)))
}

pub fn to_point_t(lat: f64, lon: f64) -> Option<String> {
    Some(Point::new(lat, lon).wkt_string())
}
