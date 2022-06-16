use super::EMPTY_NAME;
use crate::codegen::context::CompilerContext;
use crate::type_checker::typed_ast;

use crate::codegen::symbol::{Symbol, SymbolType};

use std::ffi::CStr;
use std::os::raw::c_uint;

use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;

impl typed_ast::Program {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) {
        for stmt in self.0.iter() {
            use typed_ast::TopLevelStmt;

            match stmt {
                TopLevelStmt::ClassDef(def) => def.codegen(ctx, context, module, builder),
                TopLevelStmt::FunctionDef(def) => def.codegen(ctx, context, module, builder),
                TopLevelStmt::ExternDef(def) => def.codegen(ctx, context, module, builder),
                TopLevelStmt::ConstDef(def) => def.codegen(ctx, context, module, builder),
            };
        }
    }
}

impl typed_ast::ClassDef {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        llvm_context: *mut llvm_sys::LLVMContext,
        _module: *mut llvm_sys::LLVMModule,
        _builder: *mut llvm_sys::LLVMBuilder,
    ) {
        use std::ffi::CString;
        let c_name = CString::new(self.name.as_bytes()).unwrap();
        let struct_ty = LLVMStructCreateNamed(llvm_context, c_name.as_ptr() as *const i8);
        let mut element_types: Vec<LLVMTypeRef> = self
            .fields
            .iter()
            .map(|(_, ty)| ty.as_llvm_type(ctx, llvm_context))
            .collect();
        LLVMStructSetBody(
            struct_ty,
            element_types.as_mut_ptr(),
            element_types.len() as u32,
            0,
        );

        ctx.add_class(struct_ty, self.clone());
    }
}

impl typed_ast::FunctionDef {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        builder: *mut llvm_sys::LLVMBuilder,
    ) {
        // Turn args into vec of llvm types
        let mut args: Vec<LLVMTypeRef> = self
            .params
            .iter()
            .map(|t| t.ty.as_llvm_type(ctx, context))
            .collect();

        let return_type = self.return_type.as_llvm_type(ctx, context);

        let func_type = LLVMFunctionType(return_type, args.as_mut_ptr(), args.len() as c_uint, 0);

        // Convert name to a C string
        use std::ffi::CString;
        let converted = CString::new(self.name.as_bytes()).unwrap();

        let func = LLVMAddFunction(module, converted.as_ptr() as *const i8, func_type);
        let block = LLVMAppendBasicBlockInContext(context, func, EMPTY_NAME);
        LLVMPositionBuilderAtEnd(builder, block);

        // Add function to stack
        ctx.add_func(func);
        ctx.symbols.push_scope();

        // Add arguments to current scope
        for (i, param) in self.params.iter().enumerate() {
            let alloca = LLVMBuildAlloca(builder, param.ty.as_llvm_type(ctx, context), EMPTY_NAME);
            let value = LLVMGetParam(func, i as u32);
            LLVMBuildStore(builder, value, alloca);
            ctx.symbols
                .add_symbol(param.name.clone(), Symbol::new(alloca, SymbolType::Const))
                .unwrap();
        }

        for stmt in self.body.inners.iter() {
            stmt.codegen(ctx, context, module, builder);
        }

        ctx.symbols.pop_scope().unwrap();
        ctx.pop_func();

        // Add ret void if it is a void function so that LLVM is happy.
        if matches!(self.return_type, typed_ast::Type::NoneType) {
            LLVMBuildRetVoid(builder);
        }

        // Add function to symbol table so that it can be called.
        ctx.symbols
            .add_symbol(self.name.clone(), Symbol::new(func, SymbolType::Func))
            .unwrap();
    }
}

impl typed_ast::ExternDef {
    pub unsafe fn codegen(
        &self,
        ctx: &mut CompilerContext,
        context: *mut llvm_sys::LLVMContext,
        module: *mut llvm_sys::LLVMModule,
        _builder: *mut llvm_sys::LLVMBuilder,
    ) {
        let mut args: Vec<LLVMTypeRef> = self
            .params
            .iter()
            .map(|t| t.ty.as_llvm_type(ctx, context))
            .collect();

        let return_type = self.return_type.as_llvm_type(ctx, context);

        let func_type = LLVMFunctionType(
            return_type,
            args.as_mut_ptr(),
            args.len() as c_uint,
            0, // TODO: Support varargs
        );

        // Convert name to C string
        use std::ffi::CString;
        let converted = CString::new(self.name.as_bytes()).unwrap();

        let func = LLVMAddFunction(module, converted.as_ptr() as *const i8, func_type);
        ctx.symbols
            .add_symbol(self.name.clone(), Symbol::new(func, SymbolType::Func))
            .unwrap();
    }
}

impl typed_ast::GlobalConstDef {
    pub unsafe fn codegen(
        &self,
        _ctx: &mut CompilerContext,
        _context: *mut llvm_sys::LLVMContext,
        _module: *mut llvm_sys::LLVMModule,
        _builder: *mut llvm_sys::LLVMBuilder,
    ) {
        todo!();
    }
}
