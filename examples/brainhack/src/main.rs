//
// A brainf*ck compiler.
//
// example:
//    $ bash run.sh
//
// 参考： [LLVM APIを使ってみよう！ 〜 Brainf**kコンパイラをIRBuilderで書き直してみた 〜](http://itchyny.hatenablog.com/entry/2017/03/06/100000)

#[macro_use]
extern crate llvm_sys_wrapper;

use llvm_sys_wrapper::*;
use std::io;
use std::io::Read;

struct Compiler {
    ctx: Context,
    builder: Builder,
    module: Module
}

impl Compiler {
    fn new(module_name: &str) -> Compiler {
        let context = Context::global_context();
        let builder = context.create_builder();
        let module = context.create_module(module_name);
        Compiler {
            ctx: context,
            builder: builder,
            module: module
        }
    }

    fn create_main(&self) -> Function {
        let fun_type = fn_type!(self.ctx.VoidType());
        let main_function = self.module.add_function("main", fun_type);
        let entry_block = main_function.append_basic_block("entry");
        self.builder.position_at_end(entry_block);
        main_function
    }

    fn init_memory(&self) -> (LLVMValueRef, LLVMValueRef) {
        let data = self.builder.build_alloca(self.ctx.Int8PointerType(), "data");
        let ptr = self.builder.build_alloca(self.ctx.Int8PointerType(), "ptr");

        // setup calloc function
        let calloc_type = fn_type!(self.ctx.Int8PointerType(), self.ctx.Int64Type(), self.ctx.Int64Type());
        let calloc_func = self.module.get_or_add_function("calloc", calloc_type);

        let mut args = [self.ctx.UInt64(30000), self.ctx.UInt64(1)];
        let data_ptr = self.builder.build_call(calloc_func.as_ref(), &mut args);

        self.builder.build_store(data_ptr, data);
        self.builder.build_store(data_ptr, ptr);

        (data, ptr)
    }

    fn free_memory(&self, data: LLVMValueRef){
        // setup free function
        let free_type = fn_type!(self.ctx.VoidType(), self.ctx.Int8PointerType());
        let free_func = self.module.get_or_add_function("free", free_type);

        let mut args = [self.builder.build_load(data)];
        self.builder.build_call(free_func.as_ref(), &mut args);
    }

    fn emit_move_ptr(&self, ptr: LLVMValueRef, diff: i64){
        let mut indices = [self.ctx.SInt32(diff as u64)];
        let gep = self.builder.build_inbounds_gep(self.builder.build_load(ptr), &mut indices);
        self.builder.build_store(gep, ptr);
    }

    fn emit_add(&self, ptr: LLVMValueRef, diff: i64){
        let tmp = self.builder.build_load(ptr);
        let add = self.builder.build_add(self.builder.build_load(tmp), self.ctx.SInt8(diff as u64));
        self.builder.build_store(add, tmp);
    }

    fn emit_put(&self, ptr: LLVMValueRef){
        // setup putchar function
        let putchar_type = fn_type!(self.ctx.Int32Type(), self.ctx.Int32Type());
        let putchar_func = self.module.get_or_add_function("putchar", putchar_type);

        let val = self.builder.build_load( self.builder.build_load(ptr) );
        let ext_val = self.builder.build_sext(val, self.ctx.Int32Type());
        let mut args = [ext_val];
        let _call = self.builder.build_call(putchar_func.as_ref(), &mut args);
    }

    fn emit_ret_void(&self){
        let _ret = self.builder.build_ret_void();
    }

    fn emit_while_start(&self, func: &Function, ptr: LLVMValueRef, chars: &mut std::str::Chars){
        let cond_block = func.append_basic_block("while_cond");
        let body_block = func.append_basic_block("while_body");
        let end_block = func.append_basic_block("while_end");

        self.builder.build_br(cond_block);
        self.builder.position_at_end(cond_block);

        let load = self.builder.build_load( self.builder.build_load(ptr) );
        let cond = self.builder.build_icmp_ne(load, self.ctx.UInt8(0));
        self.builder.build_cond_br(cond, body_block, end_block);
        self.builder.position_at_end(body_block);

        self.compile(func, ptr, chars);

        self.builder.build_br(cond_block);
        self.builder.position_at_end(end_block);
    }

    fn dump(&self){
        match self.module.verify() {
            Ok(_) => self.module.dump(),
            Err(msg) => panic!("Error: {}", msg),
        }
    }

    #[allow(dead_code)]
    fn run(&self){
        match self.module.verify() {
            Ok(_) => {
                let interperter = self.module.create_interpreter().unwrap();
                let named_function = self.module.named_function("main");
                let mut params = [];
                let run_result = interperter.run_function(named_function.as_ref(), &mut params);
                let _ = run_result.to_int();
            },
            Err(msg) => panic!("Error: {}", msg)
        }
    }

    fn compile(&self, func: &Function, ptr: LLVMValueRef, chars: &mut std::str::Chars){
        while let Some(ch) = chars.next() {
            match ch {
                '>' => self.emit_move_ptr(ptr, 1),
                '<' => self.emit_move_ptr(ptr, -1),
                '+' => self.emit_add(ptr, 1),
                '-' => self.emit_add(ptr, -1),
                '.' => self.emit_put(ptr),
                '[' => self.emit_while_start(func, ptr, chars),
                ']' => return,
                _ => (),
            }
        }
    }
}

#[allow(unused_must_use)]
fn main() {
    // initialize LLVM
    LLVM::initialize();

    // create compiler
    let compiler = Compiler::new("brainhack");

    // create main function and entry point
    let main = compiler.create_main();
    // alloca memory
    let (data, ptr) = compiler.init_memory();

    // read input
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);

    // compile
    let mut chars = buffer.chars();
    compiler.compile(&main, ptr, &mut chars);

    // free memory
    compiler.free_memory(data);
    // ret
    compiler.emit_ret_void();

    // dump
    compiler.dump();
}
