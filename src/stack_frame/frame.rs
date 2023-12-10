use std::sync::Arc;
use smallvec::SmallVec;
use crate::instruction::Instruction;
use crate::stack_frame::{REGISTER_COUNT, ReturnAddress};



pub struct FrameInfo {
    function_name: Box<str>,
    instructions: Arc<[Instruction]>,
    program_counter: usize,
}


pub struct Frame {
    frame_info: FrameInfo,
    return_address: Option<ReturnAddress>,
    stack: SmallVec<[u8; 128]>,
    stack_pointer: usize,
    call_backup: Option<[Register; REGISTER_COUNT]>,
    gc_backup: Option<[Register; REGISTER_COUNT]>,
}