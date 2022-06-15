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
        ctx: &CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        use typed_ast::ExprInner;

        match &self.val {
            ExprInner::Class(class_expr) => {
                let (llvm_ty, def) = ctx.class(&class_expr.class).unwrap();
                let alloca = LLVMBuildAlloca(builder, *llvm_ty, EMPTY_NAME);

                // TODO: Get rid of this unwrap
                for (name, value) in class_expr.fields.iter() {
                    let i = def.fields.get(name).unwrap().0;
                    let field = LLVMBuildStructGEP(builder, alloca, i as c_uint, EMPTY_NAME);
                    LLVMBuildStore(builder, value.codegen(ctx, context, module, builder), field);
                }

                LLVMBuildLoad(builder, alloca, EMPTY_NAME)
            }
            ExprInner::Array(array_expr) => {
                let i64_type = LLVMInt64TypeInContext(context);
                let ty = self.ty.as_llvm_type(ctx, context);
                let alloca = LLVMBuildAlloca(builder, ty, EMPTY_NAME);
                let zero = LLVMConstInt(i64_type, 0, 0);
                // TODO: Can optimize this
                // Look how clang does it by chaining GEP instructions
                for (i, item) in array_expr.items.iter().enumerate() {
                    let value = item.codegen(ctx, context, module, builder);
                    let index = LLVMConstInt(i64_type, i as c_ulonglong, 0);
                    let array_val = LLVMBuildInBoundsGEP2(
                        builder,
                        ty,
                        alloca,
                        [index, zero].as_mut_ptr(),
                        2,
                        EMPTY_NAME,
                    );
                    LLVMBuildStore(builder, value, array_val);
                }
                LLVMBuildLoad(builder, alloca, EMPTY_NAME)
            }
            ExprInner::IndexExpr(index_expr) => {
                // Use getelementptr instruction
                let data = index_expr.name.codegen(ctx, context, module, builder);
                let index = index_expr.index.codegen(ctx, context, module, builder);

                let ptr = LLVMBuildGEP(builder, data, [index].as_mut_ptr(), 1, EMPTY_NAME);
                LLVMBuildLoad(builder, ptr, EMPTY_NAME)
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
                use typed_ast::BinOp;

                let l_val = binary_expr.lhs.codegen(ctx, context, module, builder);
                let r_val = binary_expr.rhs.codegen(ctx, context, module, builder);

                // TODO: This assumes l and r are both signed ints
                // should have handling for unsigned and floats as well

                // TODO: LogicalAnd should be typechecked to ensure LHS and RHS
                // are both i1 types. If they are then it works as intended. Otherwise
                // it can give values other than 0 or 1.
                match binary_expr.op {
                    BinOp::Plus => LLVMBuildAdd(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Minus => LLVMBuildSub(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Times => LLVMBuildMul(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Divide => LLVMBuildSDiv(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::LogicalAnd => LLVMBuildAnd(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::LogicalOr => LLVMBuildOr(builder, l_val, r_val, EMPTY_NAME),
                    BinOp::Equals => {
                        // TODO: Assert LHS is an ident or indexexpr
                        // Build store
                        // TODO: Somehow don't fully codegen l val because we want the ptr to it
                        // We know that l val has to be bound to some variable for this to work
                        let l_ptr = binary_expr.lhs.codegen_ptr(ctx, context, module, builder);
                        LLVMBuildStore(builder, r_val, l_ptr)
                    }
                    BinOp::Eq => LLVMBuildICmp(
                        builder,
                        llvm_sys::LLVMIntPredicate::LLVMIntEQ,
                        l_val,
                        r_val,
                        EMPTY_NAME,
                    ),
                    BinOp::Ne => LLVMBuildICmp(
                        builder,
                        llvm_sys::LLVMIntPredicate::LLVMIntNE,
                        l_val,
                        r_val,
                        EMPTY_NAME,
                    ),
                    BinOp::Gt => LLVMBuildICmp(
                        builder,
                        llvm_sys::LLVMIntPredicate::LLVMIntSGT,
                        l_val,
                        r_val,
                        EMPTY_NAME,
                    ),
                    BinOp::Gte => LLVMBuildICmp(
                        builder,
                        llvm_sys::LLVMIntPredicate::LLVMIntSGE,
                        l_val,
                        r_val,
                        EMPTY_NAME,
                    ),
                    BinOp::Lt => LLVMBuildICmp(
                        builder,
                        llvm_sys::LLVMIntPredicate::LLVMIntSLT,
                        l_val,
                        r_val,
                        EMPTY_NAME,
                    ),
                    BinOp::Lte => LLVMBuildICmp(
                        builder,
                        llvm_sys::LLVMIntPredicate::LLVMIntSLE,
                        l_val,
                        r_val,
                        EMPTY_NAME,
                    ),
                }
            }
            ExprInner::Unary(unary_expr) => {
                let data_val = unary_expr.data.codegen(ctx, context, module, builder);

                use typed_ast::UnaryOp;
                match unary_expr.op {
                    UnaryOp::Reference => {
                        unary_expr.data.codegen_ptr(ctx, context, module, builder)
                    }
                    UnaryOp::Deref => LLVMBuildLoad(builder, data_val, EMPTY_NAME),
                    UnaryOp::Minus => LLVMBuildNeg(builder, data_val, EMPTY_NAME),
                    UnaryOp::Not => LLVMBuildNot(builder, data_val, EMPTY_NAME),
                }
            }
            ExprInner::Cast(cast_expr) => LLVMBuildCast(
                builder,
                llvm_sys::LLVMOpcode::LLVMSExt,
                cast_expr.original.codegen(ctx, context, module, builder),
                cast_expr.to_type.as_llvm_type(ctx, context),
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

    pub unsafe fn codegen_ptr(
        &self,
        ctx: &CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        use typed_ast::ExprInner;
        if let ExprInner::Ident(ident) = &self.val {
            let symbol = ctx.symbols.get_symbol(&ident).unwrap();
            symbol.value
        } else if let ExprInner::IndexExpr(index_expr) = &self.val {
            let data = index_expr.name.codegen(ctx, context, module, builder);
            let index = index_expr.index.codegen(ctx, context, module, builder);
            LLVMBuildGEP(builder, data, [index].as_mut_ptr(), 1, EMPTY_NAME)
        } else if let ExprInner::Unary(unary_expr) = &self.val {
            // TODO: assert op is deref
            // %3 = i32**
            // need i32*
            self.codegen(ctx, context, module, builder)
        } else {
            panic!("OH NO, bad ptr expr")
        }
    }
}
