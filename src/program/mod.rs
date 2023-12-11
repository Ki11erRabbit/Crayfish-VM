use std::collections::HashMap;
use crate::program::function::{Function, FunctionPath};

pub mod function;




pub struct Module {
    functions: HashMap<Box<str>, Function>,
    string_table: Vec<Box<str>>,
    sub_modules: HashMap<Box<str>, Module>,
}

impl Module {
    pub fn new(functions: HashMap<Box<str>, Function>, string_table: Vec<Box<str>>, sub_modules: HashMap<Box<str>, Module>) -> Self {
        Module {
            functions,
            string_table,
            sub_modules,
        }
    }

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

    pub fn add_function(&mut self, path: &FunctionPath, function: Function) {
        let mut module = self;
        for part in path.path.iter().take(path.path.len() - 1) {
            module = module.sub_modules.entry(part.clone()).or_insert_with(|| Module::default());
        }
        module.functions.insert(path.path.last().unwrap().clone(), function);
    }
}

impl Default for Module {
    fn default() -> Self {
        Module {
            functions: HashMap::new(),
            string_table: Vec::new(),
            sub_modules: HashMap::new(),
        }
    }
}