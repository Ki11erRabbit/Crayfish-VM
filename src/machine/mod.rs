use std::fmt::Display;
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
pub enum InstructionResult {
    Stop,
    Continue,
    Return,
    Unwind(Box<str>),
    Call(FunctionPath),
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
    FunctionNotFound(String),
    ContinuationNotFound(u64),
    InvalidString,
    InvalidOperation(String),
    MemoryError(String),
}

#[derive(Debug, Clone, Copy)]
pub struct Register {
    pub value: [u8; 8],
}

impl Register {
    fn get_value<'a>(&self, size: ValueType) -> Value<'a> {
        match size {
            ValueType::U8 => Value::U8(self.value[0]),
            ValueType::I8 => Value::I8(i8::from_le_bytes([self.value[0]])),
            ValueType::U16 => Value::U16(u16::from_le_bytes([self.value[0], self.value[1]])),
            ValueType::I16 => Value::I16(i16::from_le_bytes([self.value[0], self.value[1]])),
            ValueType::U32 => Value::U32(u32::from_le_bytes([self.value[0], self.value[1], self.value[2], self.value[3]])),
            ValueType::I32 => Value::I32(i32::from_le_bytes([self.value[0], self.value[1], self.value[2], self.value[3]])),
            ValueType::U64 => Value::U64(u64::from_le_bytes([self.value[0], self.value[1], self.value[2], self.value[3], self.value[4], self.value[5], self.value[6], self.value[7]])),
            ValueType::I64 => Value::I64(i64::from_le_bytes([self.value[0], self.value[1], self.value[2], self.value[3], self.value[4], self.value[5], self.value[6], self.value[7]])),
            ValueType::F32 => Value::F32(f32::from_le_bytes([self.value[0], self.value[1], self.value[2], self.value[3]])),
            ValueType::F64 => Value::F64(f64::from_le_bytes([self.value[0], self.value[1], self.value[2], self.value[3], self.value[4], self.value[5], self.value[6], self.value[7]])),
            _ => panic!("Invalid register size"),
        }
    }

    fn set_value(&mut self, value: Value) {
        match value {
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
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = u64::from_le_bytes(self.value);
        write!(f, "{:?}", value)
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            value: [0; 8],
        }
    }
}


pub fn call_main(core: &mut Core, module: &Module) -> Result<(), Fault> {
    let main = module.get_function(&"main".into()).ok_or(Fault::FunctionNotFound("main".to_string()))?;
    let main_frame = Frame::new("main".into(), main.get_instructions());
    let mut frames = Vec::new();
    let memory = Memory::new();
    let mut continuation_store = ContinuationStore::new();

    let result = match main {
        Function::ByteCode(_) => call_bytecode_function(core, main_frame, module, &mut frames, memory, &mut continuation_store)?,
        Function::Native(native) => call_native_function(native, core, main_frame, module, &mut frames, memory, &mut continuation_store)?,
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
            todo!("Add code for error message for letting an effect escape the main function")
        }
        _ => panic!("Invalid instruction result"),
    }
}


pub fn call_bytecode_function(core: &mut Core,
                              mut stack_frame: impl StackFrame + 'static,
                              module: &Module,
                              frames: &mut Vec<*const dyn StackFrame>,
                              memory: Memory,
                              continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
    stack_frame.backup_registers(&core.registers);

    loop {
        let result = core.execute_instruction(&mut stack_frame, module, memory.clone(), continuation_store)?;
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
                return Ok(InstructionResult::Return);
            },
            InstructionResult::Unwind(effect_name) => {
                todo!("Add code for checking effect to handle before continue unwinding")
            },
            InstructionResult::Call(path) => {
                let function = module.get_function(&path).ok_or(Fault::FunctionNotFound(path.to_string()))?;
                let new_stack_frame = Frame::new(path, function.get_instructions());
                frames.push(&stack_frame as *const dyn StackFrame);
                match function {
                    Function::ByteCode(_) => {
                        let result = call_bytecode_function(core, new_stack_frame, module, frames, memory.clone(),continuation_store)?;
                        match result {
                            InstructionResult::Continue => {},
                            InstructionResult::Stop => {
                                stack_frame.restore_registers(&mut core.registers);
                                frames.pop();
                                return Ok(InstructionResult::Stop);
                            },
                            InstructionResult::Unwind(effect_name) => {
                                todo!("Add code for checking effect to handle before continue unwinding")
                            },
                            _ => panic!("Invalid instruction result"),
                        }
                    },
                    Function::Native(native) => {
                        let result = call_native_function(native, core, new_stack_frame, module, frames, memory.clone(),continuation_store)?;
                        match result {
                            InstructionResult::Continue => {}
                            InstructionResult::Stop => {
                                stack_frame.restore_registers(&mut core.registers);
                                frames.pop();
                                return Ok(InstructionResult::Stop);
                            },
                            InstructionResult::Unwind(effect_name) => {
                                todo!("Add code for checking effect to handle before continue unwinding")
                            },
                            _ => panic!("Invalid instruction result"),
                        }
                    }
                }
                frames.pop();
            },
            InstructionResult::CallContinuation(continuation) => {
                frames.push(&stack_frame as *const dyn StackFrame);

                let result = call_bytecode_function(core, continuation, module, frames, memory.clone(), continuation_store)?;
                frames.pop();
                match result {
                    InstructionResult::Continue => {},
                    InstructionResult::Stop => {
                        stack_frame.restore_registers(&mut core.registers);
                        return Ok(InstructionResult::Stop);
                    },
                    InstructionResult::Unwind(effect_name) => {
                        todo!("Add code for checking effect to handle before continue unwinding")
                    },
                    _ => panic!("Invalid instruction result"),
                }
            }
        }
    }
    Ok(InstructionResult::Continue)
}

pub fn call_native_function(native_function: NativeFunction,
                              core: &mut Core,
                              mut stack_frame: impl StackFrame + 'static,
                              module: &Module,
                              frames: &mut Vec<*const dyn StackFrame>,
                              memory: Memory,
                              continuation_store: &mut ContinuationStore) -> Result<InstructionResult,Fault> {
    stack_frame.backup_registers(&core.registers);

    frames.push(&stack_frame as *const dyn StackFrame);


    let result = native_function(core, module, frames, memory.clone(), continuation_store)?;
    frames.pop();
    match result {
        InstructionResult::Continue => {
            stack_frame.restore_registers(&mut core.registers);
            return Ok(InstructionResult::Continue);
        },
        InstructionResult::Stop => {
            stack_frame.restore_registers(&mut core.registers);
            return Ok(InstructionResult::Stop);
        },
        InstructionResult::Unwind(effect_name) => {
            todo!("Add code for checking effect to handle before continue unwinding")
        },
        _ => panic!("Invalid instruction result"),
    }

}

