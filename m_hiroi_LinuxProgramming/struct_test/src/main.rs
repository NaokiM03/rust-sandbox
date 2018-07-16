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

fn distance(p3: &Point3D, p4: &Point3D) -> f64 {
    let dx = p3.x - p4.x;
    let dy = p3.y - p4.y;
    let dz = p3.z - p4.z;
    (dx *dx + dy * dy + dz * dz).sqrt()
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

    let p3 = Point3D {x:0.0, y:0.0, z:0.0};
    let p4 = Point3D {x:10.0, y:10.0, z:10.0};
    println!("{}", distance(&p3, &p4));
}
