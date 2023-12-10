use std::fmt::Display;
use crate::stack_frame::frame::Frame;
use crate::value::{Value, ValueType};
use crate::stack_frame::delimited_continuation::DelimitedContinuation;
pub mod core;


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


pub struct Register {
    pub value: [u8; 8],
}

impl Register {
    fn get_value(&self, size: ValueType) -> Value {
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
                self.value[0] = u8::from_le_bytes(value.to_le_bytes())
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


pub enum InstructionResult {
    Continue(InstructionResultModifier),
    Stop,
}

pub enum InstructionResultModifier {
    None,
    /// Unwind until the specified effect is found.
    Unwind(Option<Box<str>>),
    CallFrame(Frame),
    CallContinuation(DelimitedContinuation),
}
