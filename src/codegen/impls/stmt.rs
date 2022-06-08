use super::EMPTY_NAME;
use crate::codegen::context::CompilerContext;
use crate::type_checker::typed_ast;

use crate::codegen::symbol::{Symbol, SymbolType};

use llvm_sys::core::*;

impl typed_ast::Stmt {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        llvm_context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) {
        use typed_ast::Stmt;

        match self {
            Stmt::ExprStmt(expr) => {
                expr.codegen(ctx, llvm_context, module, builder);
            }
            Stmt::ConstDef(def) => {
                let alloca = LLVMBuildAlloca(
                    builder,
                    def.binding.ty.as_llvm_type(llvm_context),
                    EMPTY_NAME,
                );
                let val = def.value.codegen(ctx, llvm_context, module, builder);

                LLVMBuildStore(builder, val, alloca);

                ctx.symbols
                    .add_symbol(
                        def.binding.name.clone(),
                        Symbol::new(alloca, SymbolType::Const),
                    )
                    .unwrap();
            }
            Stmt::VarDef(def) => {
                let alloca = LLVMBuildAlloca(
                    builder,
                    def.binding.ty.as_llvm_type(llvm_context),
                    EMPTY_NAME,
                );
                let val = def.value.codegen(ctx, llvm_context, module, builder);

                LLVMBuildStore(builder, val, alloca);

                ctx.symbols
                    .add_symbol(
                        def.binding.name.clone(),
                        Symbol::new(alloca, SymbolType::Var),
                    )
                    .unwrap();
            }
            Stmt::ReturnStmt(expr) => {
                let value = expr.codegen(ctx, llvm_context, module, builder);
                LLVMBuildRet(builder, value);
            }
            _ => todo!("not implemented for stmt"),
        }
    }
}
