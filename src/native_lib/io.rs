use std::collections::HashMap;
use std::sync::Arc;
use crate::instruction::{RegisterType, Source};
use crate::machine::core::{Core, CoreUtils};
use crate::machine::{Fault, InstructionResult};
use crate::memory::Memory;
use crate::program::function::Function;
use crate::program::Module;
use crate::stack_frame::delimited_continuation::ContinuationStore;
use crate::stack_frame::StackFrame;
use crate::value::Value;

pub fn get_io_module() -> Module {
    let mut module = Module::new("io", HashMap::new(), Vec::new(), HashMap::new());

    module.add_function("println_string", Function::Native(println_string));
    module.add_function("print_string", Function::Native(print_string));
    module.add_function("println_u8", Function::Native(println_u8));
    module.add_function("println_u16", Function::Native(println_u16));
    module.add_function("println_u32", Function::Native(println_u32));
    module.add_function("println_u64", Function::Native(println_u64));
    module.add_function("println_i8", Function::Native(println_i8));
    module.add_function("println_i16", Function::Native(println_i16));
    module.add_function("println_i32", Function::Native(println_i32));
    module.add_function("println_i64", Function::Native(println_i64));
    module.add_function("println_f32", Function::Native(println_f32));
    module.add_function("println_f64", Function::Native(println_f64));
    module.add_function("eprintln_string", Function::Native(eprintln_string));
    module.add_function("eprint_string", Function::Native(eprint_string));
    module.add_function("eprintln_u8", Function::Native(eprintln_u8));
    module.add_function("eprintln_u16", Function::Native(eprintln_u16));
    module.add_function("eprintln_u32", Function::Native(eprintln_u32));
    module.add_function("eprintln_u64", Function::Native(eprintln_u64));
    module.add_function("eprintln_i8", Function::Native(eprintln_i8));
    module.add_function("eprintln_i16", Function::Native(eprintln_i16));
    module.add_function("eprintln_i32", Function::Native(eprintln_i32));
    module.add_function("eprintln_i64", Function::Native(eprintln_i64));
    module.add_function("eprintln_f32", Function::Native(eprintln_f32));
    module.add_function("eprintln_f64", Function::Native(eprintln_f64));

    module
}



fn println_string(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
    let reference = core.get_value(&Source::Register(8, RegisterType::Reference));
    match reference {
        Value::MemoryRef(ref_) => {
            let string = memory.get_string(ref_, &module).unwrap();
            println!("{}", string);
        },
        _ => {}
    }

    Ok(InstructionResult::Continue)
}

fn print_string(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
    let reference = core.get_value(&Source::Register(8, RegisterType::Reference));
    match reference {
        Value::MemoryRef(ref_) => {
            let string = memory.get_string(ref_, &module).unwrap();
            print!("{}", string);
        },
        _ => {}
    }

    Ok(InstructionResult::Continue)
}

macro_rules! generate_println {
    ($name:ident, $type:ident) => {
        fn $name(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
            let value = core.get_value(&Source::Register(8, RegisterType::$type));
            println!("{}", value);
            Ok(InstructionResult::Continue)
        }
    };
}

macro_rules! generate_print {
    ($name:ident, $type:ident) => {
        fn $name(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
            let value = core.get_value(&Source::Register(8, RegisterType::$type));
            print!("{}", value);
            Ok(InstructionResult::Continue)
        }
    };
}

generate_println!(println_u8, U8);
generate_println!(println_u16, U16);
generate_println!(println_u32, U32);
generate_println!(println_u64, U64);
generate_println!(println_i8, I8);
generate_println!(println_i16, I16);
generate_println!(println_i32, I32);
generate_println!(println_i64, I64);
generate_println!(println_f32, F32);
generate_println!(println_f64, F64);


fn eprintln_string(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
    let reference = core.get_value(&Source::Register(8, RegisterType::Reference));
    match reference {
        Value::MemoryRef(ref_) => {
            let string = memory.get_string(ref_, &module).unwrap();
            eprintln!("{}", string);
        },
        _ => {}
    }

    Ok(InstructionResult::Continue)
}

fn eprint_string(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
    let reference = core.get_value(&Source::Register(8, RegisterType::Reference));
    match reference {
        Value::MemoryRef(ref_) => {
            let string = memory.get_string(ref_, &module).unwrap();
            eprint!("{}", string);
        },
        _ => {}
    }

    Ok(InstructionResult::Continue)
}

macro_rules! generate_eprintln {
    ($name:ident, $type:ident) => {
        fn $name(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
            let value = core.get_value(&Source::Register(8, RegisterType::$type));
            eprintln!("{}", value);
            Ok(InstructionResult::Continue)
        }
    };
}

macro_rules! generate_eprint {
    ($name:ident, $type:ident) => {
        fn $name(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
            let value = core.get_value(&Source::Register(8, RegisterType::$type));
            eprint!("{}", value);
            Ok(InstructionResult::Continue)
        }
    };
}

generate_eprintln!(eprintln_u8, U8);
generate_eprintln!(eprintln_u16, U16);
generate_eprintln!(eprintln_u32, U32);
generate_eprintln!(eprintln_u64, U64);
generate_eprintln!(eprintln_i8, I8);
generate_eprintln!(eprintln_i16, I16);
generate_eprintln!(eprintln_i32, I32);
generate_eprintln!(eprintln_i64, I64);
generate_eprintln!(eprintln_f32, F32);
generate_eprintln!(eprintln_f64, F64);