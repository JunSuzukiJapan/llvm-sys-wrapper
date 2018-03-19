#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use std::os::raw::c_uint;
use std::ffi::CString;
use LLVM::Type;
use builder::Builder;
use module::Module;

#[derive(Debug)]
pub struct Context {
    llvm_context: LLVMContextRef
}

#[allow(non_snake_case)]
impl Context {
    pub fn global_context() -> Context {
        let context = unsafe { LLVMGetGlobalContext() };
        Context {
            llvm_context: context
        }
    }

    pub fn new() -> Context {
        let context = unsafe { LLVMContextCreate() };
        Context {
            llvm_context: context
        }
    }

    pub fn from_module(module: LLVMModuleRef) -> Context {
        let context = unsafe { LLVMGetModuleContext(module) };
        Context {
            llvm_context: context
        }
    }

    pub fn as_ref(&self) -> LLVMContextRef {
        self.llvm_context
    }

    pub fn create_builder(&self) -> Builder {
        Builder::new_in_context(self.as_ref())
    }

    pub fn create_module(&self, name: &str) -> Module {
        Module::new_in_context(name, self.as_ref())
    }

    //
    // get Type
    //
    pub fn StructTypeNamed(&self, name: &str) -> LLVMTypeRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMStructCreateNamed(self.llvm_context, val_name.as_ptr()) }
    }

    pub fn StructType(&self, fields: &mut [LLVMTypeRef], packed: bool) -> LLVMTypeRef {
        unsafe { LLVMStructTypeInContext(self.llvm_context, fields.as_mut_ptr(), fields.len() as u32, if packed {1}else{0}) }
    }

    pub fn SetStructBody(structType: LLVMTypeRef, fields: &mut [LLVMTypeRef], packed: bool){
        unsafe { LLVMStructSetBody(structType, fields.as_mut_ptr(), fields.len() as u32, if packed {1}else{0}) }
    }

    pub fn ConstStruct(constantValues: &mut [LLVMValueRef], packed: bool) -> LLVMValueRef {
        unsafe { LLVMConstStruct(constantValues.as_mut_ptr(), constantValues.len() as u32, if packed {1}else{0}) }
    }

    pub fn VoidType(&self) -> LLVMTypeRef {
        unsafe { LLVMVoidTypeInContext(self.llvm_context) }
    }
    pub fn IntType(&self, num_bits: c_uint) -> LLVMTypeRef {
        unsafe { LLVMIntTypeInContext(self.llvm_context, num_bits) }
    }
    pub fn Int1Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt1TypeInContext(self.llvm_context) }
    }
    pub fn Int8Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt8TypeInContext(self.llvm_context) }
    }
    pub fn Int16Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt16TypeInContext(self.llvm_context) }
    }
    pub fn Int32Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt32TypeInContext(self.llvm_context) }
    }
    pub fn Int64Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt64TypeInContext(self.llvm_context) }
    }
    pub fn Int128Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt128TypeInContext(self.llvm_context) }
    }
    pub fn HalfType(&self) -> LLVMTypeRef {
        unsafe { LLVMHalfTypeInContext(self.llvm_context) }
    }
    pub fn FloatType(&self) -> LLVMTypeRef {
        unsafe { LLVMFloatTypeInContext(self.llvm_context) }
    }
    pub fn DoubleType(&self) -> LLVMTypeRef {
        unsafe { LLVMDoubleTypeInContext(self.llvm_context) }
    }
    pub fn FP128Type(&self) -> LLVMTypeRef {
        unsafe { LLVMFP128TypeInContext(self.llvm_context) }
    }
    pub fn X86FP80Type(&self) -> LLVMTypeRef {
        unsafe { LLVMX86FP80TypeInContext(self.llvm_context) }
    }
    pub fn PPCFP128Type(&self) -> LLVMTypeRef {
        unsafe { LLVMPPCFP128TypeInContext(self.llvm_context) }
    }
    pub fn X86MMXType(&self) -> LLVMTypeRef {
        unsafe { LLVMX86MMXTypeInContext(self.llvm_context) }
    }
    pub fn LabelType(&self) -> LLVMTypeRef {
        unsafe { LLVMLabelTypeInContext(self.llvm_context) }
    }
    pub fn CharPointerType(&self) -> LLVMTypeRef {
        Type::PointerType(self.Int8Type(), 0)
    }
    pub fn Int8PointerType(&self) -> LLVMTypeRef {
        Type::PointerType(self.Int8Type(), 0)
    }

    pub fn PointerType(&self, typ: LLVMTypeRef) -> LLVMTypeRef {
        Type::PointerType(typ, 0)
    }

    //
    // define Constant util
    //
    pub fn Bitcast(&self, constant: LLVMValueRef, to_type: LLVMTypeRef) -> LLVMValueRef {
        unsafe { LLVMConstBitCast(constant, to_type) }
    }
    pub fn SInt(&self, num_bits: c_uint, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMIntTypeInContext(self.llvm_context, num_bits), val, 1) }
    }
    pub fn UInt(&self, num_bits: c_uint, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMIntTypeInContext(self.llvm_context, num_bits), val, 0) }
    }
    pub fn SInt1(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt1TypeInContext(self.llvm_context), val, 1) }
    }
    pub fn UInt1(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt1TypeInContext(self.llvm_context), val, 0) }
    }
    pub fn SInt8(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt8TypeInContext(self.llvm_context), val, 1) }
    }
    pub fn UInt8(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt8TypeInContext(self.llvm_context), val, 0) }
    }
    pub fn SInt16(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt16TypeInContext(self.llvm_context), val, 1) }
    }
    pub fn UInt16(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt16TypeInContext(self.llvm_context), val, 0) }
    }
    pub fn SInt32(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt32TypeInContext(self.llvm_context), val, 1) }
    }
    pub fn UInt32(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt32TypeInContext(self.llvm_context), val, 0) }
    }
    pub fn SInt64(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt64TypeInContext(self.llvm_context), val, 1) }
    }
    pub fn UInt64(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt64TypeInContext(self.llvm_context), val, 0) }
    }
    pub fn SInt128(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt128TypeInContext(self.llvm_context), val, 1) }
    }
    pub fn UInt128(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt128TypeInContext(self.llvm_context), val, 0) }
    }

    pub fn Half(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMHalfTypeInContext(self.llvm_context), val) }
    }
    pub fn Float(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMFloatTypeInContext(self.llvm_context), val) }
    }
    pub fn Double(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMDoubleTypeInContext(self.llvm_context), val) }
    }
    pub fn FP128(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMFP128TypeInContext(self.llvm_context), val) }
    }
    pub fn X86FP80(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMX86FP80TypeInContext(self.llvm_context), val) }
    }
    pub fn PPCFP128(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMPPCFP128TypeInContext(self.llvm_context), val) }
    }
}