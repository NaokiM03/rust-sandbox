struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point3D {
        x: 0.0,
        y: 1.0,
        z: 2.0
    };
    let p2 = Point3D {
        y: 2.0,
        .. p1
    };
    println!("{},{},{}", p1.x, p1.y, p1.z);
    println!("{},{},{}", p2.x, p2.y, p2.z);

    let Point {x: a, y: b} = Point {x: 1.0, y: 2.0};
    println!("{:?}", Point{x: a, y: b});
}
