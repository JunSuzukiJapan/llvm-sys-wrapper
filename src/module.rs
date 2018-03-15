#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use self::llvm_sys::analysis::{LLVMVerifyModule, LLVMVerifierFailureAction};
use std::ffi::CString;
use std::os::raw::c_char;
use function;
use engine::Engine;

#[derive(Debug)]
pub struct Module {
    llvm_module: LLVMModuleRef
}

impl Module {
    pub fn new(name: &str) -> Module {
        let mod_name = CString::new(name).unwrap();
        let module = unsafe { LLVMModuleCreateWithName(mod_name.as_ptr()) };
        Module {
            llvm_module: module
        }
    }

    pub fn new_in_context(name: &str, context: LLVMContextRef) -> Module {
        let mod_name = CString::new(name).unwrap();
        let module = unsafe { LLVMModuleCreateWithNameInContext(mod_name.as_ptr(), context) };
        Module {
            llvm_module: module
        }
    }

    pub fn as_ref(&self) -> LLVMModuleRef {
        self.llvm_module
    }

    pub fn add_function(&self, name: &str, function_type: LLVMTypeRef) -> function::Function {
        function::Function::new(self.llvm_module, name, function_type)
    }

    pub fn named_function(&self, name: &str) -> function::Function {
        let func_name = CString::new(name).unwrap();
        let named_function = unsafe { LLVMGetNamedFunction(self.llvm_module, func_name.as_ptr()) };
        function::Function::from_ptr(named_function)
    }

    pub fn get_or_add_function(&self, name: &str, function_type: LLVMTypeRef) -> function::Function {
        let func_name = CString::new(name).unwrap();
        let named_function = unsafe { LLVMGetNamedFunction(self.llvm_module, func_name.as_ptr()) };
        if named_function == (0 as LLVMValueRef) {
            function::Function::new(self.llvm_module, name, function_type)
        }else{
            function::Function::from_ptr(named_function)
        }
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

    pub fn create_interpreter(&self) -> Result<Engine, String> {
        Engine::create_interpreter(self.as_ref())
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { LLVMDisposeModule(self.llvm_module) }
    }
}