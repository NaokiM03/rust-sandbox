use std::iter::successors;

fn collatz_sub(n: u64) -> Option<u64> {
    match n {
        1 => None,
        _ if n % 2 == 0 => Some(n / 2),
        _ => Some(n * 3 + 1),
    }
}

fn collatz(n: u64) -> String {
    let mut ret = "".to_string();
    successors(Some(n), |x| collatz_sub(*x))
        .map(|x| x.to_string() + "\n")
        .for_each(|x| ret.push_str(&x));
    ret
}

fn main() {
    println!("{}", collatz(1234567890));
}
