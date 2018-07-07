struct Fruit {
    size: u64,
}
struct Apple {
    size: u64,
}
struct PineApple {
    size: u64,
}

trait FruitTrait {}

trait AppleTrait: FruitTrait {
    fn get_size(&self) -> u64 {
        let a: Fruit = self.as_fruit();
        a.size
    }
    fn as_fruit(&self) -> Fruit;
}

trait PineAppleTrait: AppleTrait {}


impl FruitTrait for Apple {}

impl AppleTrait for Apple {
    fn as_fruit(&self) -> Fruit {
        Fruit { size: self.size }
    }
}

impl FruitTrait for PineApple {}

impl AppleTrait for PineApple {
    fn as_fruit(&self) -> Fruit {
        Fruit{ size: self.size }
    }
}

impl PineAppleTrait for PineApple {}

fn main() {
    let apple = Apple { size: 1 };
    let pine_apple = PineApple { size: 2};

    println!("size: {}", apple.get_size());
    println!("size: {}", pine_apple.get_size());
}