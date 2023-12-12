use std::collections::HashMap;
use std::sync::Arc;
use crate::backtrace::BacktraceInfo;
use crate::machine::core::Core;
use crate::machine::{call_bytecode_function, call_native_function, Fault, InstructionResult};
use crate::memory::Memory;
use crate::program::function::Function;
use crate::program::Module;
use crate::stack_frame::delimited_continuation::ContinuationStore;
use crate::stack_frame::StackFrame;

pub fn get_threading_module() -> Module {
    let mut module = Module::new("thread", HashMap::new(), Vec::new(), HashMap::new());


    module
}




fn spawn_thread(core: &mut Core, module: Arc<Module>, stack_frames: &mut Vec<*mut dyn StackFrame>, memory: Memory, continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {


    let new_core = core.clone();

    let thread = std::thread::spawn(move || {
        let mut core = new_core;
        //let mut stack_frames = Vec::new();
        let mut continuation_store = ContinuationStore::new();
        let mut memory = memory.clone();
        let mut backtrace = BacktraceInfo::new();

        /*let function = memory.get_function()

        let result = match function {
            Function::ByteCode(_) => call_bytecode_function(core, frame, module, &mut stack_frames, memory, &mut continuation_store, &mut backtrace)?,
            Function::Native(native) => call_native_function(native, core, frame, module, &mut stack_frames, memory, &mut continuation_store, &mut backtrace)?,
        };

        match result {
            InstructionResult::Stop => {
                return Ok(())
            }
            InstructionResult::Continue => {
                return Ok(())
            }
            InstructionResult::Return => {
                return Ok(())
            }
            InstructionResult::Unwind(effect) => {
                todo!("Add code for error message for letting an effect escape beyond the main function")
            }
            _ => panic!("Invalid instruction result"),
        }*/


    });
    Ok(InstructionResult::Continue)

}

