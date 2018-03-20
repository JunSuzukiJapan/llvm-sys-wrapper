#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;

#[test]
fn test_array_alloca() {
    LLVM::initialize();

    // create context
    let context = Context::global_context();

    // setup our builder and module
    let builder = context.create_builder();
    let module = context.create_module("array_alloca");

    // create main function and entry point
    let fun_type = fn_type!(context.VoidType());
    let function = module.add_function("main", fun_type);
    let entry_block = function.append_basic_block("entry");
    builder.position_at_end(entry_block);

    // setup printf function
    let printf_type = fn_type!(context.Int32Type(), context.CharPointerType() ,,,);  // Int32 printf(CharPointer, ...)
    let printf_func = module.add_function("printf", printf_type);

    // alloca
    let buf = builder.build_array_alloca(context.Int8Type(), context.SInt32(7));
    let ptr = builder.build_alloca(context.Int8PointerType());
    let mut args = [context.SInt32(0)];
    builder.build_store(builder.build_inbounds_gep(buf, &mut args), ptr);

    // setup buf
    let tmp = builder.build_load(ptr);
    builder.build_store(context.UInt8('H' as u64), tmp);

    let mut args = [context.SInt32(1)];
    builder.build_store(builder.build_inbounds_gep(builder.build_load(ptr), &mut args), ptr);
    let tmp = builder.build_load(ptr);
    builder.build_store(context.UInt8('e' as u64), tmp);

     let mut args = [context.SInt32(1)];
    builder.build_store(builder.build_inbounds_gep(builder.build_load(ptr), &mut args), ptr);
    let tmp = builder.build_load(ptr);
    builder.build_store(context.UInt8('l' as u64), tmp);

     let mut args = [context.SInt32(1)];
    builder.build_store(builder.build_inbounds_gep(builder.build_load(ptr), &mut args), ptr);
    let tmp = builder.build_load(ptr);
    builder.build_store(context.UInt8('l' as u64), tmp);

     let mut args = [context.SInt32(1)];
    builder.build_store(builder.build_inbounds_gep(builder.build_load(ptr), &mut args), ptr);
    let tmp = builder.build_load(ptr);
    builder.build_store(context.UInt8('o' as u64), tmp);

     let mut args = [context.SInt32(1)];
    builder.build_store(builder.build_inbounds_gep(builder.build_load(ptr), &mut args), ptr);
    let tmp = builder.build_load(ptr);
    builder.build_store(context.UInt8('\n' as u64), tmp);

     let mut args = [context.SInt32(1)];
    builder.build_store(builder.build_inbounds_gep(builder.build_load(ptr), &mut args), ptr);
    let tmp = builder.build_load(ptr);
    builder.build_store(context.UInt8('\0' as u64), tmp);

    // call printf function
    let mut args = [buf];
    let _call = builder.build_call(printf_func.as_ref(), &mut args);

    // ret
    let _ret = builder.build_ret_void();

    // verify & dump
    match module.verify() {
        Ok(_) => { /* module.dump() */ },
        Err(msg) => panic!("Error: {}", msg)
    }
}