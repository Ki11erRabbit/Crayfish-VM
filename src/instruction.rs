use crate::program::function::FunctionPath;
use crate::value::{Value, ValueType};

/// Represents in what state a register should act as.
#[derive(Debug, Clone, Copy)]
pub enum RegisterType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl Into<ValueType> for RegisterType {
    fn into(self) -> ValueType {
        match self {
            RegisterType::U8 => ValueType::U8,
            RegisterType::U16 => ValueType::U16,
            RegisterType::U32 => ValueType::U32,
            RegisterType::U64 => ValueType::U64,
            RegisterType::I8 => ValueType::I8,
            RegisterType::I16 => ValueType::I16,
            RegisterType::I32 => ValueType::I32,
            RegisterType::I64 => ValueType::I64,
            RegisterType::F32 => ValueType::F32,
            RegisterType::F64 => ValueType::F64,
        }
    }
}

/// Represents a target for an instruction.
/// A register target.
/// usize represents the index of the register.
/// RegisterType represents the state the register should act as.
#[derive(Debug, Clone)]
pub struct Target(pub usize, pub RegisterType);

impl Target {
    pub fn get_type(&self) -> RegisterType {
        self.1
    }
}


#[derive(Debug, Clone)]
pub enum Source {
    /// A register source.
    /// usize represents the index of the register.
    /// RegisterType represents the state the register should act as.
    Register(usize, RegisterType),
    /// An immediate value source.
    /// Immediate represents the value of the immediate.
    Immediate(Immediate),
}


#[derive(Debug, Clone)]
pub enum Immediate {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Into<Value> for Immediate {
    fn into(self) -> Value {
        match self {
            Immediate::U8(value) => Value::U8(value),
            Immediate::U16(value) => Value::U16(value),
            Immediate::U32(value) => Value::U32(value),
            Immediate::U64(value) => Value::U64(value),
            Immediate::I8(value) => Value::I8(value),
            Immediate::I16(value) => Value::I16(value),
            Immediate::I32(value) => Value::I32(value),
            Immediate::I64(value) => Value::I64(value),
            Immediate::F32(value) => Value::F32(value),
            Immediate::F64(value) => Value::F64(value),
        }
    }
}

impl Into<Value> for &Immediate {
    fn into(self) -> Value {
        match self {
            Immediate::U8(value) => Value::U8(*value),
            Immediate::U16(value) => Value::U16(*value),
            Immediate::U32(value) => Value::U32(*value),
            Immediate::U64(value) => Value::U64(*value),
            Immediate::I8(value) => Value::I8(*value),
            Immediate::I16(value) => Value::I16(*value),
            Immediate::I32(value) => Value::I32(*value),
            Immediate::I64(value) => Value::I64(*value),
            Immediate::F32(value) => Value::F32(*value),
            Immediate::F64(value) => Value::F64(*value),
        }
    }
}

/// Represents a jump target for a jump instruction.
/// This can be a relative jump, an absolute jump or a label.
#[derive(Debug, Clone)]
pub enum JumpTarget {
    /// A relative jump target.
    /// The i32 represents the offset from the current instruction.
    /// A positive value means forward, a negative value means backward.
    /// 0 means the next instruction.
    Relative(isize),
    /// An absolute jump target.
    /// The usize represents the address of the instruction.
    /// This is the address of the instruction, not the address of the memory.
    Absolute(usize),
    /// A label jump target.
    /// The String represents the name of the label.
    /// This is the name of the label, not the address of the memory.
    /// The address of the label is resolved at runtime.
    Label(Box<str>),
}

/// Represents a condition for a jump instruction.
/// This can be a condition or no condition.
/// If the condition is not met, the jump instruction is ignored.
/// If the condition is met, the jump instruction is executed.
#[derive(Debug, Clone)]
pub enum Condition {
    /// No condition.
    Always,
    /// When the last instruction resulted in an equal result.
    Equal,
    /// When the last instruction resulted in an unequal result.
    NotEqual,
    /// When the last instruction resulted in a greater than result.
    GreaterThan,
    /// When the last instruction resulted in a greater than or equal result.
    GreaterThanOrEqual,
    /// When the last instruction resulted in a less than result.
    LessThan,
    /// When the last instruction resulted in a less than or equal result.
    LessThanOrEqual,
    /// When the last instruction resulted in a zero result.
    Zero,
    /// When the last instruction resulted in a non-zero result.
    NotZero,
    /// When the last instruction resulted in a carry result.
    Carry,
    /// When the last instruction resulted in a non-carry result.
    NotCarry,
    /// When the last instruction resulted in a negative result.
    Negative,
    /// When the last instruction resulted in a non-negative result.
    NotNegative,
    /// True if the current stack frame is a continuation.
    InContinuation,
    /// True if the current stack frame is not a continuation.
    NotInContinuation,
}


#[derive(Debug, Clone)]
pub enum ComparisonType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}


