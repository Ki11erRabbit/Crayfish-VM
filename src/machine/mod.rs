use std::fmt::Display;
use std::sync::Arc;
use crate::backtrace::{BacktraceEntry, BacktraceInfo};
use crate::machine::core::Core;
use crate::memory::Memory;
use crate::program::function::{Function, FunctionPath, NativeFunction};
use crate::program::Module;
use crate::stack_frame::frame::Frame;
use crate::value::{Value, ValueType};
use crate::stack_frame::delimited_continuation::{ContinuationStore, DelimitedContinuation};
use crate::stack_frame::StackFrame;

pub mod core;


/// The result of executing an instruction.
#[derive(Debug)]
pub enum InstructionResult {
    Stop,
    Continue,
    Return,
    Unwind(Box<str>),
    Call(Function, Frame),
    CallContinuation(DelimitedContinuation),
}


#[derive(Debug)]
pub enum Fault {
    DivisionByZero,
    StackOverflow,
    Overflow,
    Underflow,
    InvalidInstruction,
    InvalidRegister,
    InvalidJump,
    FunctionNotFound(FunctionPath),
    ContinuationNotFound(u64),
    InvalidString,
    InvalidOperation(String),
    MemoryError(String),
    InvalidStackLevel,
    InvalidStackOffset,
    StackFrameOutOfBounds,
    InvalidStackIndex,
    StackOutOfBounds,
    NullPointerReference,
    InvalidReference,
    IndexOutOfBounds,
}

#[derive(Debug, Clone)]
pub struct Register {
    pub value: Value,
}

impl Register {
    fn get_value(&self, size: ValueType) -> Value {
        self.value.clone().transmute(size)
    }

    fn set_value(&mut self, value: Value) {
        self.value = value;
        /*match value {
            Value::U8(value) => {
                self.value[0] = value;
                self.value[1] = 0;
                self.value[2] = 0;
                self.value[3] = 0;
                self.value[4] = 0;
                self.value[5] = 0;
                self.value[6] = 0;
                self.value[7] = 0;
            },
            Value::I8(value) => {
                self.value[0] = u8::from_le_bytes(value.to_le_bytes());
                self.value[1] = 0;
                self.value[2] = 0;
                self.value[3] = 0;
                self.value[4] = 0;
                self.value[5] = 0;
                self.value[6] = 0;
                self.value[7] = 0;
            },
            Value::U16(value) => {
                let bytes = value.to_le_bytes();
                self.value[0] = bytes[0];
                self.value[1] = bytes[1];
                self.value[2] = 0;
                self.value[3] = 0;
                self.value[4] = 0;
                self.value[5] = 0;
                self.value[6] = 0;
                self.value[7] = 0;
            },
            Value::I16(value) => {
                let bytes = value.to_le_bytes();
                self.value[0] = bytes[0];
                self.value[1] = bytes[1];
                self.value[2] = 0;
                self.value[3] = 0;
                self.value[4] = 0;
                self.value[5] = 0;
                self.value[6] = 0;
                self.value[7] = 0;
            },
            Value::U32(value) => {
                let bytes = value.to_le_bytes();
                self.value[0] = bytes[0];
                self.value[1] = bytes[1];
                self.value[2] = bytes[2];
                self.value[3] = bytes[3];
                self.value[4] = 0;
                self.value[5] = 0;
                self.value[6] = 0;
                self.value[7] = 0;
            },
            Value::I32(value) => {
                let bytes = value.to_le_bytes();
                self.value[0] = bytes[0];
                self.value[1] = bytes[1];
                self.value[2] = bytes[2];
                self.value[3] = bytes[3];
                self.value[4] = 0;
                self.value[5] = 0;
                self.value[6] = 0;
                self.value[7] = 0;
            },
            Value::U64(value) => {
                let bytes = value.to_le_bytes();
                self.value[0] = bytes[0];
                self.value[1] = bytes[1];
                self.value[2] = bytes[2];
                self.value[3] = bytes[3];
                self.value[4] = bytes[4];
                self.value[5] = bytes[5];
                self.value[6] = bytes[6];
                self.value[7] = bytes[7];
            },
            Value::I64(value) => {
                let bytes = value.to_le_bytes();
                self.value[0] = bytes[0];
                self.value[1] = bytes[1];
                self.value[2] = bytes[2];
                self.value[3] = bytes[3];
                self.value[4] = bytes[4];
                self.value[5] = bytes[5];
                self.value[6] = bytes[6];
                self.value[7] = bytes[7];
            },
            Value::F32(value) => {
                let bytes = value.to_le_bytes();
                self.value[0] = bytes[0];
                self.value[1] = bytes[1];
                self.value[2] = bytes[2];
                self.value[3] = bytes[3];
                self.value[4] = 0;
                self.value[5] = 0;
                self.value[6] = 0;
                self.value[7] = 0;
            },
            Value::F64(value) => {
                let bytes = value.to_le_bytes();
                self.value[0] = bytes[0];
                self.value[1] = bytes[1];
                self.value[2] = bytes[2];
                self.value[3] = bytes[3];
                self.value[4] = bytes[4];
                self.value[5] = bytes[5];
                self.value[6] = bytes[6];
                self.value[7] = bytes[7];
            },
            _ => panic!("Invalid register size"),
        }*/
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            value: Value::U64(0),
        }
    }
}


