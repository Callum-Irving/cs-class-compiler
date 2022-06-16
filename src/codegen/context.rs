use super::symbol::{ScopedSymbolTable, Symbol};
use crate::ast::Program;
use crate::c_str;
use crate::codegen::error::CodegenError;
use crate::type_checker::inference::infer_types_pass;
use crate::type_checker::typed_ast::ClassDef;

use std::collections::HashMap;

use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::core::*;
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};
use llvm_sys::{LLVMBuilder, LLVMContext, LLVMModule};

pub struct CompilerContext {
    pub symbols: ScopedSymbolTable<Symbol>,
    classes: HashMap<String, (LLVMTypeRef, ClassDef)>,
    func_stack: Vec<LLVMValueRef>,
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
                classes: HashMap::new(),
                func_stack: vec![],
                context,
                module,
                builder,
            }
        }
    }

    pub unsafe fn compile_to_file(
        &mut self,
        ast: Program,
        output_file: &str,
    ) -> Result<(), CodegenError> {
        let ast = infer_types_pass(ast);

        ast.codegen(self, self.context, self.module, self.builder)?;

        use std::ffi::CString;
        let name = CString::new(output_file).unwrap();
        LLVMPrintModuleToFile(self.module, c_str!("main.ll"), std::ptr::null_mut());
        LLVMWriteBitcodeToFile(self.module, name.as_ptr() as *const i8);

        Ok(())
    }

    pub fn add_func(&mut self, func: LLVMValueRef) {
        self.func_stack.push(func);
    }

    pub fn pop_func(&mut self) {
        self.func_stack.pop();
    }

    pub fn current_func(&self) -> LLVMValueRef {
        *self.func_stack.last().unwrap()
    }

    pub fn add_class(&mut self, ty: LLVMTypeRef, class: ClassDef) {
        self.classes.insert(class.name.clone(), (ty, class));
    }

    pub fn class(&self, name: &str) -> Option<&(LLVMTypeRef, ClassDef)> {
        self.classes.get(name)
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
