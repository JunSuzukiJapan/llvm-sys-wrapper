extern crate llvm_sys;

use std::ffi::CString;
use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;

pub struct Struct {
    struct_type: LLVMTypeRef
}

impl Struct {
    pub fn new_with_name(ctx: LLVMContextRef, name: &str) -> Struct {
        let val_name = CString::new(name).unwrap();
        let struct_ty = unsafe { LLVMStructCreateNamed(ctx, val_name.as_ptr()) };

        Struct {
            struct_type: struct_ty
        }
    }

    pub fn new(ctx: LLVMContextRef, fields: &mut [LLVMTypeRef], packed: bool) -> Struct {
        let struct_ty = unsafe { LLVMStructTypeInContext(ctx, fields.as_mut_ptr(), fields.len() as u32, if packed {1}else{0}) };

        Struct {
            struct_type: struct_ty
        }
    }

    pub fn new_const_struct(constant_values: &mut [LLVMValueRef], packed: bool) -> LLVMValueRef {
        unsafe { LLVMConstStruct(constant_values.as_mut_ptr(), constant_values.len() as u32, if packed {1}else{0}) }
    }

    pub fn set_body(&self, fields: &mut [LLVMTypeRef], packed: bool){
        unsafe { LLVMStructSetBody(self.struct_type, fields.as_mut_ptr(), fields.len() as u32, if packed {1}else{0}) }
    }
}