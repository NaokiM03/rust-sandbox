struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    let p1 = Point3D {
        x: 0.0,
        y: 1.0,
        z: 2.0
    };
    println!("{},{},{}", p1.x, p1.y, p1.z);
}
