fn main() {
    let mut a = "hello, ".to_string();
    println!("{}", a);
    a.push_str("world");
    println!("{}", a);
    let b = a + ", oops!";
    println!("{}", b);
}
