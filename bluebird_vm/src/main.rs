
use core::{ops::{Instructions, Op, ReadIntoString}, vm::{SysCallHandler, VirtualMachine}};
use std::{env, fs::File, io::Read, str::FromStr};

use tree_sitter::{Node, TreeCursor};
use tree_sitter_bluebird_asm::language as bluebird_asm_language;

mod core;
mod example;

fn main() {
    example::example();
    return;
    let args = env::args();
    let file_name = args.skip(1).take(1).next().unwrap();
    
    let instructions = {
        let mut file = File::open(file_name).expect("Cannot read reader.");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        Instructions::try_from(buffer.as_slice()).unwrap()
    };

    let mut vm = VirtualMachine::new(instructions, 128, MySysCallHandler);

    while !vm.end_of_instruction() {
        vm.clock();
    }
}

struct MySysCallHandler;

impl SysCallHandler for MySysCallHandler {
    fn handle(&mut self, syscall_number: u32) -> bool {
        false
    }
}
