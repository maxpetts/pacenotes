fn main() {
    let p1 = LongLat {
        long: 1.0,
        lat: 2.0,
    };

    let p2 = LongLat {
        long: 4.0,
        lat: 2.0,
    };

    let p3 = LongLat {
        long: 1.0,
        lat: 2.0,
    };

    find_angle_at_p2(p1, p2, p3)
}

struct LongLat {
    // Add constructor to keep within bounds
    long: f32,
    lat: f32,
}

fn find_angle_at_p2(p1: LongLat, p2: LongLat, p3: LongLat) {
    let dist_p1p2: f32 = ((p1.long - p2.long).powi(2) + (p1.lat - p2.lat).powi(2)).sqrt();
    let dist_p2p3: f32 = ((p2.long - p3.long).powi(2) + (p2.lat - p3.lat).powi(2)).sqrt();
    let dist_p3p1: f32 = ((p3.long - p1.long).powi(2) + (p3.lat - p1.lat).powi(2)).sqrt();
    println!(
        "dist_p1p2: {}, dist_p2p3: {}, div: {}",
        dist_p1p2,
        dist_p2p3,
        dist_p1p2 / &dist_p2p3
    );
}
