use std::cmp::Ordering;
use crate::program::function::Function;
use crate::value::object::Object;

pub mod object;




pub enum ValueType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Object,
    String,
    Array,
    Function,
}

#[derive(Clone)]
pub enum Value<'a> {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Object(&'a Object<'a>),
    String(Box<str>),
    Array(Box<[Value<'a>]>),
    Function(Function),
}

impl Value<'_> {
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::U8(_) => ValueType::U8,
            Value::I8(_) => ValueType::I8,
            Value::U16(_) => ValueType::U16,
            Value::I16(_) => ValueType::I16,
            Value::U32(_) => ValueType::U32,
            Value::I32(_) => ValueType::I32,
            Value::U64(_) => ValueType::U64,
            Value::I64(_) => ValueType::I64,
            Value::F32(_) => ValueType::F32,
            Value::F64(_) => ValueType::F64,
            Value::Object(_) => ValueType::Object,
            Value::String(_) => ValueType::String,
            Value::Array(_) => ValueType::Array,
            Value::Function(_) => ValueType::Function,
        }
    }

    pub fn increment_overflowing<'a>(self) -> (Value<'a>, bool) {
        match self {
            Value::U8(val) => {
                let (value, overflow) = val.overflowing_add(1);
                (Value::U8(value), overflow)
            },
            Value::I8(val) => {
                let (value, overflow ) = val.overflowing_add(1);
                (Value::I8(value), overflow)
            },
            Value::U16(val) => {
                let (value, overflow ) = val.overflowing_add(1);
                (Value::U16(value), overflow)
            },
            Value::I16(val) => {
                let (value, overflow ) = val.overflowing_add(1);
                (Value::I16(value), overflow)
            },
            Value::U32(val) => {
                let (value, overflow ) = val.overflowing_add(1);
                (Value::U32(value), overflow)
            },
            Value::I32(val) => {
                let (value, overflow ) = val.overflowing_add(1);
                (Value::I32(value), overflow)
            },
            Value::U64(val) => {
                let (value, overflow ) = val.overflowing_add(1);
                (Value::U64(value), overflow)
            },
            Value::I64(val) => {
                let (value, overflow ) = val.overflowing_add(1);
                (Value::I64(value), overflow)
            },
            Value::F32(value) => (Value::F32(value + 1.0), false),
            Value::F64(value) => (Value::F64(value + 1.0), false),
            _ => panic!("Unable to add non-numeric values"),
        }
    }

    pub fn decrement_overflowing<'a>(self) -> (Value<'a>, bool) {
        match self {
            Value::U8(val) => {
                let (value, overflow) = val.overflowing_sub(1);
                (Value::U8(value), overflow)
            },
            Value::I8(val) => {
                let (value, overflow ) = val.overflowing_sub(1);
                (Value::I8(value), overflow)
            },
            Value::U16(val) => {
                let (value, overflow ) = val.overflowing_sub(1);
                (Value::U16(value), overflow)
            },
            Value::I16(val) => {
                let (value, overflow ) = val.overflowing_sub(1);
                (Value::I16(value), overflow)
            },
            Value::U32(val) => {
                let (value, overflow ) = val.overflowing_sub(1);
                (Value::U32(value), overflow)
            },
            Value::I32(val) => {
                let (value, overflow ) = val.overflowing_sub(1);
                (Value::I32(value), overflow)
            },
            Value::U64(val) => {
                let (value, overflow ) = val.overflowing_sub(1);
                (Value::U64(value), overflow)
            },
            Value::I64(val) => {
                let (value, overflow ) = val.overflowing_sub(1);
                (Value::I64(value), overflow)
            },
            Value::F32(value) => (Value::F32(value + 1.0), false),
            Value::F64(value) => (Value::F64(value + 1.0), false),
            _ => panic!("Unable to add non-numeric values"),
        }
    }

    pub fn overflowing_add(self, rhs: Value) -> (Value, bool) {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => {
                let (value, overflow) = lhs.overflowing_add(rhs);
                (Value::U8(value), overflow)
            }
            (Value::I8(lhs), Value::I8(rhs)) => {
                let (value, overflow) = lhs.overflowing_add(rhs);
                (Value::I8(value), overflow)
            }
            (Value::U16(lhs), Value::U16(rhs)) => {
                let (value, overflow) = lhs.overflowing_add(rhs);
                (Value::U16(value), overflow)
            }
            (Value::I16(lhs), Value::I16(rhs)) => {
                let (value, overflow) = lhs.overflowing_add(rhs);
                (Value::I16(value), overflow)
            }
            (Value::U32(lhs), Value::U32(rhs)) => {
                let (value, overflow) = lhs.overflowing_add(rhs);
                (Value::U32(value), overflow)
            }
            (Value::I32(lhs), Value::I32(rhs)) => {
                let (value, overflow) = lhs.overflowing_add(rhs);
                (Value::I32(value), overflow)
            }
            (Value::U64(lhs), Value::U64(rhs)) => {
                let (value, overflow) = lhs.overflowing_add(rhs);
                (Value::U64(value), overflow)
            }
            (Value::I64(lhs), Value::I64(rhs)) => {
                let (value, overflow) = lhs.overflowing_add(rhs);
                (Value::I64(value), overflow)
            }
            (Value::F32(lhs), Value::F32(rhs)) => {
                let value = lhs + rhs;
                (Value::F32(value), false)
            }
            (Value::F64(lhs), Value::F64(rhs)) => {
                let value = lhs + rhs;
                (Value::F64(value), false)
            }
            _ => panic!("Cannot add values of different types"),
        }
    }

    pub fn overflowing_sub(self, rhs: Value) -> (Value, bool) {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => {
                let (value, overflow) = lhs.overflowing_sub(rhs);
                (Value::U8(value), overflow)
            }
            (Value::I8(lhs), Value::I8(rhs)) => {
                let (value, overflow) = lhs.overflowing_sub(rhs);
                (Value::I8(value), overflow)
            }
            (Value::U16(lhs), Value::U16(rhs)) => {
                let (value, overflow) = lhs.overflowing_sub(rhs);
                (Value::U16(value), overflow)
            }
            (Value::I16(lhs), Value::I16(rhs)) => {
                let (value, overflow) = lhs.overflowing_sub(rhs);
                (Value::I16(value), overflow)
            }
            (Value::U32(lhs), Value::U32(rhs)) => {
                let (value, overflow) = lhs.overflowing_sub(rhs);
                (Value::U32(value), overflow)
            }
            (Value::I32(lhs), Value::I32(rhs)) => {
                let (value, overflow) = lhs.overflowing_sub(rhs);
                (Value::I32(value), overflow)
            }
            (Value::U64(lhs), Value::U64(rhs)) => {
                let (value, overflow) = lhs.overflowing_sub(rhs);
                (Value::U64(value), overflow)
            }
            (Value::I64(lhs), Value::I64(rhs)) => {
                let (value, overflow) = lhs.overflowing_sub(rhs);
                (Value::I64(value), overflow)
            }
            (Value::F32(lhs), Value::F32(rhs)) => {
                let value = lhs - rhs;
                (Value::F32(value), false)
            }
            (Value::F64(lhs), Value::F64(rhs)) => {
                let value = lhs - rhs;
                (Value::F64(value), false)
            }
            _ => panic!("Cannot subtract values of different types"),
        }
    }


    pub fn overflowing_mul(self, rhs: Value) -> (Value, bool) {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => {
                let (value, overflow) = lhs.overflowing_mul(rhs);
                (Value::U8(value), overflow)
            }
            (Value::I8(lhs), Value::I8(rhs)) => {
                let (value, overflow) = lhs.overflowing_mul(rhs);
                (Value::I8(value), overflow)
            }
            (Value::U16(lhs), Value::U16(rhs)) => {
                let (value, overflow) = lhs.overflowing_mul(rhs);
                (Value::U16(value), overflow)
            }
            (Value::I16(lhs), Value::I16(rhs)) => {
                let (value, overflow) = lhs.overflowing_mul(rhs);
                (Value::I16(value), overflow)
            }
            (Value::U32(lhs), Value::U32(rhs)) => {
                let (value, overflow) = lhs.overflowing_mul(rhs);
                (Value::U32(value), overflow)
            }
            (Value::I32(lhs), Value::I32(rhs)) => {
                let (value, overflow) = lhs.overflowing_mul(rhs);
                (Value::I32(value), overflow)
            }
            (Value::U64(lhs), Value::U64(rhs)) => {
                let (value, overflow) = lhs.overflowing_mul(rhs);
                (Value::U64(value), overflow)
            }
            (Value::I64(lhs), Value::I64(rhs)) => {
                let (value, overflow) = lhs.overflowing_mul(rhs);
                (Value::I64(value), overflow)
            }
            (Value::F32(lhs), Value::F32(rhs)) => {
                let value = lhs * rhs;
                (Value::F32(value), false)
            }
            (Value::F64(lhs), Value::F64(rhs)) => {
                let value = lhs * rhs;
                (Value::F64(value), false)
            }
            _ => panic!("Cannot multiply values of different types"),
        }
    }

    pub fn safe_div(self, rhs: Value) -> Option<(Value, bool)> {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_div(rhs);
                    Some((Value::U8(value), overflow))
                }
            }
            (Value::I8(lhs), Value::I8(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_div(rhs);
                    Some((Value::I8(value), overflow))
                }
            }
            (Value::U16(lhs), Value::U16(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_div(rhs);
                    Some((Value::U16(value), overflow))
                }
            }
            (Value::I16(lhs), Value::I16(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_div(rhs);
                    Some((Value::I16(value), overflow))
                }
            }
            (Value::U32(lhs), Value::U32(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_div(rhs);
                    Some((Value::U32(value), overflow))
                }
            }
            (Value::I32(lhs), Value::I32(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_div(rhs);
                    Some((Value::I32(value), overflow))
                }
            }
            (Value::U64(lhs), Value::U64(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_div(rhs);
                    Some((Value::U64(value), overflow))
                }
            }
            (Value::I64(lhs), Value::I64(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_div(rhs);
                    Some((Value::I64(value), overflow))
                }
            }
            (Value::F32(lhs), Value::F32(rhs)) => {
                if rhs == 0.0 {
                    None
                } else {
                    let value = lhs / rhs;
                    Some((Value::F32(value), false))
                }
            }
            (Value::F64(lhs), Value::F64(rhs)) => {
                if rhs == 0.0 {
                    None
                } else {
                    let value = lhs / rhs;
                    Some((Value::F64(value), false))
                }
            }
            _ => panic!("Cannot divide values of different types"),
        }
    }

    pub fn safe_mod(self, rhs: Value) -> Option<(Value, bool)> {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_rem(rhs);
                    Some((Value::U8(value), overflow))
                }
            }
            (Value::I8(lhs), Value::I8(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_rem(rhs);
                    Some((Value::I8(value), overflow))
                }
            }
            (Value::U16(lhs), Value::U16(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_rem(rhs);
                    Some((Value::U16(value), overflow))
                }
            }
            (Value::I16(lhs), Value::I16(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_rem(rhs);
                    Some((Value::I16(value), overflow))
                }
            }
            (Value::U32(lhs), Value::U32(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_rem(rhs);
                    Some((Value::U32(value), overflow))
                }
            }
            (Value::I32(lhs), Value::I32(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_rem(rhs);
                    Some((Value::I32(value), overflow))
                }
            }
            (Value::U64(lhs), Value::U64(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_rem(rhs);
                    Some((Value::U64(value), overflow))
                }
            }
            (Value::I64(lhs), Value::I64(rhs)) => {
                if rhs == 0 {
                    None
                } else {
                    let (value, overflow) = lhs.overflowing_rem(rhs);
                    Some((Value::I64(value), overflow))
                }
            }
            (Value::F32(lhs), Value::F32(rhs)) => {
                if rhs == 0.0 {
                    None
                } else {
                    let value = lhs % rhs;
                    Some((Value::F32(value), false))
                }
            }
            (Value::F64(lhs), Value::F64(rhs)) => {
                if rhs == 0.0 {
                    None
                } else {
                    let value = lhs % rhs;
                    Some((Value::F64(value), false))
                }
            }
            _ => panic!("Cannot divide values of different types"),
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            Value::U8(_) => false,
            Value::I8(value) => *value < 0,
            Value::U16(_) => false,
            Value::I16(value) => *value < 0,
            Value::U32(_) => false,
            Value::I32(value) => *value < 0,
            Value::U64(_) => false,
            Value::I64(value) => *value < 0,
            Value::F32(value) => *value < 0.0,
            Value::F64(value) => *value < 0.0,
            _ => panic!("Cannot check if values of different types are negative"),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Value::U8(value) => *value == 0,
            Value::I8(value) => *value == 0,
            Value::U16(value) => *value == 0,
            Value::I16(value) => *value == 0,
            Value::U32(value) => *value == 0,
            Value::I32(value) => *value == 0,
            Value::U64(value) => *value == 0,
            Value::I64(value) => *value == 0,
            Value::F32(value) => *value == 0.0,
            Value::F64(value) => *value == 0.0,
            _ => panic!("Cannot check if values of different types are zero"),
        }
    }

}



