trait Foo {
    fn method_a(&self);
}

trait Bar : Foo {
    fn method_b(&self);
}

struct Baz;

impl Foo for Baz {
    fn method_a(&self) {
        println!("method_a!");
    }
}

impl Bar for Baz {
    fn method_b(&self) {
        println!("method_b!");
    }
}


fn main() {
}
