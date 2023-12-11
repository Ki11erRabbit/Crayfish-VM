
use crate::program::function::Function;
use crate::value::Value;


pub type ClassId = u64;

pub struct Object<'a> {
    class_id: ClassId,
    class_name: Box<str>,
    super_class: Option<ClassRef<'a>>,
    fields: Box<[Value<'a>]>,
    vtable: Box<[Function]>,
}


pub type ClassRef<'a> = &'a Object<'a>;