pub mod frame;
pub mod delimited_continuation;

use std::sync::Arc;
use crate::instruction::Instruction;
use crate::machine::{Fault, Register};
use crate::program::function::FunctionPath;
use crate::stack_frame::delimited_continuation::DelimitedContinuation;
use crate::value::{Value, ValueType};

pub const REGISTER_COUNT: usize = 32;



#[derive(Clone, Debug)]
pub struct ReturnAddress {
    program_counter: usize,
    function_name: FunctionPath,
}







pub trait StackFrame {
    fn push(&mut self, value: Value);
    fn pop(&mut self, size: ValueType) -> Value;

    fn get_value(&self, offset: Value, size: ValueType) -> Value;
    fn set_value(&mut self, offset: Value, value: Value) -> Result<(),Fault>;

    fn backup_registers(&mut self, registers: &[Register; REGISTER_COUNT]);

    fn restore_registers(&mut self, registers: &mut [Register; REGISTER_COUNT]);

    fn backup_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]);

    fn restore_registers_for_gc(&mut self, registers: &mut [Register; REGISTER_COUNT]);


    fn get_function_name(&self) -> FunctionPath;

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