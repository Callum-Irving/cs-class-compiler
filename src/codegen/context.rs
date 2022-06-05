use super::symbol::ScopedSymbolTable;
use crate::ast;

pub struct CompilerContext {
    pub symbols: ScopedSymbolTable,
    pub current_type: Vec<ast::Type>,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            symbols: ScopedSymbolTable::new(),
            current_type: vec![],
        }
    }
}