pub fn call_main(core: &mut Core, module: Arc<Module>, memory: Memory, backtrace: &mut BacktraceInfo) -> Result<(), Fault> {
    let main = module.get_function(&"main".into()).ok_or(Fault::FunctionNotFound("main".into()))?;
    let main_frame = Frame::new("main".into(), main.get_instructions());
    let mut frames = Vec::new();
    let mut continuation_store = ContinuationStore::new();

    backtrace.push(BacktraceEntry::new("main".into(), None, None));

    let result = match main {
        Function::ByteCode(_) => call_bytecode_function(core, main_frame, module, &mut frames, memory, &mut continuation_store, backtrace)?,
        Function::Native(native) => call_native_function(native, core, main_frame, module, &mut frames, memory, &mut continuation_store, backtrace)?,
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
    }
}


pub fn call_bytecode_function(core: &mut Core,
                              mut stack_frame: impl StackFrame + 'static,
                              module: Arc<Module>,
                              frames: &mut Vec<*mut dyn StackFrame>,
                              memory: Memory,
                              continuation_store: &mut ContinuationStore,
                              backtrace: &mut BacktraceInfo) -> Result<InstructionResult,Fault> {
    stack_frame.backup_registers(&core.registers);

    loop {
        let result = core.execute_instruction(&mut stack_frame, &module, frames, memory.clone(), continuation_store, backtrace)?;
        match result {
            InstructionResult::Continue => {},
            InstructionResult::Stop => {
                stack_frame.restore_registers(&mut core.registers);
                frames.pop();
                return Ok(InstructionResult::Stop);
            },
            InstructionResult::Return => {
                stack_frame.restore_registers(&mut core.registers);
                frames.pop();
                backtrace.pop();
                return Ok(InstructionResult::Continue);
            },
            InstructionResult::Unwind(effect_name) => {
                backtrace.increment_unwind_levels();
                todo!("Add code for checking effect to handle before continue unwinding")
            },

            InstructionResult::Call(function, new_stack_frame) => {
                backtrace.push(BacktraceEntry::new(new_stack_frame.get_function_name(), None, None));
                frames.push(&mut stack_frame as *mut dyn StackFrame);
                match function {
                    Function::ByteCode(_) => {
                        let result = call_bytecode_function(core, new_stack_frame, module.clone(), frames, memory.clone(),continuation_store, backtrace)?;
                        match result {
                            InstructionResult::Continue | InstructionResult::Return => {},
                            InstructionResult::Stop => {
                                stack_frame.restore_registers(&mut core.registers);
                                frames.pop();
                                return Ok(InstructionResult::Stop);
                            },
                            InstructionResult::Unwind(effect_name) => {
                                backtrace.increment_unwind_levels();
                                todo!("Add code for checking effect to handle before continue unwinding");
                                continue;
                            },
                            x => panic!("Invalid instruction result: {:?}", x),
                        }
                    },
                    Function::Native(native) => {
                        let result = call_native_function(native, core, new_stack_frame, module.clone(), frames, memory.clone(),continuation_store, backtrace)?;
                        match result {
                            InstructionResult::Continue | InstructionResult::Return => {}
                            InstructionResult::Stop => {
                                stack_frame.restore_registers(&mut core.registers);
                                frames.pop();
                                return Ok(InstructionResult::Stop);
                            },
                            InstructionResult::Unwind(effect_name) => {
                                backtrace.increment_unwind_levels();
                                todo!("Add code for checking effect to handle before continue unwinding");
                                continue;
                            },
                            x => panic!("Invalid instruction result: {:?}", x)
                        }
                    }
                }
                frames.pop();
                backtrace.pop();
            },
            InstructionResult::CallContinuation(continuation) => {
                frames.push(&mut stack_frame as *mut dyn StackFrame);
                backtrace.push(BacktraceEntry::new(continuation.get_function_name(), None, None));
                let result = call_bytecode_function(core, continuation, module.clone(), frames, memory.clone(), continuation_store, backtrace)?;
                frames.pop();
                match result {
                    InstructionResult::Continue => {
                        backtrace.pop();
                    },
                    InstructionResult::Stop => {
                        stack_frame.restore_registers(&mut core.registers);
                        return Ok(InstructionResult::Stop);
                    },
                    InstructionResult::Unwind(effect_name) => {
                        backtrace.increment_unwind_levels();
                        todo!("Add code for checking effect to handle before continue unwinding");
                        continue;
                    },
                    x => panic!("Invalid instruction result: {:?}", x)
                }
            }
        }
    }
    Ok(InstructionResult::Continue)
}

pub fn call_native_function(native_function: NativeFunction,
                              core: &mut Core,
                              mut stack_frame: impl StackFrame + 'static,
                              module: Arc<Module>,
                              frames: &mut Vec<*mut dyn StackFrame>,
                              memory: Memory,
                              continuation_store: &mut ContinuationStore,
                              backtrace: &mut BacktraceInfo) -> Result<InstructionResult,Fault> {
    stack_frame.backup_registers(&core.registers);

    frames.push(&mut stack_frame as *mut dyn StackFrame);


    let result = native_function(core, module, frames, memory.clone(), continuation_store)?;
    frames.pop();
    match result {
        InstructionResult::Continue => {
            stack_frame.restore_registers(&mut core.registers);
            backtrace.pop();
            return Ok(InstructionResult::Continue);
        },
        InstructionResult::Stop => {
            stack_frame.restore_registers(&mut core.registers);
            return Ok(InstructionResult::Stop);
        },
        InstructionResult::Unwind(effect_name) => {
            backtrace.increment_unwind_levels();
            todo!("Add code for checking effect to handle before continue unwinding");
        },
        _ => panic!("Invalid instruction result"),
    }

}

