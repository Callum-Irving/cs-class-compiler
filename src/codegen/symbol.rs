use llvm_sys::prelude::LLVMValueRef;
use std::collections::HashMap;

use super::CodegenError;

pub struct ScopedSymbolTable {
    stack: Vec<HashMap<String, LLVMValueRef>>,
}

impl ScopedSymbolTable {
    /// Create a new scoped symbol table with a global scope.
    pub fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
        }
    }

    /// Get a symbol by name. Returns the match in the most nested scope.
    pub fn get_symbol(&self, name: &str) -> Option<&LLVMValueRef> {
        self.stack.iter().rev().find_map(|map| map.get(name))
    }

    /// Adds a symbol to the last scope.
    pub fn add_symbol(&mut self, name: String, value: LLVMValueRef) -> Result<(), CodegenError> {
        self.stack.last_mut().ok_or(CodegenError::EmptySymbolTable)?.insert(name, value);
        Ok(())
    }

    /// Add an empty scope to the symbol table.
    pub fn add_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    /// Remove the last scope as long as there is more than one scope left.
    pub fn pop_scope(&mut self) -> Result<(), ()> {
        if self.stack.len() <= 1 {
            return Err(());
        }

        self.stack.pop();
        Ok(())
    }
}