impl<'a> std::ops::BitAnd for Value<'a> {
    type Output = Value<'a>;
    fn bitand(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => Value::U8(lhs & rhs),
            (Value::I8(lhs), Value::I8(rhs)) => Value::I8(lhs & rhs),
            (Value::U16(lhs), Value::U16(rhs)) => Value::U16(lhs & rhs),
            (Value::I16(lhs), Value::I16(rhs)) => Value::I16(lhs & rhs),
            (Value::U32(lhs), Value::U32(rhs)) => Value::U32(lhs & rhs),
            (Value::I32(lhs), Value::I32(rhs)) => Value::I32(lhs & rhs),
            (Value::U64(lhs), Value::U64(rhs)) => Value::U64(lhs & rhs),
            (Value::I64(lhs), Value::I64(rhs)) => Value::I64(lhs & rhs),
            _ => panic!("Cannot and values of different types"),
        }
    }
}

impl std::ops::BitOr for Value<'_> {
    type Output = Value<'static>;
    fn bitor(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => Value::U8(lhs | rhs),
            (Value::I8(lhs), Value::I8(rhs)) => Value::I8(lhs | rhs),
            (Value::U16(lhs), Value::U16(rhs)) => Value::U16(lhs | rhs),
            (Value::I16(lhs), Value::I16(rhs)) => Value::I16(lhs | rhs),
            (Value::U32(lhs), Value::U32(rhs)) => Value::U32(lhs | rhs),
            (Value::I32(lhs), Value::I32(rhs)) => Value::I32(lhs | rhs),
            (Value::U64(lhs), Value::U64(rhs)) => Value::U64(lhs | rhs),
            (Value::I64(lhs), Value::I64(rhs)) => Value::I64(lhs | rhs),
            _ => panic!("Cannot or values of different types"),
        }
    }
}

