use object::object::Object;
use object::object::ObjectTrait;

pub struct Apple {
    pub size: u64,
}

impl ObjectTrait for Apple {
    fn as_object(&self) -> Object {
        Object { size: self.size }
    }
}
