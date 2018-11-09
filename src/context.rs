#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use std::os::raw::c_uint;
use LLVM::Type;
use builder::Builder;
use module::Module;
use struct_type::Struct;

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
    #[inline]
    pub fn StructTypeNamed(&self, name: &str) -> Struct {
        Struct::new_with_name(self.llvm_context, name)
    }

    #[inline]
    pub fn StructType(&self, fields: &mut [LLVMTypeRef], packed: bool) -> Struct {
        Struct::new(self.llvm_context, fields, packed)
    }

    #[inline]
    pub fn VoidType(&self) -> LLVMTypeRef {
        unsafe { LLVMVoidTypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn IntType(&self, num_bits: c_uint) -> LLVMTypeRef {
        unsafe { LLVMIntTypeInContext(self.llvm_context, num_bits) }
    }
    #[inline]
    pub fn Int1Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt1TypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn Int8Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt8TypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn Int16Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt16TypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn Int32Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt32TypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn Int64Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt64TypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn Int128Type(&self) -> LLVMTypeRef {
        unsafe { LLVMInt128TypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn HalfType(&self) -> LLVMTypeRef {
        unsafe { LLVMHalfTypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn FloatType(&self) -> LLVMTypeRef {
        unsafe { LLVMFloatTypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn DoubleType(&self) -> LLVMTypeRef {
        unsafe { LLVMDoubleTypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn FP128Type(&self) -> LLVMTypeRef {
        unsafe { LLVMFP128TypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn X86FP80Type(&self) -> LLVMTypeRef {
        unsafe { LLVMX86FP80TypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn PPCFP128Type(&self) -> LLVMTypeRef {
        unsafe { LLVMPPCFP128TypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn X86MMXType(&self) -> LLVMTypeRef {
        unsafe { LLVMX86MMXTypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn LabelType(&self) -> LLVMTypeRef {
        unsafe { LLVMLabelTypeInContext(self.llvm_context) }
    }
    #[inline]
    pub fn CharPointerType(&self) -> LLVMTypeRef {
        Type::PointerType(self.Int8Type(), 0)
    }
    #[inline]
    pub fn Int8PointerType(&self) -> LLVMTypeRef {
        Type::PointerType(self.Int8Type(), 0)
    }

    #[inline]
    pub fn PointerType(&self, typ: LLVMTypeRef) -> LLVMTypeRef {
        Type::PointerType(typ, 0)
    }

    //
    // define Constant util
    //
    #[inline]
    pub fn Null(&self, typ: LLVMTypeRef) -> LLVMValueRef {
        unsafe { LLVMConstNull(typ) }
    }

    #[inline]
    pub fn PointerNull(&self, typ: LLVMTypeRef) -> LLVMValueRef {
        unsafe { LLVMConstPointerNull(typ) }
    }

    #[inline]
    pub fn Bitcast(&self, constant: LLVMValueRef, to_type: LLVMTypeRef) -> LLVMValueRef {
        unsafe { LLVMConstBitCast(constant, to_type) }
    }
    #[inline]
    pub fn SInt(&self, num_bits: c_uint, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMIntTypeInContext(self.llvm_context, num_bits), val, 1) }
    }
    #[inline]
    pub fn UInt(&self, num_bits: c_uint, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMIntTypeInContext(self.llvm_context, num_bits), val, 0) }
    }
    #[inline]
    pub fn SInt1(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt1TypeInContext(self.llvm_context), val, 1) }
    }
    #[inline]
    pub fn UInt1(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt1TypeInContext(self.llvm_context), val, 0) }
    }
    #[inline]
    pub fn SInt8(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt8TypeInContext(self.llvm_context), val, 1) }
    }
    #[inline]
    pub fn UInt8(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt8TypeInContext(self.llvm_context), val, 0) }
    }
    #[inline]
    pub fn SInt16(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt16TypeInContext(self.llvm_context), val, 1) }
    }
    #[inline]
    pub fn UInt16(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt16TypeInContext(self.llvm_context), val, 0) }
    }
    #[inline]
    pub fn SInt32(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt32TypeInContext(self.llvm_context), val, 1) }
    }
    #[inline]
    pub fn UInt32(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt32TypeInContext(self.llvm_context), val, 0) }
    }
    #[inline]
    pub fn SInt64(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt64TypeInContext(self.llvm_context), val, 1) }
    }
    #[inline]
    pub fn UInt64(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt64TypeInContext(self.llvm_context), val, 0) }
    }
    #[inline]
    pub fn SInt128(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt128TypeInContext(self.llvm_context), val, 1) }
    }
    #[inline]
    pub fn UInt128(&self, val: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt128TypeInContext(self.llvm_context), val, 0) }
    }

    #[inline]
    pub fn Half(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMHalfTypeInContext(self.llvm_context), val) }
    }
    #[inline]
    pub fn Float(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMFloatTypeInContext(self.llvm_context), val) }
    }
    #[inline]
    pub fn Double(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMDoubleTypeInContext(self.llvm_context), val) }
    }
    #[inline]
    pub fn FP128(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMFP128TypeInContext(self.llvm_context), val) }
    }
    #[inline]
    pub fn X86FP80(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMX86FP80TypeInContext(self.llvm_context), val) }
    }
    #[inline]
    pub fn PPCFP128(&self, val: f64) -> LLVMValueRef {
        unsafe { LLVMConstReal(LLVMPPCFP128TypeInContext(self.llvm_context), val) }
    }
}