use std::collections::HashMap;
use std::sync::{Arc, RwLock, TryLockError};
use rand::Rng;
use crate::machine::Fault;
use crate::program::function::Function;
use crate::program::{Module, StringTablePath};
use crate::value::object::Object;
use crate::value::{Value, ValueType};


#[derive(Debug, Clone)]
pub enum MemoryObject {
    Null,
    StringTableRef(StringTablePath, u64),
    String(*const str),
    Object(*mut Object),
    List(*mut [Value]),
    Pointer(*mut u8, usize, ValueType),
}



#[derive(Debug, Clone)]
pub struct Memory {
    reference_table: Arc<RwLock<HashMap<u64, MemoryObject>>>,
    string_lookup_table: Arc<RwLock<HashMap<(StringTablePath, u64), u64>>>,
    rng: rand::rngs::ThreadRng,
}

impl Memory {
    pub fn new() -> Self {
        Memory { reference_table: Arc::new(Default::default()), string_lookup_table: Arc::new(Default::default()), rng: rand::thread_rng() }
    }

    pub fn get(&self, reference: u64) -> Result<MemoryObject, Fault> {
        loop {
            match self.reference_table.try_read() {
                Ok(reference_table) => {
                    return reference_table.get(&reference).cloned().ok_or(Fault::InvalidReference);
                }
                Err(TryLockError::WouldBlock)=> {}
                Err(TryLockError::Poisoned(_)) => Err(Fault::MemoryError("Poisoned".to_string()))?,
            }
        }
    }

    pub fn allocate(&mut self, object: MemoryObject) -> Result<u64, Fault> {
        loop {
            match self.reference_table.try_write() {
                Ok(mut reference_table) => {
                    let mut index = self.rng.gen::<u64>();
                    while reference_table.contains_key(&index) {
                        index = self.rng.gen::<u64>();
                    }
                    reference_table.insert(index, object);
                    return Ok(index);
                }
                Err(TryLockError::WouldBlock)=> {}
                Err(TryLockError::Poisoned(_)) => Err(Fault::MemoryError("Poisoned".to_string()))?,
            }
        }
    }

    fn get_layout(size: usize, value_type: ValueType) -> std::alloc::Layout {
        match value_type {
            ValueType::U8 => std::alloc::Layout::from_size_align(size, 1).expect("Bad Layout"),
            ValueType::U16 => std::alloc::Layout::from_size_align(size * 2, 2).expect("Bad Layout"),
            ValueType::U32 => std::alloc::Layout::from_size_align(size * 4, 4).expect("Bad Layout"),
            ValueType::U64 => std::alloc::Layout::from_size_align(size * 8, 8).expect("Bad Layout"),
            ValueType::I8 => std::alloc::Layout::from_size_align(size, 1).expect("Bad Layout"),
            ValueType::I16 => std::alloc::Layout::from_size_align(size * 2, 2).expect("Bad Layout"),
            ValueType::I32 => std::alloc::Layout::from_size_align(size * 4, 4).expect("Bad Layout"),
            ValueType::I64 => std::alloc::Layout::from_size_align(size * 8, 8).expect("Bad Layout"),
            ValueType::F32 => std::alloc::Layout::from_size_align(size * 4, 4).expect("Bad Layout"),
            ValueType::F64 => std::alloc::Layout::from_size_align(size * 8, 8).expect("Bad Layout"),
            _ => panic!("Invalid value type"),
        }
    }

    fn deallocate_helper(reference_table: &mut HashMap<u64, MemoryObject>, reference: u64) -> Result<(), Fault> {
        let object = reference_table.remove(&reference);
        if let Some(object) = object {
            match object {
                MemoryObject::Pointer(pointer, size, value_type) => {
                    let layout = Self::get_layout(size, value_type);
                    unsafe { std::alloc::dealloc(pointer, layout) };
                }
                MemoryObject::List(list) => {
                    let list = unsafe { Box::from_raw(list) };

                    for value in list.iter() {
                        match value {
                            Value::MemoryRef(index) => {
                                Self::deallocate_helper(reference_table, *index)?;
                            }
                            Value::ArrayRef(index) => {
                                Self::deallocate_helper(reference_table, *index)?;
                            }
                            Value::ObjectRef(index) => {
                                Self::deallocate_helper(reference_table, *index)?;
                            }
                            Value::StringRef(index) => {
                                Self::deallocate_helper(reference_table, *index)?;
                            }
                            _ => {}
                        }
                    }
                }
                MemoryObject::StringTableRef(_, _) => {}
                MemoryObject::String(pointer) => {
                    let _ = unsafe { Box::from_raw(pointer as *mut str) };
                }
                MemoryObject::Object(object) => {
                    let _ = unsafe { Box::from_raw(object) };
                }
                MemoryObject::Null => {}
            }
        }
        Ok(())
    }
    pub fn deallocate(&mut self, reference: u64) -> Result<(), Fault> {
        loop {
            match self.reference_table.try_write() {
                Ok(mut reference_table) => {
                    Self::deallocate_helper(&mut reference_table, reference)?;
                    return Ok(());
                }
                Err(TryLockError::WouldBlock)=> {}
                Err(TryLockError::Poisoned(_)) => Err(Fault::MemoryError("Poisoned".to_string()))?,
            }
        }
    }

