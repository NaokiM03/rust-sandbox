use object::Object;
use object::ObjectTrait;

pub struct Pen {
    pub size: u64,
}

impl ObjectTrait for Pen {
    fn as_object(&self) -> Object {
        Object { size: self.size }
    }
}
