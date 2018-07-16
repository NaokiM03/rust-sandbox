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


fn main() {
    for n in 10 .. 20 {
        println!("{}", fact(n));
    }
    for n in 10 .. 20 {
        println!("{}", fact_with_wild_card(n));
    }

}
