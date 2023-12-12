use std::sync::Arc;
use crate::backtrace::BacktraceInfo;
use crate::instruction::{CallTarget, ComparisonType, Condition, Immediate, Instruction, JumpTarget, RegisterType, Source, Target};
use crate::instruction::RealInstruction;
use crate::machine::{call_main, Fault, InstructionResult};
use crate::machine::core::{Core, CoreUtils};
use crate::memory::Memory;
use crate::program::function::Function;
use crate::program::Module;
use crate::stack_frame::delimited_continuation::ContinuationStore;
use crate::stack_frame::StackFrame;
use crate::value::Value;

pub mod instruction;
pub mod stack_frame;
pub mod value;
pub mod machine;
pub mod program;
pub mod memory;
mod backtrace;
mod native_lib;


fn dp_fib() -> Arc<[Instruction]> {
    use RealInstruction::*;
    Arc::new([
        Instruction::new_without_metadata(Load(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(0)))),
        Instruction::new_without_metadata(Load(Target(2, RegisterType::U64), Source::Immediate(Immediate::U64(1)))),
        Instruction::new_without_metadata(Load(Target(3, RegisterType::U64), Source::Immediate(Immediate::U64(2)))),
        Instruction::new_without_metadata(Compare(Target(3, RegisterType::U64), Source::Immediate(Immediate::U64(10)), ComparisonType::Equal)),
        Instruction::new_without_metadata(Goto(JumpTarget::Relative(8), Condition::Equal)),
        Instruction::new_without_metadata(Add(Target(4, RegisterType::U64), Source::Register(1, RegisterType::U64), false, false)),
        Instruction::new_without_metadata(Add(Target(4, RegisterType::U64), Source::Register(2, RegisterType::U64), false, false)),
        Instruction::new_without_metadata(Load(Target(1, RegisterType::U64), Source::Register(2, RegisterType::U64))),
        Instruction::new_without_metadata(Load(Target(2, RegisterType::U64), Source::Register(4, RegisterType::U64))),
        Instruction::new_without_metadata(Sub(Target(4, RegisterType::U64), Source::Register(4, RegisterType::U64), false, false)),
        Instruction::new_without_metadata(Add(Target(3, RegisterType::U64), Source::Immediate(Immediate::U64(1)), false, false)),
        Instruction::new_without_metadata(Goto(JumpTarget::Relative(-8), Condition::Always)),
        Instruction::new_without_metadata(GetStringRef(Target(8, RegisterType::U64), "".into(), 0)),
        Instruction::new_without_metadata(Call(CallTarget::Label("std::io::println_string".into()), Condition::Always)),
        Instruction::new_without_metadata(Load(Target(8, RegisterType::U64), Source::Register(2, RegisterType::U64))),
        Instruction::new_without_metadata(Call(CallTarget::Label("std::io::println_u64".into()), Condition::Always)),
        Instruction::new_without_metadata(Return(Condition::Always)),
    ])
}

fn hello_world_main() -> Arc<[Instruction]> {
    use RealInstruction::*;
    Arc::new([
        Instruction::new_without_metadata(Call(CallTarget::Label("hello_world".into()), Condition::Always)),
        Instruction::new_without_metadata(Return(Condition::Always)),
    ])
}

fn hello_world(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
    println!("Hello, world!");
    Ok(InstructionResult::Continue)
}

fn print_string(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {

    let reference = core.get_value(&Source::Register(8, RegisterType::Reference));
    match reference {
        Value::MemoryRef(refe) => {
            let string = memory.get_string(refe, &module).unwrap();
            println!("{}", string);
        }
        _ => {
            return Err(Fault::InvalidString);
        }
    }
    Ok(InstructionResult::Continue)
}

fn print_string_main() -> Arc<[Instruction]> {
    use RealInstruction::*;
    Arc::new([
        Instruction::new_without_metadata(GetStringRef(Target(8, RegisterType::U64), "".into(), 0)),
        Instruction::new_without_metadata(Call(CallTarget::Label("print_string".into()), Condition::Always)),
        Instruction::new_without_metadata(Return(Condition::Always)),
    ])
}


fn rec_fib() -> Arc<[Instruction]> {
    use RealInstruction::*;
    Arc::new([
        Instruction::new_without_metadata(Compare(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(1)), ComparisonType::GreaterThan)),
        Instruction::new_without_metadata(Goto(JumpTarget::Relative(3), Condition::GreaterThan)),
        Instruction::new_without_metadata(Load(Target(0, RegisterType::U64), Source::Register(1, RegisterType::U64))),
        Instruction::new_without_metadata(Return(Condition::Always)),
        Instruction::new_without_metadata(Sub(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(1)), false, false)),
        Instruction::new_without_metadata(Push(Source::Register(1, RegisterType::U64))),
        Instruction::new_without_metadata(Call(CallTarget::Label("fib".into()), Condition::Always)),
        Instruction::new_without_metadata(Pop(Target(1, RegisterType::U64))),
        Instruction::new_without_metadata(Sub(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(1)), false, false)),
        Instruction::new_without_metadata(Push(Source::Register(0, RegisterType::U64))),
        Instruction::new_without_metadata(Call(CallTarget::Label("fib".into()), Condition::Always)),
        Instruction::new_without_metadata(Pop(Target(2, RegisterType::U64))),
        Instruction::new_without_metadata(Add(Target(0, RegisterType::U64), Source::Register(2, RegisterType::U64), false, false)),
        Instruction::new_without_metadata(Return(Condition::Always)),
    ])
}


fn rec_fib_main() -> Arc<[Instruction]> {
    use RealInstruction::*;
    Arc::new([
        Instruction::new_without_metadata(Load(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(10)))),
        Instruction::new_without_metadata(Call(CallTarget::Label("fib".into()), Condition::Always)),
        Instruction::new_without_metadata(Return(Condition::Always)),
    ])
}

fn print_io_mod() -> Arc<[Instruction]> {
    use RealInstruction::*;
    Arc::new([
        Instruction::new_without_metadata(GetStringRef(Target(8, RegisterType::U64), "".into(), 0)),
        Instruction::new_without_metadata(Call(CallTarget::Label("std::io::println_string".into()), Condition::Always)),
        Instruction::new_without_metadata(Return(Condition::Always)),
    ])
}

fn main() {
    let mut module = Module::default();
    module.add_sub_module(native_lib::get_std_module());
    module.add_function("main", Function::ByteCode(hello_world_main()));
    module.add_function("hello_world", Function::Native(hello_world));
    module.add_function("print_string", Function::Native(print_string));
    module.add_function("main", Function::ByteCode(print_string_main()));
    module.add_function("fib", Function::ByteCode(rec_fib()));
    module.add_function("main", Function::ByteCode(rec_fib_main()));
    module.add_function("main", Function::ByteCode(print_io_mod()));
    module.add_function("main", Function::ByteCode(dp_fib()));
    module.add_string(&"".into(), "Hello, world!");

    let mut core = Core::default();
    let module = Arc::new(module);

    let mut memory = Memory::new();
    module.add_strings_to_memory(&mut memory);
    let mut backtrace = BacktraceInfo::new();

    match call_main(&mut core, module, memory, &mut backtrace) {
        Ok(_) => {
            //println!("Program finished successfully");
        },
        Err(fault) => {
            println!("Program faulted: {:?}", fault);
            println!("{}", backtrace);
        }
    }

    //println!("{}", core);
}
