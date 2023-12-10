pub mod frame;
pub mod delimited_continuation;

use std::sync::Arc;
use crate::instruction::Instruction;
use crate::machine::Register;
use crate::stack_frame::delimited_continuation::DelimitedContinuation;
use crate::value::Value;

pub const REGISTER_COUNT: usize = 32;



pub struct ReturnAddress {
    program_counter: usize,
    function_name: Box<str>,
}







pub trait StackFrame {
    fn push(&mut self, value: Value);
    fn pop(&mut self, size: u8) -> Value;

    fn backup_registers(&mut self, registers: &[Register; REGISTER_COUNT]);

    fn restore_registers(&mut self, registers: &mut [Register; REGISTER_COUNT]);

    fn backup_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]);

    fn restore_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]);
    fn set_return_address(&mut self, return_address: ReturnAddress);

    fn create_return_address(&self) -> ReturnAddress;

    fn get_function_name(&self) -> Box<str>;

    fn set_function_name(&mut self, name: &str);

    fn get_instruction(&self) -> Instruction;

    fn get_instructions(&self) -> Arc<[Instruction]>;

    fn increment_program_counter(&mut self);

    fn reset_program_counter(&mut self);

    fn get_program_counter(&self) -> usize;

    fn set_program_counter(&mut self, program_counter: usize);

    fn make_continuation(&self) -> DelimitedContinuation;

    fn is_continuation(&self) -> bool {
        false
    }





}