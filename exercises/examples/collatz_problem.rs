use std::iter::successors;

fn collatz(n: u64) -> Option<u64> {
    match n {
        1 => None,
        _ if n % 2 == 0 => Some(n / 2),
        _ => Some(n * 3 + 1),
    }
}

fn main() {
    successors(Some(1234567890u64), |n| collatz(*n))
        .for_each(|n| println!("{}", n));
}
