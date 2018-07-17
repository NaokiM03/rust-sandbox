mod object;

use object::object::ObjectLogic;
use object::object::Apple;
use object::object::Pen;

fn main() {
    let apple = Apple { size: 1 };
    let pen = Pen { size: 2};

    println!("size: {}", apple.get_size());
    println!("size: {}", pen.get_size());
}