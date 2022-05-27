mod expr;
mod lexer;
mod parser;

use lexer::Token;

use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use logos::Logos;
use std::ptr;

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

fn main() {
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
        let double_block = LLVMAppendBasicBlockInContext(context, double_func, c_str!("double"));
        LLVMPositionBuilderAtEnd(builder, double_block);
        let result = LLVMBuildMul(
            builder,
            LLVMGetParam(double_func, 0),
            LLVMConstInt(i32_type, 2, 0),
            c_str!(""),
        );
        LLVMBuildRet(builder, result);

        // Main function
        let main_func_type = LLVMFunctionType(i32_type, ptr::null_mut(), 0, 0);
        let main_func = LLVMAddFunction(module, c_str!("main"), main_func_type);
        let main_block = LLVMAppendBasicBlockInContext(context, main_func, c_str!("main"));
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

        // Return 0
        // LLVMBuildRet(builder, LLVMConstInt(i32_type, 0, 0));
        LLVMBuildRet(builder, thirty);

        LLVMWriteBitcodeToFile(module, c_str!("main.bc"));
        LLVMDisposeBuilder(builder);

        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }
}
