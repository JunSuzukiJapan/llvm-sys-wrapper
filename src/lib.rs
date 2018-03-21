extern crate llvm_sys;

mod builder;
mod module;
mod function;
mod context;
mod engine;
mod phi;
mod struct_type;
mod cstring_manager;

pub use self::llvm_sys::core::*;
pub use self::llvm_sys::prelude::*;
pub use self::builder::Builder;
pub use self::module::Module;
pub use self::function::Function;
pub use self::context::Context;
pub use self::phi::Phi;
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
        pub fn Void() -> LLVMTypeRef {
            unsafe { LLVMVoidType() }
        }
        pub fn Int(num_bits: c_uint) -> LLVMTypeRef {
            unsafe { LLVMIntType(num_bits) }
        }
        pub fn Int1() -> LLVMTypeRef {
            unsafe { LLVMInt1Type() }
        }
        pub fn Int8() -> LLVMTypeRef {
            unsafe { LLVMInt8Type() }
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
        pub fn Int128() -> LLVMTypeRef {
            unsafe { LLVMInt128Type() }
        }
        pub fn Half() -> LLVMTypeRef {
            unsafe { LLVMHalfType() }
        }
        pub fn Float() -> LLVMTypeRef {
            unsafe { LLVMFloatType() }
        }
        pub fn Double() -> LLVMTypeRef {
            unsafe { LLVMDoubleType() }
        }
        pub fn FP128() -> LLVMTypeRef {
            unsafe { LLVMFP128Type() }
        }
        pub fn X86FP80() -> LLVMTypeRef {
            unsafe { LLVMX86FP80Type() }
        }
        pub fn PPCFP128() -> LLVMTypeRef {
            unsafe { LLVMPPCFP128Type() }
        }
        pub fn X86MMX() -> LLVMTypeRef {
            unsafe { LLVMX86MMXType() }
        }
        pub fn Label() -> LLVMTypeRef {
            unsafe { LLVMLabelType() }
        }
        pub fn CharPointer() -> LLVMTypeRef {
            Type::PointerType(Type::Int8(), 0)
        }
        pub fn Int8Pointer() -> LLVMTypeRef {
            Type::PointerType(Type::Int8(), 0)
        }
    }

    pub mod Const {
        use super::*;

        pub fn SInt(num_bits: c_uint, val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMIntType(num_bits), val, 1) }
        }
        pub fn UInt(num_bits: c_uint, val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMIntType(num_bits), val, 0) }
        }
        pub fn SInt1(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt1Type(), val, 1) }
        }
        pub fn UInt1(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt1Type(), val, 0) }
        }
        pub fn SInt8(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt8Type(), val, 1) }
        }
        pub fn UInt8(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt8Type(), val, 0) }
        }
        pub fn SInt16(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt16Type(), val, 1) }
        }
        pub fn UInt16(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt16Type(), val, 0) }
        }
        pub fn SInt32(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt32Type(), val, 1) }
        }
        pub fn UInt32(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt32Type(), val, 0) }
        }
        pub fn SInt64(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt64Type(), val, 1) }
        }
        pub fn UInt64(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt64Type(), val, 0) }
        }
        pub fn SInt128(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt128Type(), val, 1) }
        }
        pub fn UInt128(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt128Type(), val, 0) }
        }

        pub fn Half(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMHalfType(), val) }
        }
        pub fn Float(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMFloatType(), val) }
        }
        pub fn Double(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMDoubleType(), val) }
        }
        pub fn FP128(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMFP128Type(), val) }
        }
        pub fn X86FP80(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMX86FP80Type(), val) }
        }
        pub fn PPCFP128(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMPPCFP128Type(), val) }
        }
    }
}

#[macro_export]
macro_rules! fn_type {
    ($result_type:expr) => (
        unsafe {
            let mut param_types = [];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 0)
        }
    );
    ($result_type:expr,,,) => (
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
    ($result_type:expr, $( $param_type:expr ),* ,,,) => (
        unsafe {
            let mut param_types = [ $( $param_type ),* ];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 1)
        }
    )
}
