use std::collections::HashMap;
use crate::program::function::{Function, FunctionPath};

pub mod function;




pub struct Module {
    functions: HashMap<Box<str>, Function>,
    string_table: Vec<Box<str>>,
    sub_modules: HashMap<Box<str>, Module>,
}

impl Module {
    pub fn get_function(&self, path: &FunctionPath) -> Option<Function> {
        let mut module = self;
        for part in path.path.iter().take(path.path.len() - 1) {
            module = module.sub_modules.get(part)?;
        }
        match module.functions.get(path.path.last()?) {
            Some(function) => Some(function.clone()),
            None => None
        }
    }
}