fn main() {
    let n = 1000000;
    let w = 1.0 / n as f64;
    let mut s = 0.0;
    for i in 1 .. n + 1 {
        let x = (i as f64 - 0.5) * w;
        s += 4.0 / (1.0 + x * x);
    }
    print!("{}", s * w);
}
