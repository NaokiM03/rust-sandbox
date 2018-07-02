fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            print!("FizzBizz ");
        } else if n % 3 == 0 {
            print!("Fizz ");
        } else if n % 5 == 0 {
            print!("Buzz ");
        } else {
            print!("{} ", n);
        }
    }
}
