#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;

#[derive(Debug)]
pub struct Context {
    llvm_context: LLVMContextRef
}

impl Context {
    pub fn global_context() -> LLVMContextRef {
        unsafe { LLVMGetGlobalContext() }
    }

    pub fn new() -> LLVMContextRef {
        unsafe { LLVMContextCreate() }
    }

    pub fn from_module(module: LLVMModuleRef) -> LLVMContextRef {
        unsafe { LLVMGetModuleContext(module) }
    }

    pub fn as_ref(&self) -> LLVMContextRef {
        self.llvm_context
    }
}