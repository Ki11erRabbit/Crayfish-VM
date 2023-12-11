use std::sync::Arc;
use crate::instruction::{ComparisonType, Condition, Immediate, Instruction, JumpTarget, RegisterType, Source, Target};
use crate::machine::{call_main, Fault, InstructionResult};
use crate::machine::core::Core;
use crate::memory::Memory;
use crate::program::function::Function;
use crate::program::Module;
use crate::stack_frame::delimited_continuation::ContinuationStore;
use crate::stack_frame::StackFrame;

pub mod instruction;
pub mod stack_frame;
pub mod value;
pub mod machine;
pub mod program;
pub mod memory;


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

fn hello_world(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*const dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
    println!("Hello, world!");
    Ok(InstructionResult::Continue)
}

fn main() {
    let mut module = Module::default();

    module.add_function(&"main".into(), Function::ByteCode(dp_fib()));
    module.add_function(&"main".into(), Function::Native(hello_world));

    let mut core = Core::default();
    let module = Arc::new(module);

    call_main(&mut core, module).expect("Fault");

    //println!("{}", core);
}
