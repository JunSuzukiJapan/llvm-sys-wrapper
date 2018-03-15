#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use std::ffi::CString;

#[derive(Debug)]
pub struct Function {
    llvm_function: LLVMValueRef,
    llvm_module: LLVMModuleRef
}

impl Function {
    pub fn new(module: LLVMModuleRef, name: &str, function_type: LLVMTypeRef) -> Function {
        let function_name = CString::new(name).unwrap();
        let function = unsafe { LLVMAddFunction(module, function_name.as_ptr(), function_type) };
        Function {
            llvm_function: function,
            llvm_module: module
        }
    }

    pub fn from_ptr(func_ptr: LLVMValueRef) -> Function {
        Function {
            llvm_function: func_ptr,
            llvm_module: 0 as LLVMModuleRef
        }
    }

    pub fn append_basic_block(&self, name: &str) -> LLVMBasicBlockRef {
        let label_name = CString::new(name).unwrap();
        if self.llvm_module.is_null() {
            unsafe { LLVMAppendBasicBlock(self.llvm_function, label_name.as_ptr()) }
        }else{
            let context = unsafe { LLVMGetModuleContext(self.llvm_module) };
            unsafe { LLVMAppendBasicBlockInContext(context, self.llvm_function, label_name.as_ptr()) }
        }
    }

    pub fn as_ref(&self) -> LLVMValueRef {
        self.llvm_function
    }
}