use super::context::CompilerContext;
use super::Codegen;
use crate::ast;
use crate::c_str;
use crate::EMPTY_NAME;

use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use num_traits::cast::ToPrimitive;
use std::os::raw::{c_uint, c_ulonglong};

impl Codegen for ast::Program {
    unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        for stmt in self.0.iter() {
            use ast::TopLevelStmt;

            let _ = match stmt {
                TopLevelStmt::ConstDef(def) => todo!(),
                TopLevelStmt::ExternDef(def) => def.codegen(ctx, context, module, builder),
                TopLevelStmt::FunctionDef(def) => def.codegen(ctx, context, module, builder),
            };
        }

        use llvm_sys::LLVMValue;
        return std::ptr::null::<LLVMValue>() as LLVMValueRef;
    }
}

impl Codegen for ast::ExternDef {
    unsafe fn codegen(
        &self,
        ctx: &mut super::context::CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        _builder: *mut llvm_sys::LLVMBuilder,
    ) -> LLVMValueRef {
        let args: Vec<LLVMTypeRef> = self
            .params
            .iter()
            .cloned()
            .map(|t| t.ty.as_llvm_type(context))
            .collect();
        let return_type = if let Some(t) = &self.return_type {
            t.as_llvm_type(context)
        } else {
            LLVMVoidTypeInContext(context)
        };
        let func_type =
            LLVMFunctionType(return_type, args.as_ptr() as *mut _, args.len() as u32, 0);

        // Convert name to C string
        use std::ffi::CString;
        let converted = CString::new(self.name.0.as_bytes()).unwrap();

        let func = LLVMAddFunction(module, converted.as_ptr() as *const i8, func_type);
        ctx.symbols.add_symbol(self.name.0.clone(), func).unwrap();
        func
    }
}

impl Codegen for ast::FunctionDef {
    unsafe fn codegen(
        &self,
        ctx: &mut super::context::CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> llvm_sys::prelude::LLVMValueRef {
        // Turn args into vec of llvm types
        let args: Vec<LLVMTypeRef> = self
            .params
            .iter()
            .cloned()
            .map(|t| t.ty.as_llvm_type(context))
            .collect();

        let return_type = if let Some(t) = &self.return_type {
            t.as_llvm_type(context)
        } else {
            LLVMVoidTypeInContext(context)
        };
        let func_type = LLVMFunctionType(
            return_type,
            args.as_ptr() as *mut _,
            args.len() as c_uint,
            0,
        );

        // Convert name to a C string
        use std::ffi::CString;
        let converted = CString::new(self.name.0.as_bytes()).unwrap();

        let func = LLVMAddFunction(module, converted.as_ptr() as *const i8, func_type);
        let block = LLVMAppendBasicBlockInContext(context, func, c_str!(""));
        LLVMPositionBuilderAtEnd(builder, block);

        for stmt in self.body.0.iter() {
            stmt.codegen(ctx, context, module, builder);
        }

        if self.return_type.is_none() {
            LLVMBuildRetVoid(builder);
        }

        ctx.symbols.add_symbol(self.name.0.clone(), func).unwrap();
        func
    }
}

impl ast::Stmt {
    unsafe fn codegen(
        &self,
        ctx: &mut super::context::CompilerContext,
        llvm_context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) {
        use ast::Stmt;
        match self {
            Stmt::ExprStmt(expr) => {
                expr.codegen(ctx, llvm_context, module, builder);
            }
            Stmt::VarDef(def) => {
                let val = def.value.codegen(ctx, llvm_context, module, builder);
                ctx.symbols
                    .add_symbol(def.binding.name.0.clone(), val)
                    .unwrap();
            }
            _ => todo!("not implemented for stmt"),
        }
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
        use ast::ExprInner;
        match &self.val {
            ExprInner::FunctionCall(call) => {
                let func = call.name.codegen(ctx, context, module, builder);
                let args: Vec<LLVMValueRef> = call
                    .args
                    .iter()
                    .cloned()
                    .map(|expr| expr.codegen(ctx, context, module, builder))
                    .collect();

                LLVMBuildCall(
                    builder,
                    func,
                    args.as_ptr() as *mut _,
                    args.len() as c_uint,
                    EMPTY_NAME,
                )
            }
            ExprInner::Binary(l, op, r) => {
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
            ExprInner::Unary(op, data) => {
                let data_val = data.codegen(ctx, context, module, builder);
                use ast::UnaryOp;
                match op {
                    UnaryOp::Reference => todo!(),
                    UnaryOp::Minus => LLVMBuildNeg(builder, data_val, EMPTY_NAME),
                    UnaryOp::Not => LLVMBuildNot(builder, data_val, EMPTY_NAME),
                }
            }
            ExprInner::Literal(lit) => lit.codegen(ctx, context, module, builder),
            ExprInner::Ident(ident) => *ctx.symbols.get_symbol(&ident.0).unwrap(),
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
        use ast::LiteralInner;

        match &self.val {
            // TODO: This is dependent on context (fixed)
            LiteralInner::Int(value) => {
                // Default to int32 type
                let ty = self
                    .ty
                    .as_ref()
                    .and_then(|t| Some(t.as_llvm_type(context)))
                    .unwrap_or(LLVMInt32TypeInContext(context));

                // TODO: use better conversion method
                // TODO: Handle error
                LLVMConstInt(ty, value.to_i32().unwrap() as c_ulonglong, 1)
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
        }
    }
}

impl ast::Type {
    unsafe fn as_llvm_type(&self, context: *mut llvm_sys::LLVMContext) -> LLVMTypeRef {
        use ast::Type;

        match self {
            // TODO: Get the C int type for the current system for Int and UInt types
            Type::Int | Type::UInt => LLVMInt32TypeInContext(context),
            Type::Int8 | Type::UInt8 | Type::Char => LLVMInt8TypeInContext(context),
            Type::Int16 | Type::UInt16 => LLVMInt16TypeInContext(context),
            Type::Int32 | Type::UInt32 => LLVMInt32TypeInContext(context),
            Type::Int64 | Type::UInt64 => LLVMInt64TypeInContext(context),
            Type::Bool => LLVMInt1TypeInContext(context),
            Type::Str => {
                let i8_type = LLVMInt8TypeInContext(context);
                LLVMPointerType(i8_type, 0)
            }
            Type::Array(inner) => {
                let inner_type = inner.as_llvm_type(context);
                LLVMPointerType(inner_type, 0)
            }
        }
    }
}
