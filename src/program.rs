use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use crate::instruction::Instruction;


pub type NativeFunction = fn() -> ();

#[derive(Clone)]
pub struct FunctionPath {
    path: Box<[Box<str>]>
}

impl Display for FunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();
        for part in self.path.iter() {
            path.push_str(part);
            path.push_str("::");
        }
        write!(f, "{}", path)
    }
}
impl Debug for FunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


#[derive(Clone)]
pub struct Function {
    instructions: Arc<Vec<Instruction>>,
    native: NativeFunction,
}


pub struct Module {
    functions: HashMap<Box<str>, Function>,
    string_table: Vec<Box<str>>,
    sub_modules: HashMap<Box<str>, Module>,
}