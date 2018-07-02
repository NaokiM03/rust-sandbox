fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("{}", a[0]);
    println!("{}", a.len());
    let a1 = a;
    println!("{:?}", a);
    println!("{:?}", a1);

    let mut b = [0; 5];
    b[0] = 10;
    println!("{}", b[0]);
    let mut b1 = b;
    b1[0] = 20;
    println!("{:?}", b);
    println!("{:?}", b1);

    let c = &a[2..5];
    println!("{:?}", c);
    println!("{}", c.len());

    let d = &a[3..];
    println!("{:?}", d);
    println!("{}", d.len());

    {
        let e = &mut b[..];
        e[1] = 20;
        println!("{}", e[1]);
    }
    b[1] = 30;
    println!("{}", b[1]);
}
