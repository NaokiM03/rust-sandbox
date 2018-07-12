fn fact(n: i64) -> i64 {
    if n == 0 {
        1
    }
    else {
        n * fact(n -1)
    }
}

fn main() {
    for n in 10 .. 20 {
        println!("{}", fact(n));
    }
}