    pub fn allocate_pointer(&mut self, size: usize, value_type: ValueType) -> Result<Value, Fault> {
        let layout = Self::get_layout(size, value_type);
        let pointer = unsafe { std::alloc::alloc(layout) };
        if pointer.is_null() {
            Err(Fault::MemoryError("Failed to allocate memory".to_string()))?;
        }
        let pointer = MemoryObject::Pointer(pointer, size, value_type);
        let index = self.allocate(pointer)?;
        Ok(Value::MemoryRef(index))
    }

    pub fn allocate_list(&mut self, length: usize, size: ValueType) -> Result<Value, Fault> {
        let mut list = Vec::with_capacity(length);
        for _ in 0..length {
            list.push(Value::new(size));
        }
        let list = list.into_boxed_slice();
        let list = Box::into_raw(list);
        let list = list as *mut [Value];
        let list = MemoryObject::List(list);
        let index = self.allocate(list)?;
        Ok(Value::MemoryRef(index))
    }

    pub fn access_list(&self, reference: u64, index: u64) -> Result<Value, Fault> {
        let list = self.get(reference)?;
        match list {
            MemoryObject::List(list) => {
                let list = unsafe { &mut *list };
                let value = list.get(index as usize).ok_or(Fault::IndexOutOfBounds)?;

                Ok(value.clone())
            }
            MemoryObject::Null => Err(Fault::NullPointerReference),
            _ => Err(Fault::InvalidReference),
        }
    }

    pub fn get_list_length(&self, reference: u64) -> Result<Value, Fault> {
        let list = self.get(reference)?;
        match list {
            MemoryObject::List(list) => {
                let list = unsafe { &mut *list };
                Ok(Value::U64(list.len() as u64))
            }
            MemoryObject::Null => Err(Fault::NullPointerReference),
            _ => Err(Fault::InvalidReference),
        }
    }

    pub fn store_list(&mut self, reference: u64, index: u64, value: Value) -> Result<(), Fault> {
        let list = self.get(reference)?;
        match list {
            MemoryObject::List(list) => {
                let list = unsafe { &mut *list };
                list[index as usize] = value;
                Ok(())
            }
            MemoryObject::Null => Err(Fault::NullPointerReference),
            _ => Err(Fault::InvalidReference),
        }
    }

    pub fn allocate_string(&mut self, string: &str) -> Result<Value, Fault> {
        let string = MemoryObject::String(string);
        let index = self.allocate(string)?;
        Ok(Value::MemoryRef(index))
    }

    pub fn get_string<'a>(&'a self, reference: u64, module: &'a Module) -> Result<&str, Fault> {
        let string = self.get(reference)?;
        match string {
            MemoryObject::String(string) => {
                let string = unsafe { &*string };
                Ok(string)
            }
            MemoryObject::StringTableRef(path, index) => {
                let string = module.get_string(&path, index).ok_or(Fault::InvalidReference)?;
                Ok(string)
            }
            MemoryObject::Null => Err(Fault::NullPointerReference),
            _ => Err(Fault::InvalidReference),
        }
    }

    pub fn allocate_string_ref(&mut self, path: &StringTablePath, table_index: u64) -> Result<Value, Fault> {
        let string = MemoryObject::StringTableRef(path.clone(), table_index);
        let index = self.allocate(string)?;
        loop {
            match self.string_lookup_table.try_write() {
                Ok(mut string_lookup_table) => {
                    string_lookup_table.insert((path.clone(), table_index), index);

                    return Ok(Value::MemoryRef(index));
                }
                Err(TryLockError::WouldBlock)=> {}
                Err(TryLockError::Poisoned(_)) => Err(Fault::MemoryError("Poisoned".to_string()))?,
            }
        }
    }

    pub fn get_string_ref_from_path(&self, path: &StringTablePath, table_index: u64) -> Result<Value, Fault> {
        loop {
            match self.string_lookup_table.try_read() {
                Ok(string_lookup_table) => {
                    return if let Some(index) = string_lookup_table.get(&(path.clone(), table_index)) {
                        Ok(Value::MemoryRef(*index))
                    } else {
                        Err(Fault::InvalidReference)
                    }
                }
                Err(TryLockError::WouldBlock)=> {}
                Err(TryLockError::Poisoned(_)) => Err(Fault::MemoryError("Poisoned".to_string()))?,
            }
        }
    }

    pub fn concatenate_strings(&mut self, left: u64, right: u64, module: Arc<Module>) -> Result<Value, Fault> {
        let left = self.get_string(left, &module)?;
        let right = self.get_string(right, &module)?;
        let string = format!("{}{}", left, right);
        self.allocate_string(&string)
    }
 }