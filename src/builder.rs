#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use std::ffi::CString;

#[derive(Debug)]
pub struct Builder {
    llvm_builder: LLVMBuilderRef
}

impl Builder {
    pub fn new() -> Builder {
        let builder = unsafe { LLVMCreateBuilder() };
        Builder { llvm_builder: builder }
    }

    pub fn as_ref(&self) -> LLVMBuilderRef {
        self.llvm_builder
    }

    pub fn position_at_end(&self, entry_block: LLVMBasicBlockRef){
        unsafe { LLVMPositionBuilderAtEnd(self.llvm_builder, entry_block); }
    }

    pub fn build_alloca(&self, typ: LLVMTypeRef, name: &str) -> LLVMValueRef {
        let var_name = CString::new(name).unwrap();
        unsafe { LLVMBuildAlloca(self.llvm_builder, typ, var_name.as_ptr()) }
    }

    pub fn build_store(&self, val: LLVMValueRef, ptr: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildStore(self.llvm_builder, val, ptr) }
    }

    pub fn build_load(&self, pointer_val: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildLoad(self.llvm_builder, pointer_val, val_name.as_ptr()) }
    }

    pub fn build_add(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildAdd(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_ret(&self, value: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildRet(self.llvm_builder, value) }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe { LLVMDisposeBuilder(self.llvm_builder) }
    }
}