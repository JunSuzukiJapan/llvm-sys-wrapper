extern crate llvm_sys;
use llvm_sys::target;

mod builder;
mod module;
mod function;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct LLVM_WRAPPER;

impl LLVM_WRAPPER {
    pub fn initialize(){
        unsafe {
            if target::LLVM_InitializeNativeTarget() != 0 {
                panic!("Could not initialise target");
            }
            if target::LLVM_InitializeNativeAsmPrinter() != 0 {
                panic!("Could not initialise ASM Printer");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use llvm_sys::core::*;
    use super::*;

    #[test]
    fn it_works() {
        LLVM_WRAPPER::initialize();

        // setup our builder and module
        let builder = builder::Builder::new();
        let module = module::Module::new(builder.as_ref(), "my_module");

        // create our function prologue
        let function_type = unsafe {
            let mut param_types = [];
            LLVMFunctionType(LLVMInt32Type(), param_types.as_mut_ptr(), param_types.len() as u32, 0)
        };

        let function = function::Function::new(builder.as_ref(), module.as_ref(), "main", function_type);
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

        match module.verify() {
            Ok(_) => {
                module.dump()
            },
            Err(msg) => println!("Error: {}", msg)
        }        
    }
}
