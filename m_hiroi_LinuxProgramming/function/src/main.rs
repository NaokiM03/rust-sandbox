fn fact(n: i64) -> i64 {
    if n == 0 {
        1
    }
    else {
        n * fact(n -1)
    }
}

fn fibo(n: i64) -> i64 {
    fn fiboiter(n: i64, a: i64, b: i64) -> i64 {
        if n == 0 {
            a
        }
        else {
            fiboiter(n - 1, b, a + b)
        }
    }
    fiboiter(n, 0, 1)
}

fn main() {
    for n in 10 .. 20 {
        println!("{}", fact(n));
    }

    for n in 40 .. 50 {
        println!("{}", fibo(n));
    }
}
