use super::EMPTY_NAME;
use crate::codegen::context::CompilerContext;
use crate::type_checker::typed_ast;

use std::os::raw::c_ulonglong;

use llvm_sys::core::*;

impl typed_ast::Literal {
    pub unsafe fn codegen(
        &self,
        _ctx: &mut CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        _module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        use typed_ast::LiteralInner;

        match &self.val {
            // TODO: This is dependent on context (fixed)
            LiteralInner::Int(value) => {
                // Default to int32 type
                let ty = self.ty.as_llvm_type(context);

                // TODO: use better conversion method
                // TODO: Handle error
                LLVMConstInt(ty, *value as c_ulonglong, 1)
            }
            // TODO: Crashes program if not in a function
            LiteralInner::CStr(string) => {
                use std::ffi::CString;
                // TODO: Handle this error
                let converted_string = CString::new(string.as_bytes()).unwrap();
                LLVMBuildGlobalStringPtr(
                    builder,
                    converted_string.as_ptr() as *const i8,
                    EMPTY_NAME,
                )
            }
            LiteralInner::Bool(val) => {
                let i1_type = LLVMInt1TypeInContext(context);
                LLVMConstInt(i1_type, *val as c_ulonglong, 0)
            }
            LiteralInner::UInt(val) => panic!("HOW)"),
            LiteralInner::Int8(val) => {
                let ty = self.ty.as_llvm_type(context);
                LLVMConstInt(ty, *val as c_ulonglong, 1)
            }
            LiteralInner::Int16(val) => panic!("HOW"),
            LiteralInner::Int32(val) => panic!("HOW"),
            LiteralInner::Int64(val) => panic!("HOW"),
            LiteralInner::UInt8(val) => panic!("HOW"),
            LiteralInner::UInt16(val) => panic!("HOW"),
            LiteralInner::UInt32(val) => panic!("HOW"),
            LiteralInner::UInt64(val) => panic!("HOW"),
            LiteralInner::Str(val) => panic!("String struct not done yet"),
        }
    }
}
