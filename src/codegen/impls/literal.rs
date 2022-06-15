use super::EMPTY_NAME;
use crate::codegen::context::CompilerContext;
use crate::type_checker::typed_ast;

use std::os::raw::c_ulonglong;

use llvm_sys::core::*;

impl typed_ast::Literal {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        _module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        use typed_ast::LiteralInner;

        match &self.val {
            // TODO: This is dependent on context (fixed)
            LiteralInner::Int(value) => {
                // Default to int32 type
                let ty = self.ty.as_llvm_type(ctx, context);

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
            LiteralInner::UInt(val) => {
                let uint_type = LLVMInt32TypeInContext(context);
                LLVMConstInt(uint_type, *val as c_ulonglong, 0)
            }
            LiteralInner::Int8(val) => {
                let i8_type = LLVMInt8TypeInContext(context);
                LLVMConstInt(i8_type, *val as c_ulonglong, 1)
            }
            LiteralInner::Int16(val) => {
                let i16_type = LLVMInt16TypeInContext(context);
                LLVMConstInt(i16_type, *val as c_ulonglong, 1)
            }
            LiteralInner::Int32(val) => {
                let i32_type = LLVMInt32TypeInContext(context);
                LLVMConstInt(i32_type, *val as c_ulonglong, 1)
            }
            LiteralInner::Int64(val) => {
                let i64_type = LLVMInt64TypeInContext(context);
                LLVMConstInt(i64_type, *val as c_ulonglong, 1)
            }
            LiteralInner::UInt8(val) => {
                let u8_type = LLVMInt8TypeInContext(context);
                LLVMConstInt(u8_type, *val as c_ulonglong, 0)
            }
            LiteralInner::UInt16(val) => {
                let u16_type = LLVMInt16TypeInContext(context);
                LLVMConstInt(u16_type, *val as c_ulonglong, 0)
            }
            LiteralInner::UInt32(val) => {
                let u32_type = LLVMInt32TypeInContext(context);
                LLVMConstInt(u32_type, *val as c_ulonglong, 0)
            }
            LiteralInner::UInt64(val) => {
                let u64_type = LLVMInt64TypeInContext(context);
                LLVMConstInt(u64_type, *val as c_ulonglong, 0)
            }
            LiteralInner::Str(_val) => todo!("String struct not done yet"),
        }
    }
}
