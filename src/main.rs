mod point {
    use core::fmt;

    pub struct Coordinate {
        pub long: f32,
        pub lat: f32,
        pub elevation: Option<f32>,
        _private: (),
    }

    impl Coordinate {
        pub fn new(long: f32, lat: f32) -> Self {
            if long < -180.0 || long > 180.0 {
                panic!("Out of bounds. Longitude must be above -180 and below 180");
            }
            if lat < -90.0 || lat > 90.0 {
                panic!("Out of bounds. Latitude must be above -90 and below 90");
            }

            Coordinate {
                long: long,
                lat: lat,
                elevation: None,
                _private: (),
            }
        }
    }

    impl fmt::Display for Coordinate {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}, {}]", self.long, self.lat)
        }
    }
}

use point::Coordinate;
use std::f32::consts::PI;

trait RadiansDegrees {
    fn rad_to_deg(&self) -> f32;
    fn deg_to_rad(&self) -> f32;
}

impl RadiansDegrees for f32 {
    fn rad_to_deg(&self) -> f32 {
        *self * (180.0 / PI)
    }

    fn deg_to_rad(&self) -> f32 {
        *self * (PI / 180.0)
    }
}

fn main() {
    let p1 = Coordinate::new(1.0, 2.0);
    let p2 = Coordinate::new(4.0, 2.0);
    let p3 = Coordinate::new(4.0, 7.0);

    let angle = find_p2_angle_as_rad(p1, p2, p3);

    println!("angle: {} || {}°", angle, angle.rad_to_deg());
}

fn find_p2_angle_as_rad(p1: Coordinate, p2: Coordinate, p3: Coordinate) -> f32 {
    let dist_p1p2: f32 = ((p1.long - p2.long).powi(2) + (p1.lat - p2.lat).powi(2)).sqrt(); // Dont need to sqrt if going to ^2 later
    let dist_p2p3: f32 = ((p2.long - p3.long).powi(2) + (p2.lat - p3.lat).powi(2)).sqrt();
    let dist_p3p1: f32 = ((p3.long - p1.long).powi(2) + (p3.lat - p1.lat).powi(2))
        .sqrt()
        .abs();

    ((dist_p1p2.powi(2) + dist_p2p3.powi(2) - dist_p3p1.powi(2)) / (2.0 * dist_p1p2 * dist_p2p3))
        .acos()
}
