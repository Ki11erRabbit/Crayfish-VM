use std::fmt::Display;
use std::sync::Arc;
use crate::instruction::{CallTarget, ComparisonType, Condition, Instruction, JumpTarget, RegisterType, Source, Target};
use crate::machine::{Fault, InstructionResult, Register};
use crate::memory::Memory;
use crate::program::Module;
use crate::stack_frame::{REGISTER_COUNT, StackFrame};
use crate::stack_frame::delimited_continuation::ContinuationStore;
use crate::stack_frame::frame::Frame;
use crate::value::{Value, ValueType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comparison {
    None,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
pub struct CoreFlags {
    comparison: Comparison,
    carry: bool,
    negative: bool,
    zero: bool,
}

impl Default for CoreFlags {
    fn default() -> Self {
        CoreFlags {
            comparison: Comparison::None,
            carry: false,
            negative: false,
            zero: false,
        }
    }
}

impl Display for Core {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Flags:")?;
        writeln!(f, "Comparison: {:?}", self.flags.comparison)?;
        writeln!(f, "Carry: {:?}", self.flags.carry)?;
        writeln!(f, "Zero: {:?}", self.flags.zero)?;
        writeln!(f, "Negative: {:?}", self.flags.negative)?;
        writeln!(f, "Registers:")?;
        for (index, register) in self.registers.iter().enumerate() {
            writeln!(f, "Register {}: {}", index, register)?;
        }
        Ok(())
    }
}

impl Default for Core {
    fn default() -> Self {
        Core {
            flags: CoreFlags::default(),
            registers: [Register::default(); REGISTER_COUNT],
        }
    }
}

trait CoreUtils<T> {
    fn get_value(&self, source: T) -> Value;
}

impl CoreUtils<&Source> for Core {
    fn get_value(&self, source: &Source) -> Value {
        match source {
            Source::Register(index, register_type) => {
                let register = &self.registers[*index];
                match register_type {
                    RegisterType::U8 => register.get_value(ValueType::U8),
                    RegisterType::U16 => register.get_value(ValueType::U16),
                    RegisterType::U32 => register.get_value(ValueType::U32),
                    RegisterType::U64 => register.get_value(ValueType::U64),
                    RegisterType::I8 => register.get_value(ValueType::I8),
                    RegisterType::I16 => register.get_value(ValueType::I16),
                    RegisterType::I32 => register.get_value(ValueType::I32),
                    RegisterType::I64 => register.get_value(ValueType::I64),
                    RegisterType::F32 => register.get_value(ValueType::F32),
                    RegisterType::F64 => register.get_value(ValueType::F64),
                }
            }
            Source::Immediate(immediate) => immediate.into(),
        }
    }
}

impl CoreUtils<&Target> for Core {
    fn get_value(&self, source: &Target) -> Value {
        match source {
            Target(index, register_type) => {
                let register = &self.registers[*index];
                match register_type {
                    RegisterType::U8 => register.get_value(ValueType::U8),
                    RegisterType::U16 => register.get_value(ValueType::U16),
                    RegisterType::U32 => register.get_value(ValueType::U32),
                    RegisterType::U64 => register.get_value(ValueType::U64),
                    RegisterType::I8 => register.get_value(ValueType::I8),
                    RegisterType::I16 => register.get_value(ValueType::I16),
                    RegisterType::I32 => register.get_value(ValueType::I32),
                    RegisterType::I64 => register.get_value(ValueType::I64),
                    RegisterType::F32 => register.get_value(ValueType::F32),
                    RegisterType::F64 => register.get_value(ValueType::F64),
                }
            }
        }
    }
}

pub struct Core {
    flags: CoreFlags,
    pub registers: [Register; REGISTER_COUNT],
}

impl Core {

    fn set_value(&mut self, target: &Target, value: Value) {
        match target {
            Target(index, _) => {
                let register = &mut self.registers[*index];
                register.set_value(value)
            }
        }
    }

    pub fn execute_instruction(&mut self,
                               stack_frame: &mut dyn StackFrame,
                               module: &Module,
                               frames: &mut Vec<*const dyn StackFrame>,
                               memory: Memory,
                               continuation_store: &mut ContinuationStore,
    ) -> Result<InstructionResult,Fault> {

        let instruction = stack_frame.get_instruction();

        use Instruction::*;
        match instruction {
            Halt => return Ok(InstructionResult::Stop),
            NoOp => (),
            Load(ref target, ref source) => self.load_instruction(target, source),
            Add(ref target, ref source, can_wrap, use_carry) => self.add_instruction(target, source, can_wrap, use_carry)?,
            Sub(ref target, ref source, can_wrap, use_carry) => self.sub_instruction(target, source, can_wrap, use_carry)?,
            Mul(ref target, ref source, can_wrap) => self.mul_instruction(target, source, can_wrap)?,
            Div(ref target, ref source, can_wrap) => self.div_instruction(target, source, can_wrap)?,
            Mod(ref target, ref source, can_wrap) => self.mod_instruction(target, source, can_wrap)?,
            And(ref target, ref source) => self.and_instruction(target, source),
            Or(ref target, ref source) => self.or_instruction(target, source),
            Xor(ref target, ref source) => self.xor_instruction(target, source),
            Not(ref target) => self.not_instruction(target),
            ShiftLeft(ref target, ref source) => self.shift_left_instruction(target, source),
            ShiftRight(ref target, ref source) => self.shift_right_instruction(target, source),
            Goto(ref jump_target, ref condition) => return self.goto_instruction(stack_frame, jump_target, condition),
            Compare(ref target, ref source, ref comparison_type) => self.compare_instruction(target, source, comparison_type),
            Return(ref condition) => return self.return_instruction(stack_frame, condition),
            Call(call_target, ref condition) => return self.call_instruction(stack_frame, module, memory, continuation_store, call_target, condition),
            StackDeref(ref target, ref stack_level, ref offset) => self.stack_dereference_instruction(target, stack_level, offset, stack_frame, frames)?,
            Push(ref source) => self.push_instruction(stack_frame, source),
            Pop(ref target) => self.pop_instruction(stack_frame, target),

            x => unreachable!("Unimplemented instruction: {:?}", x),
        }

        stack_frame.increment_program_counter();

        Ok(InstructionResult::Continue)
    }


    fn load_instruction(&mut self, target: &Target, source: &Source) {
        let value = self.get_value(source);

        self.set_value(target, value);
    }

    fn add_instruction(&mut self, target: &Target, source: &Source, can_wrap: bool, use_carry: bool) -> Result<(), Fault> {

        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let (mut value, mut overflow) = lhs.overflowing_add(rhs);

        let value = if use_carry && self.flags.carry {
            let (value, new_overflow) = value.increment_overflowing();
            overflow = overflow || new_overflow;
            value
        } else {
            value
        };

        if overflow {
            if can_wrap {
                self.flags.carry = true;
            } else {
                return Err(Fault::Overflow);
            }
        } else {
            self.flags.carry = false;
        }

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
        Ok(())
    }

    fn sub_instruction(&mut self, target: &Target, source: &Source, can_wrap: bool, use_carry: bool) -> Result<(), Fault> {

        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let (mut value, mut overflow) = lhs.overflowing_sub(rhs);

        let value = if use_carry && self.flags.carry {
            let (value, new_overflow) = value.decrement_overflowing();
            overflow = overflow || new_overflow;
            value
        } else {
            value
        };

        if overflow {
            if can_wrap {
                self.flags.carry = true;
            } else {
                return Err(Fault::Overflow);
            }
        } else {
            self.flags.carry = false;
        }

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
        Ok(())
    }

    fn mul_instruction(&mut self, target: &Target, source: &Source, can_wrap: bool) -> Result<(), Fault> {

        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let (value, overflow) = lhs.overflowing_mul(rhs);


        if overflow {
            if can_wrap {
                self.flags.carry = true;
            } else {
                return Err(Fault::Overflow);
            }
        } else {
            self.flags.carry = false;
        }

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
        Ok(())
    }

    fn div_instruction(&mut self, target: &Target, source: &Source, can_wrap: bool) -> Result<(), Fault> {

        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let (value, overflow) = lhs.safe_div(rhs).ok_or(Fault::DivisionByZero)?;

        if overflow {
            if can_wrap {
                self.flags.carry = true;
            } else {
                return Err(Fault::Overflow);
            }
        } else {
            self.flags.carry = false;
        }

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
        Ok(())
    }

    fn mod_instruction(&mut self, target: &Target, source: &Source, can_wrap: bool) -> Result<(), Fault> {

        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let (value, overflow) = lhs.safe_mod(rhs).ok_or(Fault::DivisionByZero)?;

        if overflow {
            if can_wrap {
                self.flags.carry = true;
            } else {
                return Err(Fault::Overflow);
            }
        } else {
            self.flags.carry = false;
        }

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
        Ok(())
    }

    fn and_instruction(&mut self, target: &Target, source: &Source) {
        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let value = lhs & rhs;

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
    }

    fn or_instruction(&mut self, target: &Target, source: &Source) {
        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let value = lhs | rhs;

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
    }

    fn xor_instruction(&mut self, target: &Target, source: &Source) {
        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let value = lhs ^ rhs;

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
    }

    fn not_instruction(&mut self, target: &Target) {
        let value = self.get_value(target);

        let value = !value;

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
    }

    fn shift_left_instruction(&mut self, target: &Target, source: &Source) {
        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let value = lhs << rhs;

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
    }

    fn shift_right_instruction(&mut self, target: &Target, source: &Source) {
        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let value = lhs >> rhs;

        if value.is_negative() {
            self.flags.negative = true;
        } else {
            self.flags.negative = false;
        }

        if value.is_zero() {
            self.flags.zero = true;
        } else {
            self.flags.zero = false;
        }

        self.set_value(target, value);
    }

    fn can_jump(&self, condition: &Condition, stack_frame: &dyn StackFrame) -> bool {
        match condition {
            Condition::Always => true,
            Condition::Equal => self.flags.comparison == Comparison::Equal,
            Condition::NotEqual => self.flags.comparison == Comparison::NotEqual,
            Condition::LessThan => self.flags.comparison == Comparison::LessThan,
            Condition::LessThanOrEqual => self.flags.comparison == Comparison::LessThanOrEqual,
            Condition::GreaterThan => self.flags.comparison == Comparison::GreaterThan,
            Condition::GreaterThanOrEqual => self.flags.comparison == Comparison::GreaterThanOrEqual,
            Condition::Zero => self.flags.zero,
            Condition::NotZero => !self.flags.zero,
            Condition::Carry => self.flags.carry,
            Condition::NotCarry => !self.flags.carry,
            Condition::Negative => self.flags.negative,
            Condition::NotNegative => !self.flags.negative,
            Condition::InContinuation => stack_frame.is_continuation(),
            Condition::NotInContinuation => !stack_frame.is_continuation(),
        }
    }

    fn goto_instruction(&mut self, stack_frame: &mut dyn StackFrame, jump_target: &JumpTarget, condition: &Condition) -> Result<InstructionResult,Fault> {
        if self.can_jump(condition, stack_frame) {
            match jump_target {
                JumpTarget::Label(label) => {
                    todo!("Label Goto");
                },
                JumpTarget::Relative(offset) => {
                    stack_frame.set_program_counter((stack_frame.get_program_counter() as isize + *offset) as usize);
                },
                JumpTarget::Absolute(address) => {
                    stack_frame.set_program_counter(*address);
                },
            }
        }
        Ok(InstructionResult::Continue)
    }

    fn compare_instruction(&mut self, target: &Target, source: &Source, comparison_type: &ComparisonType) {
        let rhs = self.get_value(source);
        let lhs = self.get_value(target);

        let comparison = match comparison_type {
            ComparisonType::Equal => {
                if lhs == rhs {
                    Comparison::Equal
                } else {
                    Comparison::NotEqual
                }
            },
            ComparisonType::NotEqual => {
                if lhs != rhs {
                    Comparison::NotEqual
                } else {
                    Comparison::Equal
                }
            },
            ComparisonType::LessThan => {
                if lhs < rhs {
                    Comparison::LessThan
                } else {
                    Comparison::GreaterThanOrEqual
                }
            },
            ComparisonType::LessThanOrEqual => {
                if lhs <= rhs {
                    Comparison::LessThanOrEqual
                } else {
                    Comparison::GreaterThan
                }
            },
            ComparisonType::GreaterThan => {
                if lhs > rhs {
                    Comparison::GreaterThan
                } else {
                    Comparison::LessThanOrEqual
                }
            },
            ComparisonType::GreaterThanOrEqual => {
                if lhs >= rhs {
                    Comparison::GreaterThanOrEqual
                } else {
                    Comparison::LessThan
                }
            },
        };
        self.flags.comparison = comparison;
    }

    fn return_instruction(&mut self, stack_frame: &mut dyn StackFrame, condition: &Condition) -> Result<InstructionResult, Fault> {
        if self.can_jump(condition, stack_frame) {
            return Ok(InstructionResult::Return);
        }
        stack_frame.increment_program_counter();
        Ok(InstructionResult::Continue)
    }

    fn call_instruction(&mut self,
                        stack_frame: &mut dyn StackFrame,
                        module: &Module,
                        memory: Memory,
                        continuation_store: &mut ContinuationStore,
                        call_target: CallTarget,
                        condition: &Condition) -> Result<InstructionResult, Fault> {

        stack_frame.increment_program_counter();
        if self.can_jump(condition, stack_frame) {
            match call_target {
                CallTarget::Label(path) => {
                    let function = module.get_function(&path).ok_or(Fault::FunctionNotFound(path.clone()))?;
                    let instructions = function.get_instructions();
                    let new_stack_frame = Frame::new(path,instructions);

                    return Ok(InstructionResult::Call(function, new_stack_frame));
                }
                CallTarget::Vtable(object_reference_source, index_source) => {
                    todo!("Lookup object reference and create the stack frame from the vtable")
                }
                CallTarget::Continuation(continuation_index) => {}
                CallTarget::Closure(_) => {}
            }
        }
        Ok(InstructionResult::Continue)
    }

    fn stack_dereference_instruction(&mut self,
                                     target: &Target,
                                     stack_level: &Source,
                                     offset: &Source,
                                     stack_frame: &mut dyn StackFrame,
                                     frames: &mut Vec<*const dyn StackFrame>) -> Result<(), Fault> {

        let stack_level = self.get_value(stack_level);
        let offset = self.get_value(offset);


        let frame = if let Some(frame) = frames.iter().rev().nth(stack_level.to_usize()) {
            *frame
        } else if stack_level.to_usize() == frames.len() {
            stack_frame as *const dyn StackFrame
        } else {
            return Err(Fault::StackFrameOutOfBounds);
        };

        let value = unsafe {frame.as_ref()}.expect("Frame is null").get_value(offset, target.get_type().into());

        self.set_value(target, value);

        Ok(())
    }

    fn push_instruction(&mut self, stack_frame: &mut dyn StackFrame, source: &Source) {
        let value = self.get_value(source);
        stack_frame.push(value);
    }

    fn pop_instruction(&mut self, stack_frame: &mut dyn StackFrame, target: &Target) {
        let value = stack_frame.pop(target.get_type().into());
        self.set_value(target, value);
    }
}



