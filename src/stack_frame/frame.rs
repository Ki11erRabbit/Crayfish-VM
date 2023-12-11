use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use smallvec::SmallVec;
use crate::instruction::Instruction;
use crate::machine::Register;
use crate::program::function::FunctionPath;
use crate::stack_frame::{REGISTER_COUNT, ReturnAddress, StackFrame};
use crate::stack_frame::delimited_continuation::DelimitedContinuation;
use crate::value::{Value, ValueType};


#[derive(Clone)]
pub struct FrameInfo {
    function_name: FunctionPath,
    instructions: Arc<[Instruction]>,
    program_counter: usize,
}


#[derive(Clone)]
pub struct Frame {
    frame_info: FrameInfo,
    return_address: Option<ReturnAddress>,
    stack: SmallVec<[u8; 128]>,
    stack_pointer: usize,
    call_backup: Option<[Register; REGISTER_COUNT]>,
    gc_backup: Option<[Register; REGISTER_COUNT]>,
}

impl Frame {
    pub fn new(function_name: FunctionPath, instructions: Arc<[Instruction]>) -> Self {
        Frame {
            frame_info: FrameInfo {
                function_name: function_name.into(),
                instructions,
                program_counter: 0,
            },
            return_address: None,
            stack: SmallVec::new(),
            stack_pointer: 0,
            call_backup: None,
            gc_backup: None,
        }
    }
}


impl StackFrame for Frame {
    fn push(&mut self, value: Value) {
        let bytes: Box<[u8]> = value.into();
        if self.stack_pointer + bytes.len() > self.stack.len() {
            self.stack.resize(self.stack_pointer + bytes.len(), 0);
        }
        self.stack[self.stack_pointer..self.stack_pointer + bytes.len()].copy_from_slice(&bytes);
    }

    fn pop(&mut self, size: ValueType, mut return_value: &mut Value) {
        match size {
            ValueType::U8 => {
                let value = self.stack[self.stack_pointer];
                self.stack_pointer += 1;
                *return_value = value.into()
            },
            ValueType::U16 => {
                let value = u16::from_le_bytes([self.stack[self.stack_pointer], self.stack[self.stack_pointer + 1]]);
                self.stack_pointer += 2;
                *return_value = value.into()
            },
            ValueType::U32 => {
                let value = u32::from_le_bytes([self.stack[self.stack_pointer], self.stack[self.stack_pointer + 1], self.stack[self.stack_pointer + 2], self.stack[self.stack_pointer + 3]]);
                self.stack_pointer += 4;
                *return_value = value.into()
            },
            ValueType::U64 => {
                let value = u64::from_le_bytes([self.stack[self.stack_pointer], self.stack[self.stack_pointer + 1], self.stack[self.stack_pointer + 2], self.stack[self.stack_pointer + 3], self.stack[self.stack_pointer + 4], self.stack[self.stack_pointer + 5], self.stack[self.stack_pointer + 6], self.stack[self.stack_pointer + 7]]);
                self.stack_pointer += 8;
                *return_value = value.into()
            },
            ValueType::I8 => {
                let value = i8::from_le_bytes([self.stack[self.stack_pointer]]);
                self.stack_pointer += 1;
                *return_value = value.into()
            },
            ValueType::I16 => {
                let value = i16::from_le_bytes([self.stack[self.stack_pointer], self.stack[self.stack_pointer + 1]]);
                self.stack_pointer += 2;
                *return_value = value.into()
            },
            ValueType::I32 => {
                let value = i32::from_le_bytes([self.stack[self.stack_pointer], self.stack[self.stack_pointer + 1], self.stack[self.stack_pointer + 2], self.stack[self.stack_pointer + 3]]);
                self.stack_pointer += 4;
                *return_value = value.into()
            },
            ValueType::I64 => {
                let value = i64::from_le_bytes([self.stack[self.stack_pointer], self.stack[self.stack_pointer + 1], self.stack[self.stack_pointer + 2], self.stack[self.stack_pointer + 3], self.stack[self.stack_pointer + 4], self.stack[self.stack_pointer + 5], self.stack[self.stack_pointer + 6], self.stack[self.stack_pointer + 7]]);
                self.stack_pointer += 8;
                *return_value = value.into()
            },
            ValueType::F32 => {
                let value = f32::from_le_bytes([self.stack[self.stack_pointer], self.stack[self.stack_pointer + 1], self.stack[self.stack_pointer + 2], self.stack[self.stack_pointer + 3]]);
                self.stack_pointer += 4;
                *return_value = value.into()
            },
            ValueType::F64 => {
                let value = f64::from_le_bytes([self.stack[self.stack_pointer], self.stack[self.stack_pointer + 1], self.stack[self.stack_pointer + 2], self.stack[self.stack_pointer + 3], self.stack[self.stack_pointer + 4], self.stack[self.stack_pointer + 5], self.stack[self.stack_pointer + 6], self.stack[self.stack_pointer + 7]]);
                self.stack_pointer += 8;
                *return_value = value.into()
            },
            _ => panic!("Invalid register size"),
        }
    }

    fn backup_registers(&mut self, registers: &[Register; REGISTER_COUNT]) {
        self.call_backup = Some(*registers);

    }

    fn restore_registers(&mut self, registers: &mut [Register; REGISTER_COUNT]) {
        if let Some(call_backup) = &self.call_backup {
            registers[8..].copy_from_slice(call_backup);
        }
    }

    fn backup_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]) {
        self.gc_backup = Some((*registers).clone());

    }

    fn restore_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]) {
        if let Some(gc_backup) = &self.gc_backup {
            registers.copy_from_slice(gc_backup);
        }
    }

    fn set_return_address(&mut self, return_address: ReturnAddress) {
        self.return_address = Some(return_address);
    }

    fn create_return_address(&self) -> ReturnAddress {
        ReturnAddress {
            program_counter: self.frame_info.program_counter,
            function_name: self.frame_info.function_name.clone(),
        }
    }

    fn get_function_name(&self) -> FunctionPath {
        self.frame_info.function_name.clone()
    }

    fn set_function_name(&mut self, name: &str) {
        self.frame_info.function_name = name.into();
    }

    fn get_instruction(&self) -> Instruction {
        self.frame_info.instructions[self.frame_info.program_counter].clone()
    }

    fn get_instructions(&self) -> Arc<[Instruction]> {
        self.frame_info.instructions.clone()
    }

    fn increment_program_counter(&mut self) {
        self.frame_info.program_counter += 1;
    }

    fn reset_program_counter(&mut self) {
        self.frame_info.program_counter = 0;
    }

    fn get_program_counter(&self) -> usize {
        self.frame_info.program_counter
    }

    fn set_program_counter(&mut self, program_counter: usize) {
        self.frame_info.program_counter = program_counter;
    }

    fn make_continuation(&self) -> DelimitedContinuation {
        DelimitedContinuation::new(Rc::new(RefCell::new((*self).clone())), self.frame_info.program_counter)
    }
}