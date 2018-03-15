#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use self::llvm_sys::LLVMIntPredicate::*;
use self::llvm_sys::LLVMRealPredicate::*;
use std::ffi::CString;

#[derive(Debug)]
pub struct Builder {
    llvm_builder: LLVMBuilderRef
}

impl Builder {
    pub fn new() -> Builder {
        let builder = unsafe { LLVMCreateBuilder() };
        Builder {
            llvm_builder: builder
        }
    }

    pub fn new_in_context(context: LLVMContextRef) -> Builder {
        let builder = unsafe { LLVMCreateBuilderInContext(context) };
        Builder {
            llvm_builder: builder
        }
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

    pub fn build_load(&self, pointer_val: LLVMValueRef) -> LLVMValueRef {
        self.build_load_with_name(pointer_val, "")
    }

    pub fn build_load_with_name(&self, pointer_val: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildLoad(self.llvm_builder, pointer_val, val_name.as_ptr()) }
    }

    pub fn build_add(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_add_with_name(lhs, rhs, "")
    }

    pub fn build_add_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildAdd(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_sub(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_sub_with_name(lhs, rhs, "")
    }

    pub fn build_sub_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildSub(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_mul(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_mul_with_name(lhs, rhs, "")
    }

    pub fn build_mul_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildMul(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_sdiv(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_sdiv_with_name(lhs, rhs, "")
    }

    pub fn build_sdiv_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildSDiv(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_udiv(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_udiv_with_name(lhs, rhs, "")
    }

    pub fn build_udiv_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildUDiv(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_srem(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_srem_with_name(lhs, rhs, "")
    }

    pub fn build_srem_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildSRem(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_urem(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_urem_with_name(lhs, rhs, "")
    }

    pub fn build_urem_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildURem(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_ret(&self, value: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildRet(self.llvm_builder, value) }
    }

    pub fn build_ret_void(&self) -> LLVMValueRef {
        unsafe { LLVMBuildRetVoid(self.llvm_builder) }
    }

    pub fn build_and(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_add_with_name(lhs, rhs, "")
    }

    pub fn build_and_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildAnd(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_or(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_or_with_name(lhs, rhs, "")
    }

    pub fn build_or_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildOr(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_xor(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_xor_with_name(lhs, rhs, "")
    }

    pub fn build_xor_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildXor(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_neg(&self, value: LLVMValueRef) -> LLVMValueRef {
        self.build_neg_with_name(value, "")
    }

    pub fn build_neg_with_name(&self, value: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildNeg(self.llvm_builder, value, val_name.as_ptr()) }
    }

    pub fn build_fneg(&self, value: LLVMValueRef) -> LLVMValueRef {
        self.build_fneg_with_name(value, "")
    }

    pub fn build_fneg_with_name(&self, value: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFNeg(self.llvm_builder, value, val_name.as_ptr()) }
    }

    pub fn build_shl(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_shl_with_name(lhs, rhs, "")
    }

    pub fn build_shl_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildShl(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_ashr(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_ashr_with_name(lhs, rhs, "")
    }

    pub fn build_ashr_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildAShr(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_lshr(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_lshr_with_name(lhs, rhs, "")
    }

    pub fn build_lshr_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildLShr(self.llvm_builder, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_not(&self, value: LLVMValueRef) -> LLVMValueRef {
        self.build_not_with_name(value, "")
    }

    pub fn build_not_with_name(&self, value: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildNot(self.llvm_builder, value, val_name.as_ptr()) }
    }

    pub fn build_is_not_null(&self, value: LLVMValueRef) -> LLVMValueRef {
        self.build_is_not_null_with_name(value, "")
    }

    pub fn build_is_not_null_with_name(&self, value: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildIsNotNull(self.llvm_builder, value, val_name.as_ptr()) }
    }

    pub fn build_icmp_eq(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_eq_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_eq_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntEQ, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_icmp_ne(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_ne_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_ne_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntNE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_icmp_ugt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_ugt_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_ugt_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntUGT, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_icmp_uge(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_uge_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_uge_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntUGE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_icmp_ult(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_ult_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_ult_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntULT, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_icmp_ule(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_ule_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_ule_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntULE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_icmp_sgt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_sgt_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_sgt_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntSGT, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_icmp_sge(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_sge_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_sge_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntSGE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_icmp_slt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_slt_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_slt_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntSLT, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_icmp_sle(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_icmp_sle_with_name(lhs, rhs, "")
    }

    pub fn build_icmp_sle_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildICmp(self.llvm_builder, LLVMIntSLE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_predicate_false(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_predicate_false_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_predicate_false_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealPredicateFalse, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_predicate_true(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_predicate_true_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_predicate_true_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealPredicateTrue, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_ord(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_ord_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_ord_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealORD, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_oeq(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_oeq_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_oeq_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealOEQ, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_one(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_one_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_one_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealONE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_ogt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_ogt_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_ogt_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealOGT, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_oge(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_oge_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_oge_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealOGE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_olt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_olt_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_olt_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealOLT, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_ole(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_ole_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_ole_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealOLE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_uno(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_uno_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_uno_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealUNO, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_ueq(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_ueq_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_ueq_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealUEQ, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_une(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_une_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_une_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealUNE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_ugt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_ugt_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_ugt_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealUGT, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_uge(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_uge_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_uge_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealUGE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_ult(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_ult_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_ult_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealULT, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_fcmp_ule(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.build_fcmp_ule_with_name(lhs, rhs, "")
    }

    pub fn build_fcmp_ule_with_name(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildFCmp(self.llvm_builder, LLVMRealULE, lhs, rhs, val_name.as_ptr()) }
    }

    pub fn build_global_string_ptr(&self, string: &str) -> LLVMValueRef {
        self.build_global_string_ptr_with_name(string, "")
    }

    pub fn build_global_string_ptr_with_name(&self, string: &str, name: &str) -> LLVMValueRef {
        let val_str = CString::new(string).unwrap();
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildGlobalStringPtr(self.llvm_builder, val_str.as_ptr(), val_name.as_ptr()) }
    }

    pub fn build_call(&self, func: LLVMValueRef, params:  &mut [LLVMValueRef]) -> LLVMValueRef {
        self.build_call_with_name(func, params, "")
    }

    pub fn build_call_with_name(&self, func: LLVMValueRef, params:  &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildCall(self.llvm_builder, func, params.as_mut_ptr(), params.len() as u32, val_name.as_ptr()) }
    }

    pub fn build_br(&self, dest_block: LLVMBasicBlockRef) -> LLVMValueRef {
        unsafe { LLVMBuildBr(self.llvm_builder, dest_block) }
    }

    pub fn build_cond_br(&self, condition: LLVMValueRef, then_block: LLVMBasicBlockRef, else_block: LLVMBasicBlockRef) -> LLVMValueRef {
        unsafe { LLVMBuildCondBr(self.llvm_builder, condition, then_block, else_block) }
    }

    pub fn build_sext(&self, value: LLVMValueRef, dest_type: LLVMTypeRef) -> LLVMValueRef {
        self.build_sext_with_name(value, dest_type, "")
    }

    pub fn build_sext_with_name(&self, value: LLVMValueRef, dest_type: LLVMTypeRef, name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildSExt(self.llvm_builder, value, dest_type, val_name.as_ptr()) }
    }

    pub fn build_inbounds_gep(&self, target: LLVMValueRef, indices: &mut [LLVMValueRef]) -> LLVMValueRef {
        self.build_inbounds_gep_with_name(target, indices, "")
    }

    pub fn build_inbounds_gep_with_name(&self, target: LLVMValueRef, indices: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        let val_name = CString::new(name).unwrap();
        unsafe { LLVMBuildInBoundsGEP(self.llvm_builder, target, indices.as_mut_ptr(), indices.len() as u32, val_name.as_ptr()) }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe { LLVMDisposeBuilder(self.llvm_builder) }
    }
}