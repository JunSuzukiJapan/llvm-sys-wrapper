#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;
use LLVM::Type;

#[test]
fn test_puts() {
    // initialize LLVM
    LLVM::initialize();

    // create context
    let context = Context::global_context();

    // setup our builder and module
    let builder = Builder::new_in_context(context);
    let module = Module::new_in_context("call_printf", context);

    // create main function and entry point
    let fun_type = fn_type!(Type::Void());
    let function = module.add_function("main", fun_type);
    let entry_block = function.append_basic_block("entry");
    builder.position_at_end(entry_block);

    // setup strings
    let hello = builder.build_global_string_ptr("Hello, %s\n", "hello_str");
    let world = builder.build_global_string_ptr("world!", "world_str");

    // setup printf function
    let printf_type = fn_type!(Type::Int32(), Type::CharPointer() ,,,);  // Int32 printf(CharPointer, ...)
    let printf_func = module.add_function("printf", printf_type);

    // call printf function
    let mut args = [hello, world];
    let _call = builder.build_call(printf_func.as_ref(), args.as_mut_ptr(), args.len() as u32, "call_printf");
    // ret void
    let _ret = builder.build_ret_void();

    // verify & dump
    match module.verify() {
        Ok(_) => module.dump(),
        Err(msg) => panic!("Error: {}", msg)
    }
}