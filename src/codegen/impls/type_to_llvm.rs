use crate::codegen::context::CompilerContext;
use crate::type_checker::typed_ast::Type;

use std::os::raw::c_uint;

use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::LLVMContext;

impl Type {
    pub unsafe fn as_llvm_type(
        &self,
        ctx: &CompilerContext,
        llvm_context: *mut LLVMContext,
    ) -> LLVMTypeRef {
        match self {
            // TODO: Make system dependent?
            Type::Class(name) => {
                // TODO: Don't unwrap
                ctx.class(name).unwrap().0
            }
            Type::Int | Type::UInt => LLVMInt32TypeInContext(llvm_context),
            Type::Int8 | Type::UInt8 | Type::Char => LLVMInt8TypeInContext(llvm_context),
            Type::Int16 | Type::UInt16 => LLVMInt16TypeInContext(llvm_context),
            Type::Int32 | Type::UInt32 => LLVMInt32TypeInContext(llvm_context),
            Type::Int64 | Type::UInt64 => LLVMInt64TypeInContext(llvm_context),
            Type::Bool => LLVMInt1TypeInContext(llvm_context),
            Type::Array(inner, len) => {
                let inner_type = inner.as_llvm_type(ctx, llvm_context);
                LLVMArrayType(inner_type, *len as c_uint)
            }
            Type::Ref(inner) => {
                let inner_type = inner.as_llvm_type(ctx, llvm_context);
                LLVMPointerType(inner_type, 0)
            }
            Type::Str => todo!("Write a string struct"),
            Type::CStr => {
                let i8_type = LLVMInt8TypeInContext(llvm_context);
                LLVMPointerType(i8_type, 0)
            }
            Type::NoneType => LLVMVoidTypeInContext(llvm_context),
        }
    }
}
