use std::env;
use std::process;

use silicon;
use silicon::Config;
use std::path::PathBuf;
use crate::runtime::{VM, CallFrame};
use crate::opcode::OpCode::*;
use crate::instance::{Instance, Instance::*, Function};
use std::intrinsics::transmute;
use crate::opcode::Chunk;
use std::rc::Rc;
use std::collections::HashSet;
use crate::string_pool::StringPool;

mod runtime;
mod opcode;
mod instance;
mod string_pool;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vm = VM::new();

    let mut chunk = Chunk::new();
    chunk.set_register_size(1);
    let pooled_string = vm.string_pool.pool_string("Hello, world!".to_string());
    let pooled_string_two = vm.string_pool.pool_string("How are you?".to_string());
    chunk.add_const(0, Str(pooled_string));
    chunk.add_const(1, Char(' '));
    chunk.add_const(2, Str(pooled_string_two));
    chunk.add_const(3, Byte(25));
    chunk.write(Get(true, 0));
    chunk.write(Get(true, 1));
    chunk.write(Concat);
    chunk.write(Get(true, 2));
    chunk.write(Concat);
    chunk.write(Set(0));
    chunk.write(Get(false, 0));
    chunk.write(Print);
    chunk.write(Get(false, 0));
    chunk.write(Get(true, 3));
    chunk.write(IndexGet);
    chunk.write(Print);

    vm.execute_chunk(Rc::new(chunk), Rc::new(CallFrame::new()));

    /*
    if args.len() >= 2 {
        //engine::run(PathBuf::from(&args[1]))
    }
    else {
        println!("Please specify the path to the main script!");
        process::exit(1);
    }

    println!("Program execution finished successfully.");
    */
}
