use std::collections::HashMap;
use crate::program::Module;

pub mod io;
mod threading;


pub fn get_std_module() -> Module {
    let mut module = Module::new("std", HashMap::new(), Vec::new(), HashMap::new());

    module.add_sub_module(io::get_io_module());

    module
}