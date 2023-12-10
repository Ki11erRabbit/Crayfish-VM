use std::fmt::Display;
use crate::instruction::{RegisterType, Source, Target};
use crate::machine::Register;
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

pub struct Core {
    flags: CoreFlags,
    pub registers: [Register; REGISTER_COUNT],
}

impl Core {

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

    fn set_value(&mut self, target: &Target, value: Value) {
        match target {
            Target(index, _) => {
                let register = &mut self.registers[*index];
                register.set_value(value)
            }
        }
    }

    pub fn execute_instruction(&mut self, stack_frame: &mut dyn StackFrame) -> Result<,Fault>
    )
}



