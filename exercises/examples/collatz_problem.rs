fn collatz_sub(n: u64, mut ret: String) -> (u64, String) {
    let num = match n % 2 {
        0 => n / 2,
        _ => n * 3 + 1,
    };
    let num_str = num.to_string() + "\n";
    ret.push_str(&num_str);
    if num == 1 {
        (num, ret)
    } else {
        collatz_sub(num, ret)
    }
}

fn collatz(n: u64) -> String {
    let mut ret: String = "".to_owned();
    let result = collatz_sub(n, ret);
    result.1
}

fn main() {
    println!("{}", collatz(1234567890));
}
