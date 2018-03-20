// usage:
//  save output to file 'tailcall.ll'
//  $ llvm-as tailcall.ll
//  $ llc tailcall.bc
//  $ llvm-gcc tailcall.s -o tailcall
//  $ ./tailcall
//
// If you read 'tailcall.s', you can look that tail calling function changed to jump operation.

#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;

#[test]
fn test_tailcall() {
    // initialize LLVM
    LLVM::initialize();

    // create context
    let context = Context::global_context();

    // setup our builder and module
    let builder = context.create_builder();
    let module = context.create_module("tailcall");

    // declare printf function
    let printf_type = fn_type!(context.Int32Type(), context.CharPointerType() ,,,);  // Int32 printf(CharPointer, ...)
    let printf_func = module.get_or_add_function("printf", printf_type);

    //
    // define fastcc function
    //
    let fun_type = fn_type!(context.Int32Type(), context.Int32Type()); // Int32 fastcc(Int32)
    let fastcc_func = module.add_function("fastcc", fun_type);
    let entry_block = fastcc_func.append_basic_block("entry");
    builder.position_at_end(entry_block);

    // load param to x
    let x = builder.build_alloca(context.Int32Type());
    let param = fastcc_func.get_param(0);
    builder.build_store(param, x);
    let x_val = builder.build_load(x);

    // define blocks
    let then_block = fastcc_func.append_basic_block("then");
    let else_block = fastcc_func.append_basic_block("else");

    // x == 1 ?
    let cmp = builder.build_icmp_eq(x_val, context.UInt32(1));
    builder.build_cond_br(cmp, then_block, else_block);

    // if false
    builder.position_at_end(else_block);
    let x2 = builder.build_sub(x_val, context.UInt32(1));
    let mut args = [x2];
    let ret = builder.build_tail_call(fastcc_func.as_ref(), &mut args);
    builder.build_ret(ret);

    // if true
    builder.position_at_end(then_block);
    // ret 1
    let _ret = builder.build_ret(context.SInt32(1));

    //
    // define main function
    //
    let fun_type = fn_type!(context.VoidType());
    let main_func = module.add_function("main", fun_type);
    let entry_block = main_func.append_basic_block("entry");
    builder.position_at_end(entry_block);

    // call fastcc
    let arg = context.UInt32(100000000);
    let mut args = [arg];
    let ret = builder.build_call(fastcc_func.as_ref(), &mut args);

    // setup format string
    let fmt_d = builder.build_global_string_ptr("%d\n");

    // call printf function
    let mut args = [fmt_d, ret];
    let _call = builder.build_call(printf_func.as_ref(), &mut args);
    // ret void
    let _ret = builder.build_ret_void();

    // verify & dump
    match module.verify() {
        Ok(_) => { /* module.dump() */ },
        Err(msg) => panic!("Error: {}", msg)
    }
}