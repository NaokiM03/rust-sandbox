fn main() {
    let a = 123;
    let ra = &a;
    let a1 = ra * 10;
    println!("{}", ra);
    println!("{}", a1);

    let mut b = 456;
    println!("{}", b);
    {
    let rb = &mut b;
    println!("{}", rb);
    *rb = 1000;
    }
    println!("{}", a);
    println!("{}", ra);
    println!("{}", a1);
    println!("{}", b);
}
