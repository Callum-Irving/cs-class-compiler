mod ast;
mod codegen;
mod lexer;
mod type_checker;

use std::io::Read;
use std::path::PathBuf;
use std::process::{exit, Command};

use clap::Parser;

use codegen::context::CompilerContext;
use lexer::Token;
use logos::Logos;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

#[derive(Parser, Debug)]
struct CliArgs {
    src: PathBuf,
}

fn main() {
    // Parse command line args
    let args = CliArgs::parse();

    // Open source file
    let mut file = match std::fs::File::open(args.src) {
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(1);
        }
        Ok(file) => file,
    };
    let mut src = String::new();
    file.read_to_string(&mut src).unwrap();

    // Tokenize and parse file
    let tokens = Token::lexer(&src).spanned().map(Token::to_lalr_triple);
    let ast = match grammar::ProgramParser::new().parse(tokens) {
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            exit(1);
        }
        Ok(ast) => ast,
    };

    // Compile to LLVM Bitcode
    let mut compiler = CompilerContext::new();
    println!("Writing code to file");
    unsafe {
        compiler.compile_to_file(ast, "output.bc");
    }

    println!("Wrote code to files");

    // Use clang to compile LLVM Bitcode to native binary
    Command::new("clang")
        .arg("output.bc")
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("Could not start clang");

    // TODO: Wait for Clang to finish

    // Run the compiled binary
    Command::new("./a.out")
        .spawn()
        .expect("Could not execute compiled binary");
}

#[cfg(test)]
mod tests {
    use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
    use logos::Logos;

    use crate::c_str;

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
