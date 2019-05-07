pub struct MaxThree {
    pub counter: usize,
}
impl MaxThree {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
    pub fn increase(&mut self) {
        if self.counter < 3 {
            self.counter += 1;
        }
    }
    pub fn current(&self) -> usize {
        self.counter
    }
}

fn main() {
    let mut hoge = MaxThree::new();
    hoge.increase();
    println!("{}", hoge.current());
}
