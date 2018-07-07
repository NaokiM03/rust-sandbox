mod object;

use object::ObjectLogic;
use object::apple::Apple;
use object::pen::Pen;

fn main() {
    let apple = Apple { size: 1 };
    let pen = Pen { size: 2};

    println!("size: {}", apple.get_size());
    println!("size: {}", pen.get_size());
}