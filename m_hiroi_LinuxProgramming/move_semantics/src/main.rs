fn main() {
    let a = vec![1, 2, 3, 4, 5];
    println!("{:?}", a);
    let b = a;
    println!("{:?}", b);
    let c: Vec<i32>;
    {
        let d = b;
        println!("{:?}", d);
        c = d;
    }
    println!("{:?}", c);
}
