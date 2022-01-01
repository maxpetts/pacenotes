use core::fmt;

struct Coordinate {
    long: f32,
    lat: f32,
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
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.long, self.lat)
    }
}

fn main() {
    let p1 = Coordinate::new(1.0, 2.0);
    let p2 = Coordinate::new(4.0, 2.0);
    let p3 = Coordinate::new(4.0, 7.0);

    println!("angle: {}", find_angle_at_p2(p1, p2, p3));
}

fn find_angle_at_p2(p1: Coordinate, p2: Coordinate, p3: Coordinate) -> f32 {
    let dist_p1p2: f32 = ((p1.long - p2.long).powi(2) + (p1.lat - p2.lat).powi(2)).sqrt(); // Dont need to sqrt if going to ^2 later
    let dist_p2p3: f32 = ((p2.long - p3.long).powi(2) + (p2.lat - p3.lat).powi(2)).sqrt();
    let dist_p3p1: f32 = ((p3.long - p1.long).powi(2) + (p3.lat - p1.lat).powi(2)).sqrt();
    println!(
        "dist_p1p2: {}, dist_p2p3: {}, div: {}",
        dist_p1p2,
        dist_p2p3,
        dist_p1p2 / &dist_p2p3
    );
}
