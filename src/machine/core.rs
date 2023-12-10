use std::fmt::Display;
use crate::instruction::{Instruction, RegisterType, Source, Target};
use crate::machine::{Fault, InstructionResult, InstructionResultModifier, Register};
use crate::stack_frame::{REGISTER_COUNT, StackFrame};
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

    pub fn execute_instruction(&mut self, stack_frame: &mut dyn StackFrame) -> Result<InstructionResult,Fault> {
        use Instruction::*;
        match &stack_frame.get_instruction() {
            Halt => return Ok(InstructionResult::Stop),
            NoOp => (),
            Load(target, source) => self.load_instruction(target, source),
            Add(target, source, can_wrap, use_carry) => self.add_instruction(target, source, *can_wrap, *use_carry)?,
            Sub(target, source, can_wrap, use_carry) => self.sub_instruction(target, source, *can_wrap, *use_carry)?,
            Mul(target, source, can_wrap) => self.mul_instruction(target, source, *can_wrap)?,
            Div(target, source, can_wrap) => self.div_instruction(target, source, *can_wrap)?,
            Mod(target, source, can_wrap) => self.mod_instruction(target, source, *can_wrap)?,

        }

        stack_frame.increment_program_counter();

        Ok(InstructionResult::Continue(InstructionResultModifier::None))
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
}



