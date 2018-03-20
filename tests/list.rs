#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;

#[test]
fn test_list() {
    // initialize LLVM
    LLVM::initialize();

    // create context
    let context = Context::global_context();

    // setup our builder and module
    let builder = context.create_builder();
    let module = context.create_module("call_printf");

    // create main function and entry point
    let fun_type = fn_type!(context.VoidType());
    let function = module.add_function("main", fun_type);
    let entry_block = function.append_basic_block("entry");
    builder.position_at_end(entry_block);

    // define struct Pair<int, &Pair>
    let pair_type = context.StructTypeNamed("Pair");
    let pair_pointer_type = context.PointerType(pair_type.as_ref());
    // define struct body
    let mut field_types = [context.Int32Type(), pair_pointer_type];
    pair_type.set_body(&mut field_types, false);

    // setup strings
    let fmt_num = builder.build_global_string_ptr("%d");
    let fmt_str = builder.build_global_string_ptr("%s");
    let str_lparen = builder.build_global_string_ptr("(");
    let str_rparen = builder.build_global_string_ptr(")\n");

    // declare printf function
    let printf_type = fn_type!(context.Int32Type(), context.CharPointerType() ,,,);  // Int32 printf(CharPointer, ...)
    let printf_func = module.add_function("printf", printf_type);

    //
    // make list (0 1 2 3 4)
    //

    // alloca array of Pair
    let buf = builder.build_array_alloca(pair_type.as_ref(), context.SInt32(5));
    let ptr = builder.build_alloca(context.PointerType(pair_type.as_ref()));
    let next_ptr = builder.build_alloca(context.PointerType(pair_type.as_ref()));
    let i32_ptr = builder.build_alloca(context.PointerType(context.Int32Type()));
    let null_pointer = builder.build_int_to_ptr(context.UInt32(0), pair_pointer_type);

    // setup buf
    for i in 0..5 {
        let index = context.UInt32(i as u64);
        // get pointer to buf[i]
        let mut args = [index];
        builder.build_store(builder.build_inbounds_gep(buf, &mut args), ptr);

        // // ptr.value = i
        builder.build_store(builder.build_struct_gep(builder.build_load(ptr), 0), i32_ptr);
        builder.build_store(context.SInt32(i as u64), builder.build_load(i32_ptr));

        // // ptr.next = next pointer
        let tmp = builder.build_struct_gep(builder.build_load(ptr), 1);
        if i == 4 {
            builder.build_store(null_pointer, tmp);
        }else{
            let mut args = [context.UInt32(i + 1 as u64)];
            builder.build_store(builder.build_inbounds_gep(buf, &mut args), next_ptr);
            builder.build_store(builder.build_load(next_ptr), tmp);
        }
    }

    //
    // Display
    //

    // display '('
    let mut args = [str_lparen];
    builder.build_call(printf_func.as_ref(), &mut args);

    // display values

    // get pointer to buf[i]
    let mut args = [context.UInt32(0)];
    builder.build_store(builder.build_inbounds_gep(buf, &mut args), ptr);

    // define loop block
    let loop_block = function.append_basic_block("loop");
    let end_block = function.append_basic_block("loop_end");
    builder.build_br(loop_block);
    builder.position_at_end(loop_block);

    // loop start
    builder.build_store(builder.build_struct_gep(builder.build_load(ptr), 0), i32_ptr);
    let val = builder.build_load(builder.build_load(i32_ptr));
    let mut args = [fmt_num, val];
    builder.build_call(printf_func.as_ref(), &mut args);

    // get ptr.next
    let next = builder.build_struct_gep(builder.build_load(ptr), 1);
    builder.build_store(builder.build_load(next), ptr);
    let cond = builder.build_icmp_eq(null_pointer, builder.build_load(ptr));

    builder.build_cond_br(cond, loop_block, end_block);

    // loop end
    builder.position_at_end(end_block);

    // display ')'
    let mut args = [str_rparen];
    builder.build_call(printf_func.as_ref(), &mut args);


    // ret void
    let _ret = builder.build_ret_void();

    // verify & dump
    match module.verify() {
        Ok(_) => module.dump(),
        Err(msg) => panic!("Error: {}", msg)
    }
}