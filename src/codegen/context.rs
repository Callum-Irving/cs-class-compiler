use super::symbol::ScopedSymbolTable;
use crate::ast::Program;
use crate::c_str;
use crate::type_checker::infer_types;

use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::core::*;
use llvm_sys::{LLVMBuilder, LLVMContext, LLVMModule};

pub struct CompilerContext {
    pub symbols: ScopedSymbolTable,
    context: *mut LLVMContext,
    module: *mut LLVMModule,
    builder: *mut LLVMBuilder,
}

impl CompilerContext {
    pub fn new() -> Self {
        unsafe {
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithName(c_str!("main"));
            let builder = LLVMCreateBuilderInContext(context);
            LLVMSetTarget(module, c_str!("x86_64-pc-linux-gnu"));

            Self {
                symbols: ScopedSymbolTable::new(),
                context,
                module,
                builder,
            }
        }
    }

    pub unsafe fn compile_to_file(&mut self, ast: Program, output_file: &str) {
        let ast = infer_types(ast);

        ast.codegen(self, self.context, self.module, self.builder);

        use std::ffi::CString;
        let name = CString::new(output_file).unwrap();
        LLVMPrintModuleToFile(self.module, c_str!("main.ll"), std::ptr::null_mut());
        LLVMWriteBitcodeToFile(self.module, name.as_ptr() as *const i8);
    }
}

impl Drop for CompilerContext {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
            LLVMDisposeModule(self.module);
            LLVMContextDispose(self.context);
        }
    }
}
