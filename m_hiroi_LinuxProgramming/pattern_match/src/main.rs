fn fact(n: i64) -> i64 {
    match n {
        0 => 1,
        x => x * fact(x - 1)
    }
}

fn main() {
    for n in 10 .. 20 {
        println!("{}", fact(n));
    }
}
