fn find_if<T>(pred: fn(&T) -> bool, table: &[T]) -> Option<&T> {
    for x in table {
        if pred(x) { return Some(x); }
    }
    None
}

#[derive(Debug, PartialEq)]
struct Pair<T, U> {
    fst: T, snd: U
}

impl<T, U> Pair<T, U> {
    fn new(a: T, b: U) -> Pair<T, U> {
        Pair { fst: a, snd: b }
    }
}

fn main() {
    fn evenp(x: &i32) -> bool { x % 2 == 0 }
    fn oddp(x: &i32) -> bool { x % 2 != 0 }
    let a = [2, 4, 6, 8, 10];
    match find_if(evenp, &a) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
    match find_if(oddp, &a) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }

    let p1 = Pair::new(1, 2.0);
    let p2 = Pair::new(1, 3.0);
    let p3 = Pair::new("foo", 100);
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
    println!("{}", p1 == p2);
    println!("{}", p3 == p3);
}
