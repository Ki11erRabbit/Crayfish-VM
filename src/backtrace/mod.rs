use std::fmt::{Display, Formatter};
use crate::program::function::FunctionPath;

pub struct BacktraceInfo {
    backtrace: Vec<BacktraceEntry>,
    unwind_levels: usize,
}

impl BacktraceInfo {
    pub fn new() -> Self {
        BacktraceInfo {
            backtrace: Vec::new(),
            unwind_levels: 0,
        }
    }

    pub fn increment_unwind_levels(&mut self) {
        self.unwind_levels += 1;
    }

    pub fn remove_unwind_levels(&mut self) {
        while self.unwind_levels > 0 {
            self.backtrace.pop();
            self.unwind_levels -= 1;
        }
    }

    pub fn push(&mut self, entry: BacktraceEntry) {
        self.backtrace.push(entry);
    }

    pub fn pop(&mut self) -> Option<BacktraceEntry> {
        self.backtrace.pop()
    }

    pub fn get(&self, index: usize) -> Option<&BacktraceEntry> {
        self.backtrace.get(index)
    }

    pub fn len(&self) -> usize {
        self.backtrace.len()
    }
}

impl Display for BacktraceInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for entry in self.backtrace.iter() {
            write!(f, "{}\n", entry)?;
        }
        Ok(())
    }
}


pub struct BacktraceEntry {
    pub(crate) function_name: FunctionPath,
    pub(crate) line: Option<u64>,
    pub(crate) column: Option<u64>,
}

impl BacktraceEntry {
    pub fn new(function_name: FunctionPath, line: Option<u64>, column: Option<u64>) -> Self {
        BacktraceEntry {
            function_name,
            line,
            column,
        }
    }

    pub fn get_function_name(&self) -> String {
        self.function_name.to_string()
    }

    pub fn get_line(&self) -> Option<u64> {
        self.line
    }

    pub fn get_column(&self) -> Option<u64> {
        self.column
    }
}

impl Display for BacktraceEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.function_name)?;
        if let Some(line) = self.line {
            write!(f, ":{}", line)?;
            if let Some(column) = self.column {
                write!(f, ":{}", column)?;
            }
        }
        Ok(())
    }
}