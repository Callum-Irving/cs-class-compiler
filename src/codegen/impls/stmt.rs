use super::EMPTY_NAME;
use crate::codegen::context::CompilerContext;
use crate::codegen::error::CodegenError;
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
    ) -> Result<(), CodegenError> {
        use typed_ast::Stmt;

        match self {
            Stmt::ExprStmt(expr) => {
                expr.codegen(ctx, llvm_context, module, builder)?;
            }
            Stmt::BlockStmt(block) => {
                block.codegen(ctx, llvm_context, module, builder)?;
            }
            Stmt::ConstDef(def) => {
                let alloca = LLVMBuildAlloca(
                    builder,
                    def.binding.ty.as_llvm_type(ctx, llvm_context),
                    EMPTY_NAME,
                );
                let val = def.value.codegen(ctx, llvm_context, module, builder)?;

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
                    def.binding.ty.as_llvm_type(ctx, llvm_context),
                    EMPTY_NAME,
                );
                let val = def.value.codegen(ctx, llvm_context, module, builder)?;

                LLVMBuildStore(builder, val, alloca);

                ctx.symbols
                    .add_symbol(
                        def.binding.name.clone(),
                        Symbol::new(alloca, SymbolType::Var),
                    )
                    .unwrap();
            }
            Stmt::ReturnStmt(expr) => {
                let value = expr.codegen(ctx, llvm_context, module, builder)?;
                LLVMBuildRet(builder, value);
            }
            Stmt::IfStmt(if_stmt) => {
                if_stmt.codegen(ctx, llvm_context, module, builder)?;
            }
            Stmt::WhileStmt(while_stmt) => {
                while_stmt.codegen(ctx, llvm_context, module, builder)?;
            }
        }

        Ok(())
    }
}

impl typed_ast::BlockStmt {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        llvm_context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> Result<(), CodegenError> {
        for stmt in self.inners.iter() {
            stmt.codegen(ctx, llvm_context, module, builder)?;
        }

        Ok(())
    }
}

impl typed_ast::IfStmt {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        llvm_context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> Result<(), CodegenError> {
        let cond = self.condition.codegen(ctx, llvm_context, module, builder)?;

        let if_block = LLVMAppendBasicBlockInContext(llvm_context, ctx.current_func(), EMPTY_NAME);
        let else_block =
            LLVMAppendBasicBlockInContext(llvm_context, ctx.current_func(), EMPTY_NAME);
        let final_block =
            LLVMAppendBasicBlockInContext(llvm_context, ctx.current_func(), EMPTY_NAME);

        LLVMBuildCondBr(builder, cond, if_block, else_block);

        // Gen if block
        LLVMPositionBuilderAtEnd(builder, if_block);
        self.body.codegen(ctx, llvm_context, module, builder)?;
        LLVMBuildBr(builder, final_block);

        // Gen else block
        LLVMPositionBuilderAtEnd(builder, else_block);
        if let Some(if_or_else) = self.else_stmt.clone() {
            use typed_ast::IfOrElse;
            match if_or_else {
                IfOrElse::If(if_stmt) => {
                    if_stmt.codegen(ctx, llvm_context, module, builder)?;
                }
                IfOrElse::Else(block) => {
                    block.codegen(ctx, llvm_context, module, builder)?;
                }
            }
        }

        LLVMBuildBr(builder, final_block);

        LLVMPositionBuilderAtEnd(builder, final_block);

        Ok(())
    }
}

impl typed_ast::WhileStmt {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        llvm_context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) -> Result<(), CodegenError> {
        let condition_block =
            LLVMAppendBasicBlockInContext(llvm_context, ctx.current_func(), EMPTY_NAME);
        let body_block =
            LLVMAppendBasicBlockInContext(llvm_context, ctx.current_func(), EMPTY_NAME);
        let final_block =
            LLVMAppendBasicBlockInContext(llvm_context, ctx.current_func(), EMPTY_NAME);
        LLVMBuildBr(builder, condition_block);

        // Conditional break
        LLVMPositionBuilderAtEnd(builder, condition_block);
        let condition = self.condition.codegen(ctx, llvm_context, module, builder)?;
        // If condition == 1 then go to body block, else go to final block
        LLVMBuildCondBr(builder, condition, body_block, final_block);

        // Body block
        LLVMPositionBuilderAtEnd(builder, body_block);
        self.body.codegen(ctx, llvm_context, module, builder)?;
        // Go back to condition check
        LLVMBuildBr(builder, condition_block);

        // Exit
        LLVMPositionBuilderAtEnd(builder, final_block);

        Ok(())
    }
}