#[derive(Debug, Clone)]
pub enum CallTarget {
    Label(FunctionPath),
    Vtable(Source, Source),
    Continuation(Source),
    Closure(Source),
}


#[derive(Debug, Clone)]
pub enum Instruction {
    /// A halt instruction.
    /// This instruction stops the program.
    /// This instruction has no arguments.
    Halt,
    /// A no-op instruction.
    /// This instruction does nothing.
    NoOp,
    /// A load instruction.
    /// This instruction loads a value from a source into a target.
    /// The target can be a register or a memory address.
    /// The source can be a register, a memory address or an immediate value.
    Load(Target, Source),
    /// A store instruction.
    /// This instruction stores a value from a source into a memory reference.
    /// The source can be a register or an immediate value.
    Store(Source, Source, Source),
    /// Stack dereference instruction.
    /// This instruction dereferences a value from a stack.
    /// The target is where the value is stored
    /// The source is the index of the value on the stack.
    /// The last source is the index of the stack frame.
    StackDeref(Target, Source, Source),
    /// Stack reference instruction.
    /// This instruction stores a into the stack.
    /// The first source is the stack level
    /// The second source is the offset from the stack pointer
    /// The third source value to store
    StackStore(Source, Source, Source),
    /// Add instruction.
    /// This instruction adds a value from a register into a register.
    /// Overflow is considered an error unless first bool is set to true.
    /// First bool is if wrapping is allowed.
    /// Second bool is if carry flag should be factored in
    Add(Target, Source, bool, bool),
    /// Subtract instruction.
    /// This instruction subtracts a value from a register into a register.
    /// Overflow is considered an error first bool is set to true.
    /// First bool is if wrapping is allowed.
    /// Second bool is if carry flag should be factored in
    Sub(Target, Source, bool, bool),
    /// Multiply instruction.
    /// This instruction multiplies a value from a register into a register.
    /// Overflow is considered an error unless bool is set to true.
    /// Bool is if wrapping is allowed.
    Mul(Target, Source, bool),
    /// Divide instruction.
    /// This instruction divides a value from a register into a register.
    /// Overflow is considered an error but should never happen.
    /// Division by zero is considered an error.
    Div(Target, Source, bool),
    /// Modulo instruction.
    /// This instruction calculates the modulo of a value from a register into a register.
    /// Overflow is considered an error but should never happen.
    /// Division by zero is considered an error.
    Mod(Target, Source, bool),
    /// And instruction.
    /// This instruction calculates the bitwise and of a value from a register into a register.
    And(Target, Source),
    /// Or instruction.
    /// This instruction calculates the bitwise or of a value from a register into a register.
    Or(Target, Source),
    /// Xor instruction.
    /// This instruction calculates the bitwise xor of a value from a register into a register.
    Xor(Target, Source),
    /// Not instruction.
    /// This instruction calculates the bitwise not of a value from a register into a register.
    Not(Target),
    /// Shift left instruction.
    /// This instruction shifts a value from a register into a register to the left.
    ShiftLeft(Target, Source),
    /// Shift right instruction.
    /// This instruction shifts a value from a register into a register to the right.
    /// This will be an arithmetic shift if the values are signed.
    /// This will be a logical shift if the values are unsigned.
    ShiftRight(Target, Source),
    /// Jump instruction.
    /// This instruction jumps to a jump target if the condition is met.
    /// The jump target can be a relative jump, an absolute jump or a label.
    /// The condition can be a condition or no condition.
    Goto(JumpTarget, Condition),
    /// Compare instruction.
    /// This instruction compares a value from a register with a value from a register.
    /// The result is stored in the flags.
    /// The flags are used by the jump instructions.
    Compare(Target, Source, ComparisonType),
    /// Push instruction.
    /// This instruction pushes a value from a register onto the stack.
    /// The stack pointer is decremented by the size of the value.
    Push(Source),
    /// Pop instruction.
    /// This instruction pops a value from the stack into a register.
    /// The stack pointer is incremented by the size of the value.
    /// The register is cleared before the value is pushed.
    Pop(Target),
    /// Call instruction.
    /// This instruction calls a function.
    Call(CallTarget, Condition),
    /// Return instruction.
    /// This instruction returns from a function.
    /// The return address is popped from the stack.
    Return(Condition),
    /// Create Delimited Continuation instruction.
    /// This instruction creates a delimited continuation from the current stack frame.
    /// The index/reference of the continuation is stored in the target.
    /// The target can be a register or a memory address.
    CreateContinuation(Target),
    CreateObject(Target),

}