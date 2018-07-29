enum Fruit  {
    Apple, Banana, Grape, Orange
}

use Fruit::*;

fn get_price(fruit: Fruit) -> i32 {
    match fruit {
        Apple => 200,
        Banana => 150,
        Grape => 300,
        Orange => 100,
    }
}

fn main() {
    println!("{}", get_price(Apple));
    println!("{}", get_price(Banana));
    println!("{}", get_price(Grape));
    println!("{}", get_price(Orange));
}
