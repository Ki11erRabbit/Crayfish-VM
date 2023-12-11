
use crate::program::function::{Function, FunctionPath};
use crate::value::Value;


pub type ClassId = u64;

pub type Method = (FunctionPath, Function);

pub struct Object {
    class_id: ClassId,
    class_name: *const str,
    super_class: Option<ClassRef>,
    fields: *mut [Value],
    vtable: *mut [Method],
}


pub type ClassRef = *mut Object;