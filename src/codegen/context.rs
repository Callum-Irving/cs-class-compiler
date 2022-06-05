#![allow(dead_code)]

use super::symbol::ScopedSymbolTable;

pub struct CompilerContext {
    pub symbols: ScopedSymbolTable,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            symbols: ScopedSymbolTable::new(),
        }
    }
}
