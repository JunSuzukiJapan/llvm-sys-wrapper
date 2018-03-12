extern crate llvm_sys;

mod builder;
mod module;
mod function;
mod context;

pub use self::llvm_sys::core::*;
pub use self::llvm_sys::prelude::*;
pub use self::builder::Builder;
pub use self::module::Module;
pub use self::function::Function;
pub use self::context::Context;
pub use self::llvm_sys::*;

#[allow(non_snake_case)]
pub mod LLVM {
    use llvm_sys::core::*;
    use llvm_sys::target;
    use llvm_sys::prelude::*;
    use std::os::raw::c_uint;

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

    pub mod Type {
        use super::*;

        pub fn PointerType(elem_type: LLVMTypeRef, address_space: c_uint) -> LLVMTypeRef {
            unsafe { LLVMPointerType(elem_type, address_space) }
        }
        pub fn Half() -> LLVMTypeRef {
            unsafe { LLVMHalfType() }
        }
        pub fn Int1() -> LLVMTypeRef {
            unsafe { LLVMInt1Type() }
        }
        pub fn Int8() -> LLVMTypeRef {
            unsafe { LLVMInt8Type() }
        }
        pub fn Void() -> LLVMTypeRef {
            unsafe { LLVMVoidType() }
        }
        pub fn Float() -> LLVMTypeRef {
            unsafe { LLVMFloatType() }
        }
        pub fn FP128() -> LLVMTypeRef {
            unsafe { LLVMFP128Type() }
        }
        pub fn Int16() -> LLVMTypeRef {
            unsafe { LLVMInt16Type() }
        }
        pub fn Int32() -> LLVMTypeRef {
            unsafe { LLVMInt32Type() }
        }
        pub fn Int64() -> LLVMTypeRef {
            unsafe { LLVMInt64Type() }
        }
        pub fn Label() -> LLVMTypeRef {
            unsafe { LLVMLabelType() }
        }
        pub fn Double() -> LLVMTypeRef {
            unsafe { LLVMDoubleType() }
        }
        pub fn Int128() -> LLVMTypeRef {
            unsafe { LLVMInt128Type() }
        }
        pub fn X86MMX() -> LLVMTypeRef {
            unsafe { LLVMX86MMXType() }
        }
        pub fn X86FP80() -> LLVMTypeRef {
            unsafe { LLVMX86FP80Type() }
        }
        pub fn PPCFP128() -> LLVMTypeRef {
            unsafe { LLVMPPCFP128Type() }
        }
    }

    pub mod Const {

    }
}

#[macro_export]
macro_rules! function_type {
    ($result_type:expr) => (
        unsafe {
            let mut param_types = [];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 0)
        }
    );
    ($result_type:expr, ...) => (
        unsafe {
            let mut param_types = [];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 1)
        }
    );
    ($result_type:expr, $( $param_type:expr ),* ) => (
        unsafe {
            let mut param_types = [ $( $param_type ),* ];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 0)
        }
    );
    ($result_type:expr, $( $param_type:expr ),* , ...) => (
        unsafe {
            let mut param_types = [ $( $param_type ),* ];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 1)
        }
    )
}
