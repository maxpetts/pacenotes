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

#[derive(Debug)]
struct directions {}

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
    let p3 = Coordinate::new(4.0, 1.0);

    let angle = find_p2_angle_as_rad(p1, p2, p3);

    println!("angle: {} || {}°", angle, angle.rad_to_deg());
    get_driving_route("13.388860,52.517037;13.397634,52.529407;13.428555,52.523219?overview=full&annotations=true&steps=true");
}

fn get_driving_route(coords: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://router.project-osrm.org/route/v1/driving/{}", coords);
    let mut res = reqwest::blocking::get(&url)?;

    match res.status() {
        reqwest::StatusCode::OK => {
            println!("{}", url);
            println!("Stat: {}", res.status());
            println!("Headers: {:#?}", res.headers());
            println!("Body: {:#?}", res.text().unwrap());
            println!("---------");
            Ok(())
        }
        _ => {
            panic!("Couldn't get data from OSRM");
        }
    }
}

fn find_p2_angle_as_rad(p1: Coordinate, p2: Coordinate, p3: Coordinate) -> f32 {
    let sqdist_p1p2: f32 = (p1.long - p2.long).powi(2) + (p1.lat - p2.lat).powi(2);
    let sqdist_p2p3: f32 = (p2.long - p3.long).powi(2) + (p2.lat - p3.lat).powi(2);
    let sqdist_p3p1: f32 = ((p3.long - p1.long).powi(2) + (p3.lat - p1.lat).powi(2)).abs();

    let angle = ((sqdist_p1p2 + sqdist_p2p3 - sqdist_p3p1)
        / (2.0 * sqdist_p1p2.sqrt() * sqdist_p2p3.sqrt()))
    .acos();

    if angle != f32::NAN {
        return angle;
    }

    90.0
}

fn get_bearing(p1: Coordinate, p2: Coordinate, p3: Coordinate) {}
