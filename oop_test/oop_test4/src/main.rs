struct Object {
    size: u64,
}
struct Apple {
    size: u64,
}
struct Pen {
    size: u64,
}


trait ObjectTrait {
    fn as_object(&self) -> Object;
}

trait ObjectLogic {
    fn get_size(&self) -> u64;
}


impl ObjectTrait for Apple {
    fn as_object(&self) -> Object {
        Object { size: self.size }
    }
}

impl ObjectTrait for Pen {
    fn as_object(&self) -> Object {
        Object { size: self.size }
    }
}

impl <T: ObjectTrait> ObjectLogic for T {
    fn get_size(&self) -> u64 {
        let a = self.as_object();
        a.size
    }
}


fn main() {
    let apple = Apple { size: 1 };
    let pen = Pen { size: 2};

    println!("size: {}", apple.get_size());
    println!("size: {}", pen.get_size());
}