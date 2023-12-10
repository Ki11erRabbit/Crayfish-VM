use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use crate::instruction::Instruction;
use crate::machine::Register;
use crate::stack_frame::{REGISTER_COUNT, ReturnAddress, StackFrame};
use crate::value::Value;

pub struct DelimitedContinuation {
    stack_frame: Rc<RefCell<dyn StackFrame>>,
    start_program_counter: usize,
}




impl StackFrame for DelimitedContinuation {
    fn push(&mut self, value: Value) {
        todo!()
    }

    fn pop(&mut self, size: u8) -> Value {
        todo!()
    }

    fn backup_registers(&mut self, registers: &[Register; REGISTER_COUNT]) {
        todo!()
    }

    fn restore_registers(&mut self, registers: &mut [Register; REGISTER_COUNT]) {
        todo!()
    }

    fn backup_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]) {
        todo!()
    }

    fn restore_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]) {
        todo!()
    }

    fn set_return_address(&mut self, return_address: ReturnAddress) {
        todo!()
    }

    fn create_return_address(&self) -> ReturnAddress {
        todo!()
    }

    fn get_function_name(&self) -> Box<str> {
        todo!()
    }

    fn set_function_name(&mut self, name: &str) {
        todo!()
    }

    fn get_instruction(&self) -> Instruction {
        todo!()
    }

    fn get_instructions(&self) -> Arc<[Instruction]> {
        todo!()
    }

    fn increment_program_counter(&mut self) {
        todo!()
    }

    fn reset_program_counter(&mut self) {
        todo!()
    }

    fn get_program_counter(&self) -> usize {
        todo!()
    }

    fn set_program_counter(&mut self, program_counter: usize) {
        todo!()
    }

    fn make_continuation(&self) -> DelimitedContinuation {
        todo!()
    }

    fn is_continuation(&self) -> bool {
        true
    }
}