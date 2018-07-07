struct Apple {
    size: u64,
}
struct PineApple {
    size: u64,
}

trait AppleTrait {
    fn get_size(&self) -> u64;
}

trait PineAppleTrait: AppleTrait {}

impl AppleTrait for Apple {
    fn get_size(&self) -> u64 {
        println!("apple");
        self.size
    }
}

impl AppleTrait for PineApple {
    fn get_size(&self) -> u64 {
        println!("pine apple");
        self.size
    }
}

impl PineAppleTrait for PineApple {}

fn main() {
    let apple = Apple { size: 1 };
    let pine_apple = PineApple { size: 2};

    println!("size: {}", apple.get_size());
    println!("size: {}", pine_apple.get_size());
}