impl std::ops::BitXor for Value<'_> {
    type Output = Value<'static>;
    fn bitxor(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => Value::U8(lhs ^ rhs),
            (Value::I8(lhs), Value::I8(rhs)) => Value::I8(lhs ^ rhs),
            (Value::U16(lhs), Value::U16(rhs)) => Value::U16(lhs ^ rhs),
            (Value::I16(lhs), Value::I16(rhs)) => Value::I16(lhs ^ rhs),
            (Value::U32(lhs), Value::U32(rhs)) => Value::U32(lhs ^ rhs),
            (Value::I32(lhs), Value::I32(rhs)) => Value::I32(lhs ^ rhs),
            (Value::U64(lhs), Value::U64(rhs)) => Value::U64(lhs ^ rhs),
            (Value::I64(lhs), Value::I64(rhs)) => Value::I64(lhs ^ rhs),
            _ => panic!("Cannot xor values of different types"),
        }
    }
}

impl<'a> std::ops::Shl<Value<'a>> for Value<'_> {
    type Output = Value<'a>;
    fn shl(self, rhs: Value<'a>) -> Self::Output {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => Value::U8(lhs << rhs),
            (Value::I8(lhs), Value::I8(rhs)) => Value::I8(lhs << rhs),
            (Value::U16(lhs), Value::U16(rhs)) => Value::U16(lhs << rhs),
            (Value::I16(lhs), Value::I16(rhs)) => Value::I16(lhs << rhs),
            (Value::U32(lhs), Value::U32(rhs)) => Value::U32(lhs << rhs),
            (Value::I32(lhs), Value::I32(rhs)) => Value::I32(lhs << rhs),
            (Value::U64(lhs), Value::U64(rhs)) => Value::U64(lhs << rhs),
            (Value::I64(lhs), Value::I64(rhs)) => Value::I64(lhs << rhs),
            _ => panic!("Cannot shift values of different types"),
        }
    }
}

