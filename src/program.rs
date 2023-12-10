use std::collections::HashMap;
use std::sync::Arc;
use crate::instruction::Instruction;


pub type NativeFunction = fn() -> ();
pub struct FunctionPath {
    path: Box<[Box<str>]>
}
pub struct Function {
    instructions: Arc<Vec<Instruction>>,
    native: NativeFunction,
}


pub struct Module {
    functions: HashMap<Box<str>, Function>,
    string_table: Vec<Box<str>>,
    sub_modules: HashMap<Box<str>, Module>,
}