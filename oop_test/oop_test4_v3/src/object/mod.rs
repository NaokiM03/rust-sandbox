#[path="apple.rs"] pub mod apple;
#[path="pen.rs"] pub mod pen;

pub struct Object {
    size: u64,
}

pub trait ObjectTrait {
    fn as_object(&self) -> Object;
}

pub trait ObjectLogic {
    fn get_size(&self) -> u64;
}


impl <T: ObjectTrait> ObjectLogic for T {
    fn get_size(&self) -> u64 {
        let a = self.as_object();
        a.size
    }
}