pub struct Object {
    size: u64,
}
pub struct Apple {
    pub size: u64,
}
pub struct Pen {
    pub size: u64,
}


pub trait ObjectTrait {
    fn as_object(&self) -> Object;
}

pub trait ObjectLogic {
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