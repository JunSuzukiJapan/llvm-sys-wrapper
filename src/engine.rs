extern crate llvm_sys;
extern crate libc;

use self::llvm_sys::prelude::*;
use self::llvm_sys::execution_engine::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_ulonglong, c_uint};
use self::libc::c_void;
use LLVM::Type;

#[derive(Debug)]
pub struct Engine {
    llvm_execute_engine: LLVMExecutionEngineRef
}

impl Engine {
    pub fn create_interpreter(module: LLVMModuleRef) -> Result<Engine, String> {
        let mut error: *mut c_char = 0 as *mut c_char;
        let mut engine: LLVMExecutionEngineRef = 0 as LLVMExecutionEngineRef;
        let result = unsafe {
            let buf: *mut *mut c_char = &mut error;
            let engine_ref: *mut LLVMExecutionEngineRef = &mut engine;
            LLVMLinkInInterpreter();
            LLVMCreateInterpreterForModule(engine_ref, module, buf)
        };        

        if result == 1 { // error
            let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
            Err(err_msg)

        }else{           // ok
            Ok(Engine {
                llvm_execute_engine: engine
            })
        }
    }

    pub fn create_jit_engine(module: LLVMModuleRef) -> Result<Engine, String> {
        let mut error: *mut c_char = 0 as *mut c_char;
        let mut engine: LLVMExecutionEngineRef = 0 as LLVMExecutionEngineRef;
        let result = unsafe {
            let buf: *mut *mut c_char = &mut error;
            let engine_ref: *mut LLVMExecutionEngineRef = &mut engine;
            LLVMLinkInMCJIT();
            LLVMCreateMCJITCompilerForModule(engine_ref, module, 0 as *mut LLVMMCJITCompilerOptions, 0, buf)
        };        

        if result == 1 { // error
            let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
            Err(err_msg)

        }else{           // ok
            Ok(Engine {
                llvm_execute_engine: engine
            })
        }
    }

    pub fn as_ref(&self) -> LLVMExecutionEngineRef {
        self.llvm_execute_engine
    }

    pub fn run_function(&self, function: LLVMValueRef, args: &mut [LLVMGenericValueRef]) -> FuncallResult {
        let func_result = unsafe { LLVMRunFunction(self.llvm_execute_engine, function, args.len() as u32, args.as_mut_ptr()) };
        FuncallResult::new(func_result)
    }
}

pub struct FuncallResult {
    value: LLVMGenericValueRef
}

impl FuncallResult {
    pub fn new(val: LLVMGenericValueRef) -> FuncallResult {
        FuncallResult {
            value: val
        }
    }

    pub fn to_int(&self) -> c_ulonglong {
        unsafe { LLVMGenericValueToInt(self.value, 0) }
    }

    pub fn int_width(&self) -> c_uint {
        unsafe { LLVMGenericValueIntWidth(self.value) }
    }

    pub fn to_ptr(&self) -> *mut c_void {
        unsafe { LLVMGenericValueToPointer(self.value) }
    }

    pub fn to_float(&self) -> f32 {
        unsafe { LLVMGenericValueToFloat(Type::Float(), self.value) as f32 }
    }

    pub fn to_double(&self) -> f64 {
        unsafe { LLVMGenericValueToFloat(Type::Double(), self.value) }
    }
}