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

    //
    // make list (0 1 2 3 4)
    //

    // alloca array of Pair
    let buf = builder.build_array_alloca(pair_type.as_ref(), context.SInt32(5));
    let ptr = builder.build_alloca(context.PointerType(pair_type.as_ref()));
    let next_ptr = builder.build_alloca(context.PointerType(pair_type.as_ref()));
    let i32_ptr = builder.build_alloca(context.PointerType(context.Int32Type()));
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
            let null_pointer = builder.build_int_to_ptr(context.UInt32(0), pair_pointer_type);
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

    // setup strings
    let fmt_num = builder.build_global_string_ptr("%d\n");
    let fmt_str = builder.build_global_string_ptr("%s\n");
    let str_lparen = builder.build_global_string_ptr("(");
    let str_rparen = builder.build_global_string_ptr(")");

    // declare printf function
    let printf_type = fn_type!(context.Int32Type(), context.CharPointerType() ,,,);  // Int32 printf(CharPointer, ...)
    let printf_func = module.add_function("printf", printf_type);

    // display '('
    let mut args = [str_lparen];
    let _call = builder.build_call(printf_func.as_ref(), &mut args);

    // display ')'
    let mut args = [str_rparen];
    let _call = builder.build_call(printf_func.as_ref(), &mut args);


    // ret void
    let _ret = builder.build_ret_void();

    // verify & dump
    match module.verify() {
        Ok(_) => module.dump(),
        Err(msg) => panic!("Error: {}", msg)
    }
}