#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use self::llvm_sys::analysis::{LLVMVerifyModule, LLVMVerifierFailureAction};
use std::ffi::CString;
use std::os::raw::c_char;
use function;

#[derive(Debug)]
pub struct Module {
    llvm_builder: LLVMBuilderRef,
    llvm_module: LLVMModuleRef
}

impl Module {
    pub fn new(builder: LLVMBuilderRef, name: &str) -> Module {
        let mod_name = CString::new(name).unwrap();
        let module = unsafe { LLVMModuleCreateWithName(mod_name.as_ptr()) };
        Module {
            llvm_builder: builder,
            llvm_module: module
        }
    }

    pub fn with_context(builder: LLVMBuilderRef, name: &str, context: LLVMContextRef) -> Module {
        let mod_name = CString::new(name).unwrap();
        let module = unsafe { LLVMModuleCreateWithNameInContext(mod_name.as_ptr(), context) };
        Module {
            llvm_builder: builder,
            llvm_module: module
        }
    }

    pub fn as_ref(&self) -> LLVMModuleRef {
        self.llvm_module
    }

    pub fn add_function(&self, name: &str, function_type: LLVMTypeRef) -> function::Function {
        function::Function::new(self.llvm_builder, self.llvm_module, name, function_type)
    }

    pub fn verify(&self) -> Result<(), String> {
        let mut error: *mut c_char = 0 as *mut c_char;
        let ok = unsafe {
            let buf: *mut *mut c_char = &mut error;
            LLVMVerifyModule(self.llvm_module, LLVMVerifierFailureAction::LLVMReturnStatusAction, buf)
        };
        if ok == 1 { // error
            let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
            Err(err_msg)
        }else{ // success
            Ok(())
        }
    }

    pub fn dump(&self){
        unsafe { LLVMDumpModule(self.llvm_module) }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { LLVMDisposeModule(self.llvm_module) }
    }
}