use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;
use crate::instruction::Instruction;
use crate::machine::{Fault, Register};
use crate::program::function::FunctionPath;
use crate::stack_frame::{REGISTER_COUNT, ReturnAddress, StackFrame};
use crate::value::{Value, ValueType};



pub struct ContinuationStore {
    continuations: Vec<DelimitedContinuation>,
}

impl ContinuationStore {
    pub fn new() -> Self {
        ContinuationStore {
            continuations: Vec::new()
        }
    }

    pub fn store(&mut self, continuation: DelimitedContinuation) {
        self.continuations.push(continuation);
    }

    pub fn get(&mut self, index: usize) -> Option<DelimitedContinuation> {
        self.continuations.get(index).map(|c| c.clone())
    }

    pub fn get_last_index(&mut self) -> u64 {
        self.continuations.len() as u64 - 1
    }

}


#[derive(Clone)]
pub struct DelimitedContinuation {
    stack_frame: Rc<RefCell<dyn StackFrame>>,
    start_program_counter: usize,
}


impl DelimitedContinuation {
    pub fn new(stack_frame: Rc<RefCell<dyn StackFrame>>, start_program_counter: usize) -> Self {
        let stack_frame = stack_frame.clone();
        DelimitedContinuation {
            stack_frame,
            start_program_counter,
        }
    }

    pub fn get_stack_frame(&self) -> Rc<RefCell<dyn StackFrame>> {
        self.stack_frame.clone()
    }

    pub fn get_start_program_counter(&self) -> usize {
        self.start_program_counter
    }

}

impl Debug for DelimitedContinuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DelimitedContinuation")
            .field("start_program_counter", &self.start_program_counter)
            .finish()
    }
}


impl StackFrame for DelimitedContinuation {
    fn push(&mut self, value: Value) {
        self.stack_frame.borrow_mut().push(value)
    }

    fn pop(&mut self, size: ValueType) -> Value {
        let mut stack_frame = self.stack_frame.borrow_mut();
        stack_frame.pop(size)
    }

    fn get_value(&self, offset: Value, size: ValueType) -> Value {
        self.stack_frame.borrow().get_value(offset, size)
    }

    fn set_value(&mut self, offset: Value, value: Value) -> Result<(), Fault> {
        self.stack_frame.borrow_mut().set_value(offset, value)
    }

    fn backup_registers(&mut self, registers: &[Register; REGISTER_COUNT]) {
        self.stack_frame.borrow_mut().backup_registers(registers)
    }

    fn restore_registers(&mut self, registers: &mut [Register; REGISTER_COUNT]) {
        self.stack_frame.borrow_mut().restore_registers(registers)
    }

    fn backup_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]) {
        self.stack_frame.borrow_mut().backup_registers_for_gc(registers)
    }

    fn restore_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]) {
        self.stack_frame.borrow_mut().restore_registers_for_gc(registers)
    }


    fn get_function_name(&self) -> FunctionPath {
        self.stack_frame.borrow().get_function_name()
    }

    fn set_function_name(&mut self, name: &str) {
        self.stack_frame.borrow_mut().set_function_name(name)
    }

    fn get_instruction(&self) -> Instruction {
        self.stack_frame.borrow().get_instruction()
    }

    fn get_instructions(&self) -> Arc<[Instruction]> {
        self.stack_frame.borrow().get_instructions()
    }

    fn increment_program_counter(&mut self) {
        self.stack_frame.borrow_mut().increment_program_counter()
    }

    fn reset_program_counter(&mut self) {
        self.stack_frame.borrow_mut().set_program_counter(self.start_program_counter)
    }

    fn get_program_counter(&self) -> usize {
        self.stack_frame.borrow().get_program_counter()
    }

    fn set_program_counter(&mut self, program_counter: usize) {
        self.stack_frame.borrow_mut().set_program_counter(program_counter)
    }

    fn make_continuation(&self) -> DelimitedContinuation {
        (*self).clone()
    }

    fn is_continuation(&self) -> bool {
        true
    }
}