mod ast;
mod codegen;
mod lexer;
mod type_checker;

use codegen::*;

use lalrpop_util::lalrpop_mod;

use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use std::ptr;

lalrpop_mod!(pub grammar);

#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

pub const EMPTY_NAME: *const i8 = c_str!("");

// POSIX system calls
use std::ffi::c_void;
use std::os::raw::c_int;
extern "C" {
    fn write(fd: c_int, buf: *const c_void, count: usize) -> c_int;
    fn read(fdi: c_int, buf: *const c_void, count: usize) -> c_int;
}

/// Write a string to stdout using POSIX write system call.
fn write_safe(s: &str) -> i32 {
    unsafe {
        let res = write(0, s.as_bytes().as_ptr() as *const c_void, s.len());
        return res;
    }
}

/// Reads until a newline from stdout using POSIX read call.
fn read_safe(buf: &mut [u8]) -> i32 {
    unsafe {
        let res = read(1, buf.as_mut_ptr() as *mut c_void, 10);
        return res;
    }
}

fn main() {
    let mut buf = [0u8; 20];
    for i in 0..20 {
        buf[i] = b'a' + i as u8;
    }
    buf[19] = b'\n';
    unsafe {
        write(0, buf.as_ptr() as *const std::ffi::c_void, 20);
    }
    write_safe("Hello, world!\n");
    let mut buf = vec![0u8; 128];
    let status = read_safe(&mut buf);
    print!("{}: {}", status, String::from_utf8(buf).unwrap());

    // let src = r#"fun main() -> void {
    //     println!("Hello, world!")
    //     var arr = [0, -1, 65, 23, 34]
    //     for (value in arr) {
    //     	print!(value, ", ")
    //     }
    // }
    // "#;
    // let lex = Token::lexer(src);
    // for tok in lex {
    //     if matches!(tok, Token::Error) {
    //         println!("{:?}", tok);
    //     }
    // }

    unsafe {
        // Create stuff
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithName(c_str!("main"));
        let builder = LLVMCreateBuilderInContext(context);

        // Set target triple
        // TODO: Should depend on system
        LLVMSetTarget(module, c_str!("x86_64-pc-linux-gnu"));

        // Commonly used types
        let i32_type = LLVMInt32TypeInContext(context);
        let i8_type = LLVMInt8TypeInContext(context);
        let i8_ptr_type = LLVMPointerType(i8_type, 0);

        // External functions
        let printf_fn_type = LLVMFunctionType(i32_type, [i8_ptr_type].as_ptr() as *mut _, 1, 1);
        let printf_fn = LLVMAddFunction(module, c_str!("printf"), printf_fn_type);

        // Create 35 + 34 expr
        let a = Int32Expr(35);
        let b = Int32Expr(34);
        let binary_expr = BinaryExpr {
            op: "+".to_owned(),
            lhs: Box::new(a),
            rhs: Box::new(b),
        };

        // Main function
        // let main_fn_type = LLVMFunctionType(void_type, ptr::null_mut(), 0, 0);
        let main_fn_type = LLVMFunctionType(i32_type, ptr::null_mut(), 0, 0);
        let main_fn = LLVMAddFunction(module, c_str!("main"), main_fn_type);
        let main_block = LLVMAppendBasicBlockInContext(context, main_fn, c_str!(""));
        LLVMPositionBuilderAtEnd(builder, main_block);

        let mut compiler = codegen::context::CompilerContext::new();
        let res = binary_expr.codegen(&mut compiler, context, module, builder);

        let i32_fmt_string = LLVMBuildGlobalStringPtr(builder, c_str!("Result: %d\n"), c_str!(""));
        let args = vec![i32_fmt_string, res];
        LLVMBuildCall(builder, printf_fn, args.as_ptr() as *mut _, 2, c_str!(""));

        // LLVMBuildRetVoid(builder);
        LLVMBuildRet(builder, LLVMConstInt(i32_type, 0, 0));

        // Write the bitcode to a file
        LLVMPrintModuleToFile(module, c_str!("main.ll"), ptr::null_mut());
        LLVMWriteBitcodeToFile(module, c_str!("main.bc"));

        // Dispose of all stuff
        LLVMDisposeBuilder(builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }

    /*
    unsafe {
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithName(c_str!("main"));
        let builder = LLVMCreateBuilderInContext(context);

        LLVMSetTarget(module, c_str!("x86_64-pc-linux-gnu"));

        let i8_type = LLVMInt8TypeInContext(context);
        let i8_ptr_type = LLVMPointerType(i8_type, 0);
        let i32_type = LLVMInt32TypeInContext(context);

        // Add printf function
        let log_func_type = LLVMFunctionType(i32_type, [i8_ptr_type].as_ptr() as *mut _, 1, 1);
        let log_func = LLVMAddFunction(module, c_str!("printf"), log_func_type);

        // Function to double a number
        let double_func_type = LLVMFunctionType(i32_type, [i32_type].as_ptr() as *mut _, 1, 0);
        let double_func = LLVMAddFunction(module, c_str!("double"), double_func_type);
        let double_block = LLVMAppendBasicBlockInContext(context, double_func, c_str!("entry"));
        LLVMPositionBuilderAtEnd(builder, double_block);
        let result = LLVMBuildMul(
            builder,
            LLVMGetParam(double_func, 0),
            LLVMConstInt(i32_type, 2, 0),
            c_str!(""),
        );
        LLVMBuildRet(builder, result);

        // Function to return 0 if less than 15
        let if_func_type = LLVMFunctionType(i32_type, [i32_type].as_ptr() as *mut _, 1, 0);
        let if_func = LLVMAddFunction(module, c_str!("if_func"), if_func_type);
        let if_block = LLVMAppendBasicBlockInContext(context, if_func, c_str!("entry"));
        LLVMPositionBuilderAtEnd(builder, if_block);
        let cond = LLVMBuildICmp(
            builder,
            llvm_sys::LLVMIntPredicate::LLVMIntUGT,
            LLVMGetParam(if_func, 0),
            LLVMConstInt(i32_type, 15, 0),
            c_str!("ifcond"),
        );
        // Then
        let thenbb = LLVMAppendBasicBlockInContext(context, if_func, c_str!("then"));
        // Else
        let elsebb = LLVMAppendBasicBlockInContext(context, if_func, c_str!("else"));
        // Ifcont
        let ifcontbb = LLVMAppendBasicBlockInContext(context, if_func, c_str!("ifcont"));

        LLVMBuildCondBr(builder, cond, thenbb, elsebb);

        LLVMPositionBuilderAtEnd(builder, thenbb);
        let res = LLVMBuildMul(
            builder,
            LLVMGetParam(if_func, 0),
            LLVMConstInt(i32_type, 2, 0),
            c_str!("if_res"),
        );
        LLVMBuildBr(builder, ifcontbb);

        LLVMPositionBuilderAtEnd(builder, elsebb);
        let else_res = LLVMBuildMul(
            builder,
            LLVMGetParam(if_func, 0),
            LLVMConstInt(i32_type, 3, 0),
            c_str!("else_res"),
        );
        LLVMBuildBr(builder, ifcontbb);

        LLVMPositionBuilderAtEnd(builder, ifcontbb);
        let phi = LLVMBuildPhi(builder, i32_type, c_str!("ret"));
        LLVMAddIncoming(
            phi,
            [res, else_res].as_ptr() as *mut _,
            [thenbb, elsebb].as_ptr() as *mut _,
            2,
        );
        LLVMBuildRet(builder, phi);

        // Main function
        let main_func_type = LLVMFunctionType(i32_type, ptr::null_mut(), 0, 0);
        let main_func = LLVMAddFunction(module, c_str!("main"), main_func_type);
        let main_block = LLVMAppendBasicBlockInContext(context, main_func, c_str!("entry"));
        LLVMPositionBuilderAtEnd(builder, main_block);

        // Main function's body
        let hello_world_str =
            LLVMBuildGlobalStringPtr(builder, c_str!("Hello, world!\n"), c_str!(""));
        let log_args = [hello_world_str].as_ptr() as *mut _;
        // Calling `printf("Hello, world!\n")`
        LLVMBuildCall(builder, log_func, log_args, 1, c_str!(""));

        // Double the number 15
        let thirty = LLVMBuildCall(
            builder,
            double_func,
            [LLVMConstInt(i32_type, 15, 0)].as_ptr() as *mut _,
            1,
            c_str!(""),
        );

        let result_from =
            LLVMBuildCall(builder, if_func, [thirty].as_ptr() as *mut _, 1, c_str!(""));

        // Return 0
        // LLVMBuildRet(builder, LLVMConstInt(i32_type, 0, 0));
        LLVMBuildRet(builder, result_from);

        // Print the IR
        // let ir = LLVMPrintModuleToString(module);
        // use std::ffi::CStr;
        // let cstr = CStr::from_ptr(ir);
        // println!("{}", cstr.to_str().unwrap());

        LLVMWriteBitcodeToFile(module, c_str!("main.bc"));
        LLVMDisposeBuilder(builder);

        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }
    */
}

#[cfg(test)]
mod tests {
    use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
    use logos::Logos;

    use crate::{grammar, lexer::Token, type_checker::infer_types};

    #[test]
    fn math_parse() {
        let lex = Token::lexer("1 + 3 * 2 + 79 * 3")
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::ExprParser::new().parse(lex);
        assert!(ast.is_ok());
    }

    #[test]
    fn expr_parse() {
        let lex = Token::lexer("not 123 and (ident and 123) or other_ident and true")
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::ExprParser::new().parse(lex);
        assert!(ast.is_ok());
    }

    #[test]
    fn expr_codegen() {
        let lex = Token::lexer("1 + 123 - -3 - 0 and 75")
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::ExprParser::new().parse(lex).unwrap();

        unsafe {
            use crate::codegen::Codegen;
            use llvm_sys::core::*;

            let mut compiler = crate::codegen::context::CompilerContext::new();
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithName(c_str!("main"));
            let builder = LLVMCreateBuilderInContext(context);

            let value = ast.codegen(&mut compiler, context, module, builder);
            let ir = LLVMPrintValueToString(value);
            //let ir = LLVMPrintModuleToString(module);
            use std::ffi::CStr;
            let cstr = CStr::from_ptr(ir);
            println!("{}", cstr.to_str().unwrap());

            LLVMDisposeBuilder(builder);
            LLVMDisposeModule(module);
            LLVMContextDispose(context);
        }
    }

    #[test]
    fn func_parse() {
        let lex = Token::lexer(include_str!("../examples/func.test"))
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::FunctionDefParser::new().parse(lex);
        assert!(ast.is_ok());
    }

    #[test]
    fn func_codegen() {
        let lex = Token::lexer(include_str!("../examples/func.test"))
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::FunctionDefParser::new().parse(lex).unwrap();

        unsafe {
            use llvm_sys::core::*;

            let mut compiler = crate::codegen::context::CompilerContext::new();
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithName(c_str!("main"));
            let builder = LLVMCreateBuilderInContext(context);

            ast.codegen(&mut compiler, context, module, builder);
            // let ir = LLVMPrintValueToString(value);
            let ir = LLVMPrintModuleToString(module);
            use std::ffi::CStr;
            let cstr = CStr::from_ptr(ir);
            println!("{}", cstr.to_str().unwrap());

            LLVMDisposeBuilder(builder);
            LLVMDisposeModule(module);
            LLVMContextDispose(context);
        }
    }

    #[test]
    fn extern_parse() {
        let lex = Token::lexer("extern puts(s: str) -> int;")
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::ProgramParser::new().parse(lex);
        assert!(ast.is_ok());
    }

    #[test]
    fn extern_codegen() {
        let lex = Token::lexer("extern puts(s: str) -> int;")
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::ProgramParser::new().parse(lex).unwrap();

        unsafe {
            use llvm_sys::core::*;

            let mut compiler = crate::codegen::context::CompilerContext::new();
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithName(c_str!("main"));
            let builder = LLVMCreateBuilderInContext(context);

            ast.codegen(&mut compiler, context, module, builder);
            // let ir = LLVMPrintValueToString(value);
            let ir = LLVMPrintModuleToString(module);
            use std::ffi::CStr;
            let cstr = CStr::from_ptr(ir);
            println!("{}", cstr.to_str().unwrap());

            LLVMDisposeBuilder(builder);
            LLVMDisposeModule(module);
            LLVMContextDispose(context);
        }
    }

    #[test]
    fn program_parse() {
        let lex = Token::lexer(include_str!("../examples/main.test"))
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::ProgramParser::new().parse(lex);
        assert!(ast.is_ok());
    }

    #[test]
    fn program_codegen() {
        let lex = Token::lexer(include_str!("../examples/puts.test"))
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::ProgramParser::new().parse(lex).unwrap();

        unsafe {
            use llvm_sys::core::*;

            let mut compiler = crate::codegen::context::CompilerContext::new();
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithName(c_str!("main"));
            let builder = LLVMCreateBuilderInContext(context);
            LLVMSetTarget(module, c_str!("x86_64-pc-linux-gnu"));

            ast.codegen(&mut compiler, context, module, builder);
            LLVMPrintModuleToFile(module, c_str!("main.ll"), std::ptr::null_mut());
            LLVMWriteBitcodeToFile(module, c_str!("main.bc"));

            LLVMDisposeBuilder(builder);
            LLVMDisposeModule(module);
            LLVMContextDispose(context);
        }
    }

    #[test]
    fn var_codegen() {
        let lex = Token::lexer(include_str!("../examples/println.test"))
            .spanned()
            .map(Token::to_lalr_triple);
        let ast = grammar::ProgramParser::new().parse(lex).unwrap();

        unsafe {
            use llvm_sys::core::*;

            let mut compiler = crate::codegen::context::CompilerContext::new();
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithName(c_str!("main"));
            let builder = LLVMCreateBuilderInContext(context);
            LLVMSetTarget(module, c_str!("x86_64-pc-linux-gnu"));

            let ast = infer_types(ast);
            ast.codegen(&mut compiler, context, module, builder);
            LLVMPrintModuleToFile(module, c_str!("main.ll"), std::ptr::null_mut());
            LLVMWriteBitcodeToFile(module, c_str!("main.bc"));

            LLVMDisposeBuilder(builder);
            LLVMDisposeModule(module);
            LLVMContextDispose(context);
        }
    }
}