impl<'a> std::ops::Shr<Value<'a>> for Value<'a> {
    type Output = Value<'a>;
    fn shr(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::U8(lhs), Value::U8(rhs)) => Value::U8(lhs >> rhs),
            (Value::I8(lhs), Value::I8(rhs)) => Value::I8(lhs >> rhs),
            (Value::U16(lhs), Value::U16(rhs)) => Value::U16(lhs >> rhs),
            (Value::I16(lhs), Value::I16(rhs)) => Value::I16(lhs >> rhs),
            (Value::U32(lhs), Value::U32(rhs)) => Value::U32(lhs >> rhs),
            (Value::I32(lhs), Value::I32(rhs)) => Value::I32(lhs >> rhs),
            (Value::U64(lhs), Value::U64(rhs)) => Value::U64(lhs >> rhs),
            (Value::I64(lhs), Value::I64(rhs)) => Value::I64(lhs >> rhs),
            _ => panic!("Cannot shift values of different types"),
        }
    }
}

impl std::ops::Not for Value<'_> {
    type Output = Value<'static>;

    fn not(self) -> Self::Output {
        match self {
            Value::U8(val) => Value::U8(!val),
            Value::I8(val) => Value::I8(!val),
            Value::U16(val) => Value::U16(!val),
            Value::I16(val) => Value::I16(!val),
            Value::U32(val) => Value::U32(!val),
            Value::I32(val) => Value::I32(!val),
            Value::U64(val) => Value::U64(!val),
            Value::I64(val) => Value::I64(!val),
            _ => panic!("Cannot Bitwise Not Floats or other types")
        }
    }
}

