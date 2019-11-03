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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz_sub_test() {
        assert_eq!(collatz_sub(4), Some(2));
        assert_eq!(collatz_sub(5), Some(16));
    }

    #[test]
    fn collatz_test() {
        let result_1 = "\
6
3
10
5
16
8
4
2
1
";
        assert_eq!(collatz(6), result_1);

        let result_2 = "\
13
40
20
10
5
16
8
4
2
1
";
        assert_eq!(collatz(13), result_2);
    }
}
