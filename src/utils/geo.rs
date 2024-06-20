use geo::Point;

pub fn to_point(lat: f64, lon: f64) -> Option<Point> {
    Some(Point::new(lat, lon))
}

