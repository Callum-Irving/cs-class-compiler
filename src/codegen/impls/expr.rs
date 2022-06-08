use super::EMPTY_NAME;
use crate::codegen::context::CompilerContext;
use crate::type_checker::typed_ast;

use crate::codegen::symbol::SymbolType;

use std::os::raw::{c_uint, c_ulonglong};

use llvm_sys::core::*;
use llvm_sys::prelude::LLVMValueRef;

impl typed_ast::Expr {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        use typed_ast::ExprInner;

        match &self.val {
            ExprInner::Array(array_expr) => {
                let ty = self.ty.as_llvm_type(context);
                LLVMBuildAlloca(builder, ty, EMPTY_NAME)
                // TODO: Store values in array
            }
            ExprInner::IndexExpr(index_expr) => {
                todo!();
            }
            ExprInner::FunctionCall(call) => {
                let func = call.name.codegen(ctx, context, module, builder);
                // TODO: Handle error better
                let mut args: Vec<LLVMValueRef> = call
                    .args
                    .iter()
                    .map(|expr| expr.codegen(ctx, context, module, builder))
                    .collect();

                LLVMBuildCall(
                    builder,
                    func,
                    args.as_mut_ptr(),
                    args.len() as c_uint,
                    EMPTY_NAME,
                )
            }
            ExprInner::Binary(binary_expr) => {
                let l_val = binary_expr.lhs.codegen(ctx, context, module, builder);
                let r_val = binary_expr.rhs.codegen(ctx, context, module, builder);

                // TODO: This assumes l and r are both ints
                // should have handling for floats as well

                // TODO: LogicalAnd should be typechecked to ensure LHS and RHS
                // are both i1 types. If they are then it works as intended. Otherwise
                // it can give values other than 0 or 1.
                use typed_ast::BinOp;
                match binary_expr.op {
                    BinOp::Plus => LLVMBuildAdd(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Minus => LLVMBuildSub(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Times => LLVMBuildMul(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Divide => LLVMBuildSDiv(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::LogicalAnd => LLVMBuildAnd(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::LogicalOr => LLVMBuildOr(builder, l_val, r_val, EMPTY_NAME),
                }
            }
            ExprInner::Unary(unary_expr) => {
                let data_val = unary_expr.data.codegen(ctx, context, module, builder);

                use typed_ast::UnaryOp;
                match unary_expr.op {
                    UnaryOp::Reference => data_val,
                    UnaryOp::Minus => LLVMBuildNeg(builder, data_val, EMPTY_NAME),
                    UnaryOp::Not => LLVMBuildNot(builder, data_val, EMPTY_NAME),
                }
            }
            ExprInner::Cast(cast_expr) => LLVMBuildCast(
                builder,
                llvm_sys::LLVMOpcode::LLVMBitCast,
                cast_expr.original.codegen(ctx, context, module, builder),
                cast_expr.to_type.as_llvm_type(context),
                EMPTY_NAME,
            ),
            ExprInner::Literal(lit) => lit.codegen(ctx, context, module, builder),
            ExprInner::Ident(ident) => {
                let symbol = ctx.symbols.get_symbol(&ident).unwrap();
                match symbol.ty {
                    SymbolType::Const => LLVMBuildLoad(builder, symbol.value, EMPTY_NAME),
                    SymbolType::Var => LLVMBuildLoad(builder, symbol.value, EMPTY_NAME),
                    SymbolType::Func => symbol.value,
                }
            }
        }
    }
}

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
            LiteralInner::Str(string) => {
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
            _ => panic!("OOPSIE, INT NOT CODEGENED YET"),
        }
    }
}
