#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;

fn main() {
    // initialize LLVM
    LLVM::initialize();

    // create context, builder and module
    let ctx = Context::global_context();
    let builder = ctx.create_builder();
    let module = ctx.create_module("fib_example");

    //
    // declare printf function
    //
    let printf_type = fn_type!(ctx.Int32Type(), ctx.CharPointerType() ,,,);  // Int32 printf(CharPointer, ...)
    let printf_func = module.add_function("printf", printf_type);

    //
    // declare fib function
    //
    let fun_type = fn_type!(ctx.Int64Type(), ctx.Int64Type());    // Int32 fib(Int32)
    let fib_func = module.add_function("fib", fun_type);
    let entry_block = fib_func.append_basic_block("entry");
    builder.position_at_end(entry_block);

    let arg = fib_func.get_param(0);
    let cond = builder.build_icmp_eq(arg, ctx.UInt64(0));

    let then0 = fib_func.append_basic_block("then0");
    let else0 = fib_func.append_basic_block("else0");
    let end = fib_func.append_basic_block("end");
    builder.build_cond_br(cond, then0, else0);

    // then0
    builder.position_at_end(then0);
    builder.build_br(end);

    // else0
    builder.position_at_end(else0);

    let cond = builder.build_icmp_eq(arg, ctx.UInt64(1));

    let then1 = fib_func.append_basic_block("then1");
    let else1 = fib_func.append_basic_block("else1");
    builder.build_cond_br(cond, then1, else1);

    // then1
    builder.position_at_end(then1);
    builder.build_br(end);

    // else1
    builder.position_at_end(else1);
    let sub2 = builder.build_sub(arg, ctx.UInt64(2));
    let mut args = [sub2];
    let fib_sub2 = builder.build_tail_call(fib_func.as_ref(), &mut args);
    let sub1 = builder.build_sub(arg, ctx.UInt64(1));
    let mut args = [sub1];
    let fib_sub1 = builder.build_tail_call(fib_func.as_ref(), &mut args);

    let sum = builder.build_add(fib_sub2, fib_sub1);
    builder.build_br(end);

    // end
    builder.position_at_end(end);
    let phi = builder.build_phi(ctx.Int64Type());
    phi.add_incoming(ctx.UInt64(0), then0);
    phi.add_incoming(ctx.UInt64(1), then1);
    phi.add_incoming(sum, else1);

    // return
    builder.build_ret(phi.as_ref());

    //
    // declare main function
    //
    let fun_type = fn_type!(ctx.VoidType());
    let main_func = module.add_function("main", fun_type);
    let entry_block = main_func.append_basic_block("entry");
    builder.position_at_end(entry_block);

    // call fib(10)
    let mut args = [ctx.UInt64(10)];
    let ret = builder.build_call(fib_func.as_ref(), &mut args);

    // format string
    let fmt_d = builder.build_global_string_ptr("%lu\n");

    // call printf function
    let mut args = [fmt_d, ret];
    builder.build_call(printf_func.as_ref(), &mut args);

    // return void
    builder.build_ret_void();

    //
    // dump
    //
    match module.verify() {
        Ok(_) => module.dump(),
        Err(msg) => panic!("Error: {}", msg),
    }
}
