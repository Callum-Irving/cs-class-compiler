#![allow(dead_code)]

pub mod context;
mod symbol;
mod ast_codegen;

use crate::c_str;

use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::{LLVMBuilder, LLVMContext, LLVMModule};
use std::os::raw::c_ulonglong;

pub trait Codegen {
    unsafe fn codegen(
        &self,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
    ) -> LLVMValueRef;
}

pub enum CodegenError {
    EmptySymbolTable,
}

pub struct Int32Expr(pub i32);

impl Codegen for Int32Expr {
    unsafe fn codegen(
        &self,
        context: *mut LLVMContext,
        _module: *mut LLVMModule,
        _builder: *mut LLVMBuilder,
    ) -> LLVMValueRef {
        let i32_type = LLVMInt32TypeInContext(context);
        LLVMConstInt(i32_type, self.0 as c_ulonglong, 0)
    }
}

pub struct BinaryExpr {
    pub op: String,
    pub lhs: Box<dyn Codegen>,
    pub rhs: Box<dyn Codegen>,
}

impl Codegen for BinaryExpr {
    unsafe fn codegen(
        &self,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
    ) -> LLVMValueRef {
        match self.op.as_str() {
            "+" => LLVMBuildAdd(
                builder,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            "-" => LLVMBuildSub(
                builder,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            "*" => LLVMBuildMul(
                builder,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            "/" => LLVMBuildUDiv(
                builder,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            ">" => LLVMBuildICmp(
                builder,
                llvm_sys::LLVMIntPredicate::LLVMIntUGT,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            ">=" => LLVMBuildICmp(
                builder,
                llvm_sys::LLVMIntPredicate::LLVMIntUGE,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            "<" => LLVMBuildICmp(
                builder,
                llvm_sys::LLVMIntPredicate::LLVMIntULT,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            "<=" => LLVMBuildICmp(
                builder,
                llvm_sys::LLVMIntPredicate::LLVMIntULE,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            "==" => LLVMBuildICmp(
                builder,
                llvm_sys::LLVMIntPredicate::LLVMIntEQ,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            "!=" => LLVMBuildICmp(
                builder,
                llvm_sys::LLVMIntPredicate::LLVMIntNE,
                self.lhs.codegen(context, module, builder),
                self.rhs.codegen(context, module, builder),
                c_str!(""),
            ),
            _ => panic!("Invalid op for binary expr: {}", self.op),
        }
    }
}
