#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;

#[test]
fn it_works() {    // 参考: [Go言語で利用するLLVM入門](https://postd.cc/an-introduction-to-llvm-in-go/)
    LLVM::initialize();

    // setup our builder and module
    let builder = Builder::new();
    let module = Module::new(builder.as_ref(), "my_module");

    // create our function prologue
    let fun_type = function_type!(LLVM::Type::Int32());
    let function = Function::new(builder.as_ref(), module.as_ref(), "main", fun_type);
    let entry_block = function.append_basic_block("entry");
    builder.position_at_end(entry_block);

    // int a = 32
    let typ = unsafe { LLVMInt32Type() };
    let a = builder.build_alloca(typ, "a");
    let const_a_value = unsafe { LLVMConstInt(LLVMInt32Type(), 32, 0) };
    builder.build_store(const_a_value, a);

    // int b = 16
    let b = builder.build_alloca(typ, "b");
    let const_b_value = unsafe { LLVMConstInt(LLVMInt32Type(), 16, 0) };
    builder.build_store(const_b_value, b);

    // return a + b
    let a_val = builder.build_load(a, "a_val");
    let b_val = builder.build_load(b, "b_val");
    let ab_val = builder.build_add(a_val, b_val, "ab_val");
    builder.build_ret(ab_val);

    // verify & dump
    match module.verify() {
        Ok(_) => {
            module.dump()
        },
        Err(msg) => panic!("Error: {}", msg)
    }
}