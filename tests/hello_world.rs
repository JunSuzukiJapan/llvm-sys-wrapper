#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;

#[test]
fn test_puts() {    // 参考: [llvm で Hello wolrd!! 〜llvm入門 その2〜](http://blog.64p.org/entry/2012/07/18/172418)
    LLVM::initialize();

    // create context
    let context = Context::global_context();

    // setup our builder and module
    let builder = Builder::new();
    let module = Module::with_context("call_puts", context);

    // create main function and entry point
    let fun_type = fn_type!(LLVM::Type::Void());
    let function = Function::new(module.as_ref(), "main", fun_type);
    let entry_block = function.append_basic_block("entry");
    builder.position_at_end(entry_block);

    let helloworld = builder.build_global_string_ptr("Hello, world!", "hello_world_str");

    let puts_type = fn_type!(LLVM::Type::Int32(), LLVM::Type::PointerType(LLVM::Type::Int8(), 0) );
    let puts_func = Function::new(module.as_ref(), "puts", puts_type);

    let mut args = [helloworld];
    let _call = builder.build_call(puts_func.as_ref(), args.as_mut_ptr(), args.len() as u32, "call_puts");

    let _ret = builder.build_ret_void();

    // verify & dump
    match module.verify() {
        Ok(_) => {
            module.dump()
        },
        Err(msg) => panic!("Error: {}", msg)
    }
}