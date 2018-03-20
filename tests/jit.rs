#[macro_use]
extern crate llvm_sys_wrapper;
extern crate llvm_sys;

use llvm_sys_wrapper::*;

#[test]
fn test_jit_engine() {
    // initialize LLVM
    LLVM::initialize();

    // create context
    let context = Context::global_context();

    // setup our builder and module
    let builder = context.create_builder();
    let module = context.create_module("jit_engine");

    // create main function and entry point
    let fun_type = fn_type!(context.VoidType());
    let function = module.add_function("main", fun_type);
    let entry_block = function.append_basic_block("entry");
    builder.position_at_end(entry_block);

    // setup strings
    let hello = builder.build_global_string_ptr("Hello, %s\n");
    let world = builder.build_global_string_ptr("JIT Engine!");

    // setup printf function
    let printf_type = fn_type!(context.Int32Type(), context.CharPointerType() ,,,);  // Int32 printf(CharPointer, ...)
    let printf_func = module.add_function("printf", printf_type);

    // call printf function
    let mut args = [hello, world];
    let _call = builder.build_call(printf_func.as_ref(), &mut args);
    // ret void
    let _ret = builder.build_ret_void();

    // verify & dump
    match module.verify() {
        Ok(_) => {
            //module.dump();

            let interperter = module.create_jit_engine().unwrap();
            let named_function = module.named_function("main");
            let mut params = [];
            let run_result = interperter.run_function(named_function.as_ref(), &mut params);
            let _ = run_result.to_int();
        },
        Err(msg) => panic!("Error: {}", msg)
    }
}