impl<'a> PartialEq<Value<'a>> for Value<'a> {
    fn eq(&self, other: &Value<'a>) -> bool {
        match (self, other) {
            (Value::U8(lhs), Value::U8(rhs)) => lhs == rhs,
            (Value::I8(lhs), Value::I8(rhs)) => lhs == rhs,
            (Value::U16(lhs), Value::U16(rhs)) => lhs == rhs,
            (Value::I16(lhs), Value::I16(rhs)) => lhs == rhs,
            (Value::U32(lhs), Value::U32(rhs)) => lhs == rhs,
            (Value::I32(lhs), Value::I32(rhs)) => lhs == rhs,
            (Value::U64(lhs), Value::U64(rhs)) => lhs == rhs,
            (Value::I64(lhs), Value::I64(rhs)) => lhs == rhs,
            (Value::F32(lhs), Value::F32(rhs)) => lhs == rhs,
            (Value::F64(lhs), Value::F64(rhs)) => lhs == rhs,
            _ => panic!("Cannot compare values of different types"),
        }
    }
}

impl<'a> PartialOrd<Value<'a>> for Value<'a> {
    fn partial_cmp(&self, other: &Value<'a>) -> Option<Ordering> {
        match (self, other) {
            (Value::U8(lhs), Value::U8(rhs)) => lhs.partial_cmp(rhs),
            (Value::I8(lhs), Value::I8(rhs)) => lhs.partial_cmp(rhs),
            (Value::U16(lhs), Value::U16(rhs)) => lhs.partial_cmp(rhs),
            (Value::I16(lhs), Value::I16(rhs)) => lhs.partial_cmp(rhs),
            (Value::U32(lhs), Value::U32(rhs)) => lhs.partial_cmp(rhs),
            (Value::I32(lhs), Value::I32(rhs)) => lhs.partial_cmp(rhs),
            (Value::U64(lhs), Value::U64(rhs)) => lhs.partial_cmp(rhs),
            (Value::I64(lhs), Value::I64(rhs)) => lhs.partial_cmp(rhs),
            (Value::F32(lhs), Value::F32(rhs)) => lhs.partial_cmp(rhs),
            (Value::F64(lhs), Value::F64(rhs)) => lhs.partial_cmp(rhs),
            _ => panic!("Cannot compare values of different types"),
        }
    }
}

