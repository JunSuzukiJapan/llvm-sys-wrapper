#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;
use LLVM::{Type, Const};

#[test]
fn it_works() {    // 参考: [Go言語で利用するLLVM入門](https://postd.cc/an-introduction-to-llvm-in-go/)
    LLVM::initialize();

    // setup our builder and module
    let builder = Builder::new();
    let module = Module::new("my_module");

    // create our function prologue
    let fun_type = fn_type!(Type::Int32());
    let function = module.add_function("main", fun_type);
    let entry_block = function.append_basic_block("entry");
    builder.position_at_end(entry_block);

    // int a = 32
    let int32_typ = Type::Int32();
    let a = builder.build_alloca(int32_typ);
    let const_a_value = Const::SInt32(32);
    builder.build_store(const_a_value, a);

    // int b = 16
    let b = builder.build_alloca(int32_typ);
    let const_b_value = Const::SInt32(16);
    builder.build_store(const_b_value, b);

    // return a + b
    let a_val = builder.build_load(a);
    let b_val = builder.build_load(b);
    let ab_val = builder.build_add(a_val, b_val);
    builder.build_ret(ab_val);

    // verify & dump
    match module.verify() {
        Ok(_) => { /* module.dump() */ },
        Err(msg) => panic!("Error: {}", msg)
    }
}