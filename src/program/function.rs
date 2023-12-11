use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use crate::instruction::Instruction;
use crate::machine::core::Core;
use crate::machine::{Fault, InstructionResult};
use crate::memory::Memory;
use crate::program::Module;
use crate::stack_frame::delimited_continuation::ContinuationStore;
use crate::stack_frame::frame::Frame;
use crate::stack_frame::StackFrame;


pub type NativeFunction = fn(&mut Core, &Module, &mut Vec<*const dyn StackFrame>, Memory, &mut ContinuationStore) -> Result<InstructionResult,Fault>;

#[derive(Clone)]
pub struct FunctionPath {
    pub(crate) path: Box<[Box<str>]>
}

impl Display for FunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();
        for part in self.path.iter() {
            path.push_str(part);
            path.push_str("::");
        }
        write!(f, "{}", path)
    }
}
impl Debug for FunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Into<FunctionPath> for &str {
    fn into(self) -> FunctionPath {
        FunctionPath {
            path: self.split("::").map(|s| s.into()).collect::<Vec<Box<str>>>().into_boxed_slice()
        }
    }
}


#[derive(Clone)]
pub enum Function {
    ByteCode(Arc<[Instruction]>),
    Native(NativeFunction),
}

impl Function {
    pub fn get_instructions(&self) -> Arc<[Instruction]> {
        match self {
            Function::ByteCode(instructions) => instructions.clone(),
            Function::Native(_) => Arc::new([]),
        }
    }
}


