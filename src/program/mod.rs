use std::collections::HashMap;
use std::fmt::{Debug, Display, format};
use std::hash::Hash;
use crate::memory::Memory;
use crate::program::function::{Function, FunctionPath};

pub mod function;


#[derive(Clone,Eq)]
pub struct StringTablePath {
    pub(crate) path: Box<[Box<str>]>
}

impl Hash for StringTablePath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let path = format!("{}",self);
        path.hash(state);
    }
}

impl PartialEq for StringTablePath {
    fn eq(&self, other: &Self) -> bool {
        format!("{}",self) == format!("{}",other)
    }
}
impl Display for StringTablePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();
        for part in self.path.iter() {
            path.push_str(part);
            path.push_str("::");
        }
        write!(f, "{}", path)
    }
}

impl Debug for StringTablePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Into<StringTablePath> for &str {
    fn into(self) -> StringTablePath {
        StringTablePath {
            path: self.split("::").map(|s| s.into()).collect::<Vec<Box<str>>>().into_boxed_slice()
        }
    }
}

impl Into<StringTablePath> for Vec<Box<str>> {
    fn into(self) -> StringTablePath {
        StringTablePath {
            path: self.into_boxed_slice()
        }
    }
}


pub struct Module {
    module_name: Box<str>,
    functions: HashMap<Box<str>, Function>,
    string_table: Vec<Box<str>>,
    sub_modules: HashMap<Box<str>, Module>,
}

impl Module {
    pub fn new(module_name: &str, functions: HashMap<Box<str>, Function>, string_table: Vec<Box<str>>, sub_modules: HashMap<Box<str>, Module>) -> Self {
        Module {
            module_name: module_name.to_string().into(),
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

    pub fn get_string(&self, path: &StringTablePath, index: u64) -> Option<&str> {
        let mut module = self;
        for part in path.path.iter().take(path.path.len() - 1) {
            module = module.sub_modules.get(part)?;
        }
        module.string_table.get(index as usize).map(|s| s.as_ref())
    }

    pub fn add_string(&mut self, path: &StringTablePath, string: &str) -> u64 {
        let mut module = self;
        for part in path.path.iter().take(path.path.len() - 1) {
            module = module.sub_modules.entry(part.clone()).or_insert_with(|| Module::default());
        }
        let index = module.string_table.len() as u64;
        module.string_table.push(string.to_string().into_boxed_str());
        index
    }

    pub fn add_strings_to_memory(&self, memory: &mut Memory) {
        let mut path = Vec::new();
        for (index, string) in self.string_table.iter().enumerate() {
            memory.allocate_string_ref(&path.clone().into(), index as u64).expect("Failed to allocate string");
        }

        for (name, module) in self.sub_modules.iter() {
            path.push(name.clone());
            module.add_strings_to_memory_helper(memory, &mut path);
            path.pop();
        }
    }

    fn add_strings_to_memory_helper(&self, memory: &mut Memory, path: &mut Vec<Box<str>>) {
        for (index, string) in self.string_table.iter().enumerate() {
            memory.allocate_string_ref(&(*path).clone().into(), index as u64).expect("Failed to allocate string");
        }

        for (name, module) in self.sub_modules.iter() {
            path.push(name.clone());
            module.add_strings_to_memory_helper(memory, path);
            path.pop();
        }
    }
}

impl Default for Module {
    fn default() -> Self {
        Module {
            module_name: <Box<str>>::into(Box::from("")),
            functions: HashMap::new(),
            string_table: Vec::new(),
            sub_modules: HashMap::new(),
        }
    }
}