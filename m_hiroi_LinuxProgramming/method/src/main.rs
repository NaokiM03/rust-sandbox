struct Point {
    x: f64, y: f64
}

impl Point {
    fn new(x1: f64, y1: f64) -> Point {
        Point { x: x1, y: y1 }
    }
    fn distance(&self, p: &Point) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(10.0, 10.0);
    let p3 = &p1;
    println!("{}", p1.distance(&p2));
    println!("{}", p3.distance(&p2));    
}
