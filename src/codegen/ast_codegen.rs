use super::context::CompilerContext;
use super::Codegen;
use crate::ast;
use crate::EMPTY_NAME;

impl Codegen for ast::Program {
    unsafe fn codegen(
        &self,
        _ctx: &mut CompilerContext,
        _context: *mut llvm_sys::LLVMContext,
        _module: *mut llvm_sys::LLVMModule,
        _builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        todo!();
    }
}

impl Codegen for ast::Expr {
    unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        use ast::Expr;
        use llvm_sys::core::*;
        use llvm_sys::prelude::LLVMValueRef;
        match self {
            Expr::FunctionCall(call) => {
                let func = call.name.codegen(ctx, context, module, builder);
                let mut args: Vec<LLVMValueRef> = call
                    .args
                    .iter()
                    .cloned()
                    .map(|expr| expr.codegen(ctx, context, module, builder))
                    .collect();
                LLVMBuildCall(
                    builder,
                    func,
                    args.as_mut_ptr(),
                    args.len() as u32,
                    EMPTY_NAME,
                )
            }
            Expr::Binary(l, op, r) => {
                let l_val = l.codegen(ctx, context, module, builder);
                let r_val = r.codegen(ctx, context, module, builder);

                // TODO: This assumes l and r are both ints
                // should have handling for floats as well

                // TODO: LogicalAnd should be typechecked to ensure LHS and RHS
                // are both i1 types. If they are then it works as intended. Otherwise
                // it can give values other than 0 or 1.
                use ast::BinOp;
                match op {
                    BinOp::Plus => LLVMBuildAdd(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Minus => LLVMBuildSub(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Times => LLVMBuildMul(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Divide => LLVMBuildSDiv(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::LogicalAnd => LLVMBuildAnd(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::LogicalOr => LLVMBuildOr(builder, l_val, r_val, EMPTY_NAME),
                }
            }
            Expr::Unary(op, data) => {
                let data_val = data.codegen(ctx, context, module, builder);
                use ast::UnaryOp;
                match op {
                    UnaryOp::Reference => todo!(),
                    UnaryOp::Minus => LLVMBuildNeg(builder, data_val, EMPTY_NAME),
                    UnaryOp::Not => LLVMBuildNot(builder, data_val, EMPTY_NAME),
                }
            }
            Expr::Literal(lit) => lit.codegen(ctx, context, module, builder),
            Expr::Ident(_ident) => todo!(),
        }
    }
}

impl Codegen for ast::Literal {
    unsafe fn codegen(
        &self,
        _ctx: &mut CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        _module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        use ast::Literal;
        use llvm_sys::core::*;

        match self {
            Literal::Int32(value) => {
                let i32_type = LLVMInt32TypeInContext(context);
                // TODO: use better conversion method
                LLVMConstInt(i32_type, *value as u64, 1)
            }
            // TODO: Crashes program if not in a function
            Literal::Str(string) => {
                use std::ffi::CString;
                // TODO: Handle this error
                let converted_string = CString::new(string.as_bytes()).unwrap();
                LLVMBuildGlobalStringPtr(
                    builder,
                    converted_string.as_ptr() as *const i8,
                    EMPTY_NAME,
                )
            }
            Literal::True => {
                let i1_type = LLVMInt1TypeInContext(context);
                LLVMConstInt(i1_type, 1, 0)
            }
            Literal::False => {
                let i1_type = LLVMInt1TypeInContext(context);
                LLVMConstInt(i1_type, 0, 0)
            }
        }
    }
}
