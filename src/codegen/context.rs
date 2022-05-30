use llvm_sys::prelude::LLVMValueRef;
use std::collections::HashMap;

pub struct CompilerContext {
    named_values: HashMap<String, LLVMValueRef>,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            named_values: HashMap::new(),
        }
    }
}
