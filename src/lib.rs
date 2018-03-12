extern crate llvm_sys;

mod builder;
mod module;
mod function;

#[allow(unused_imports)]
use builder::Builder;
#[allow(unused_imports)]
use module::Module;
#[allow(unused_imports)]
use function::Function;

#[allow(non_snake_case)]
pub mod LLVM {
    use llvm_sys::core::*;
    use llvm_sys::target;
    use llvm_sys::prelude::*;

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

    pub mod types {
        use super::*;

        pub fn Int32() -> LLVMTypeRef {
            unsafe { LLVMInt32Type() }
        }
    }
}

#[cfg(test)]
mod tests {
    use llvm_sys::core::*;
    use super::*;

    #[test]
    fn it_works() {
        LLVM::initialize();

        // setup our builder and module
        let builder = Builder::new();
        let module = Module::new(builder.as_ref(), "my_module");

        // create our function prologue
        let function_type = unsafe {
            let mut param_types = [];
            LLVMFunctionType(LLVM::types::Int32(), param_types.as_mut_ptr(), param_types.len() as u32, 0)
        };

        let function = Function::new(builder.as_ref(), module.as_ref(), "main", function_type);
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
