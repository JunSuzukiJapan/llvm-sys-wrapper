extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use cstring_manager::CStringManager;

#[derive(Debug)]
pub struct Phi {
    llvm_phi: LLVMValueRef
}

impl Phi {
    pub fn new(builder: LLVMBuilderRef, typ: LLVMTypeRef, name: &str) -> Phi {
        let name_ptr = CStringManager::new_cstring_as_ptr(name);
        let phi = unsafe { LLVMBuildPhi(builder, typ, name_ptr) };
        Phi {
            llvm_phi: phi
        }
    }

    pub fn as_ref(&self) -> LLVMValueRef {
        self.llvm_phi
    }

    #[inline]
    pub fn add_incoming(&self, value: LLVMValueRef, block: LLVMBasicBlockRef){
        let mut values = [value];
        let mut blocks = [block];
        self.add_incomings(&mut values, &mut blocks);
    }

    #[inline]
    pub fn add_incomings(&self, values: &mut [LLVMValueRef], blocks: &mut [LLVMBasicBlockRef]){
        let count = values.len();
        if count != blocks.len() {
            panic!("values count not equal blocks count.");
        }
        unsafe {
            LLVMAddIncoming(self.llvm_phi, values.as_mut_ptr(), blocks.as_mut_ptr(), count as u32)
        }
    }

    #[inline]
    pub fn incoming_count(&self) -> u32 {
        unsafe { LLVMCountIncoming(self.llvm_phi) }
    }

    #[inline]
    pub fn get_incoming(&self, index: u32) -> (LLVMValueRef, LLVMBasicBlockRef) {
        let value = unsafe { LLVMGetIncomingValue(self.llvm_phi, index) };
        let block = unsafe { LLVMGetIncomingBlock(self.llvm_phi, index) };
        (value, block)
    }
}