#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;

#[test]
fn test_puts() {    // 参考: [llvm で Hello wolrd!! 〜llvm入門 その2〜](http://blog.64p.org/entry/2012/07/18/172418)
    LLVM::initialize();

    // create context
    let context = Context::global_context();

    // setup our builder and module
    let builder = context.create_builder();
    let module = context.create_module("call_puts");

    // create main function and entry point
    let fun_type = fn_type!(context.VoidType());
    let function = module.add_function("main", fun_type);
    let entry_block = function.append_basic_block("entry");
    builder.position_at_end(entry_block);

    let helloworld = builder.build_global_string_ptr("Hello, world!");

    let puts_type = fn_type!(context.Int32Type(), context.CharPointerType());
    let puts_func = module.add_function("puts", puts_type);

    let mut args = [helloworld];
    let _call = builder.build_call(puts_func.as_ref(), &mut args);

    let _ret = builder.build_ret_void();

    // verify & dump
    match module.verify() {
        Ok(_) => module.dump(),
        Err(msg) => panic!("Error: {}", msg)
    }
}