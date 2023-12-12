use std::sync::Arc;
use crate::backtrace::BacktraceInfo;
use crate::instruction::{CallTarget, ComparisonType, Condition, Immediate, Instruction, JumpTarget, RegisterType, Source, Target};
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


fn dp_fib() -> Arc<[Instruction]> {
    use Instruction::*;
    Arc::new([
        Load(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(0))),
        Load(Target(2, RegisterType::U64), Source::Immediate(Immediate::U64(1))),
        Load(Target(3, RegisterType::U64), Source::Immediate(Immediate::U64(2))),
        Compare(Target(3, RegisterType::U64), Source::Immediate(Immediate::U64(10)), ComparisonType::Equal),
        Return(Condition::Equal),
        Add(Target(4, RegisterType::U64), Source::Register(1, RegisterType::U64), false, false),
        Add(Target(4, RegisterType::U64), Source::Register(2, RegisterType::U64), false, false),
        Load(Target(1, RegisterType::U64), Source::Register(2, RegisterType::U64)),
        Load(Target(2, RegisterType::U64), Source::Register(4, RegisterType::U64)),
        Sub(Target(4, RegisterType::U64), Source::Register(4, RegisterType::U64), false, false),
        Add(Target(3, RegisterType::U64), Source::Immediate(Immediate::U64(1)), false, false),
        Goto(JumpTarget::Relative(-8), Condition::Always),
    ])
}

fn hello_world_main() -> Arc<[Instruction]> {
    use Instruction::*;
    Arc::new([
        Call(CallTarget::Label("hello_world".into()), Condition::Always),
        Return(Condition::Always),
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
    use Instruction::*;
    Arc::new([
        GetStringRef(Target(8, RegisterType::U64), "".into(), 0),
        Call(CallTarget::Label("print_string".into()), Condition::Always),
        Return(Condition::Always),
    ])
}


fn rec_fib() -> Arc<[Instruction]> {
    use Instruction::*;
    Arc::new([
        Compare(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(1)), ComparisonType::GreaterThan),
        Goto(JumpTarget::Relative(3), Condition::GreaterThan),
        Load(Target(0, RegisterType::U64), Source::Register(1, RegisterType::U64)),
        Return(Condition::Always),
        Sub(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(1)), false, false),
        Push(Source::Register(1, RegisterType::U64)),
        Call(CallTarget::Label("fib".into()), Condition::Always),
        Pop(Target(1, RegisterType::U64)),
        Sub(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(1)), false, false),
        Push(Source::Register(0, RegisterType::U64)),
        Call(CallTarget::Label("fib".into()), Condition::Always),
        Pop(Target(2, RegisterType::U64)),
        Add(Target(0, RegisterType::U64), Source::Register(2, RegisterType::U64), false, false),
        Return(Condition::Always),
    ])
}

fn rec_fib_main() -> Arc<[Instruction]> {
    use Instruction::*;
    Arc::new([
        Load(Target(1, RegisterType::U64), Source::Immediate(Immediate::U64(10))),
        Call(CallTarget::Label("fib".into()), Condition::Always),
        Return(Condition::Always),
    ])
}


fn main() {
    let mut module = Module::default();
    module.add_function(&"main".into(), Function::ByteCode(dp_fib()));
    module.add_function(&"main".into(), Function::ByteCode(hello_world_main()));
    module.add_function(&"hello_world".into(), Function::Native(hello_world));
    module.add_function(&"print_string".into(), Function::Native(print_string));
    module.add_function(&"main".into(), Function::ByteCode(print_string_main()));
    module.add_function(&"fib".into(), Function::ByteCode(rec_fib()));
    module.add_function(&"main".into(), Function::ByteCode(rec_fib_main()));
    module.add_string(&"".into(), "Hello, world!");

    let mut core = Core::default();
    let module = Arc::new(module);

    let mut memory = Memory::new();
    module.add_strings_to_memory(&mut memory);
    let mut backtrace = BacktraceInfo::new();

    match call_main(&mut core, module, memory, &mut backtrace) {
        Ok(_) => {
            println!("Program finished successfully");
        },
        Err(fault) => {
            println!("Program faulted: {:?}", fault);
            println!("{}", backtrace);
        }
    }

    println!("{}", core);
}
