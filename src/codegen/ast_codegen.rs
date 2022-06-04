use super::Codegen;
use crate::ast;

impl Codegen for ast::Program {
    unsafe fn codegen(
        &self,
        _context: *mut llvm_sys::LLVMContext,
        _module: *mut llvm_sys::LLVMModule,
        _builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        todo!();
    }
}
