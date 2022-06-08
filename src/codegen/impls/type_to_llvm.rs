use crate::type_checker::typed_ast::Type;

use std::os::raw::c_uint;

use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::LLVMContext;

impl Type {
    pub unsafe fn as_llvm_type(&self, context: *mut LLVMContext) -> LLVMTypeRef {
        match self {
            // TODO: Make system dependent?
            Type::Int | Type::UInt => LLVMInt32TypeInContext(context),
            Type::Int8 | Type::UInt8 | Type::Char => LLVMInt8TypeInContext(context),
            Type::Int16 | Type::UInt16 => LLVMInt16TypeInContext(context),
            Type::Int32 | Type::UInt32 => LLVMInt32TypeInContext(context),
            Type::Int64 | Type::UInt64 => LLVMInt64TypeInContext(context),
            Type::Bool => LLVMInt1TypeInContext(context),
            Type::Array(inner, len) => {
                let inner_type = inner.as_llvm_type(context);
                LLVMArrayType(inner_type, *len as c_uint)
            }
            Type::Ref(inner) => {
                let inner_type = inner.as_llvm_type(context);
                LLVMPointerType(inner_type, 0)
            }
            Type::Str => {
                let i8_type = LLVMInt8TypeInContext(context);
                LLVMPointerType(i8_type, 0)
            }
            Type::NoneType => LLVMVoidTypeInContext(context),
        }
    }
}
