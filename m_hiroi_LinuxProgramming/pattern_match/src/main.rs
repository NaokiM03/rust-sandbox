fn fact(n: i64) -> i64 {
    match n {
        0 => 1,
        x => x * fact(x - 1)
    }
}
fn fact_with_wild_card(n: i64) -> i64 {
    match n {
        0 => 1,
        _ => n * fact_with_wild_card(n - 1)
    }
}

fn fibo(n: i64) -> i64 {
    match n {
        0 ... 1 => n,
        _ => fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    for n in 10 .. 20 {
        println!("{}", fact(n));
    }
    for n in 10 .. 20 {
        println!("{}", fact_with_wild_card(n));
    }

    for n in 30 .. 40 {
        println!("{}", fibo(n));
    }

    let ref x = 100;
    match x {
        y => println!("{}", y)
    }
    let ref x = 100;
    match x {
        &y => println!("{}", y)
    }
    let ref x = 100;
    match *x {
        y => println!("{}", y)
    }

}