impl<'a> Into<Box<[u8]>> for Value<'a> {
    fn into(self) -> Box<[u8]> {
        match self {
            Value::U8(val) => Box::new([val]),
            Value::I8(val) => Box::new([val as u8]),
            Value::U16(val) => Box::new(val.to_le_bytes()),
            Value::I16(val) => Box::new(val.to_le_bytes()),
            Value::U32(val) => Box::new(val.to_le_bytes()),
            Value::I32(val) => Box::new(val.to_le_bytes()),
            Value::U64(val) => Box::new(val.to_le_bytes()),
            Value::I64(val) => Box::new(val.to_le_bytes()),
            Value::F32(val) => Box::new(val.to_le_bytes()),
            Value::F64(val) => Box::new(val.to_le_bytes()),
            _ => panic!("Cannot convert non-numeric values to bytes"),
        }
    }
}

impl<'a> Into<Value<'a>> for u8 {
    fn into(self) -> Value<'a> {
        Value::U8(self)
    }
}

impl<'a> Into<Value<'a>> for i8 {
    fn into(self) -> Value<'a> {
        Value::I8(self)
    }
}

impl<'a> Into<Value<'a>> for u16 {
    fn into(self) -> Value<'a> {
        Value::U16(self)
    }
}

impl<'a> Into<Value<'a>> for i16 {
    fn into(self) -> Value<'a> {
        Value::I16(self)
    }
}

impl<'a> Into<Value<'a>> for u32 {
    fn into(self) -> Value<'a> {
        Value::U32(self)
    }
}

impl<'a> Into<Value<'a>> for i32 {
    fn into(self) -> Value<'a> {
        Value::I32(self)
    }
}

impl<'a> Into<Value<'a>> for u64 {
    fn into(self) -> Value<'a> {
        Value::U64(self)
    }
}

impl<'a> Into<Value<'a>> for i64 {
    fn into(self) -> Value<'a> {
        Value::I64(self)
    }
}

impl<'a> Into<Value<'a>> for f32 {
    fn into(self) -> Value<'a> {
        Value::F32(self)
    }
}

impl<'a> Into<Value<'a>> for f64 {
    fn into(self) -> Value<'a> {
        Value::F64(self)
    }
}
