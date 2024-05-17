
use core::{ops::{Instructions, Op, ReadIntoString}, vm::VirtualMachine};
use std::{env, fs::File, io::Read, str::FromStr};

use tree_sitter::{Node, TreeCursor};
use tree_sitter_bluebird_asm::language as bluebird_asm_language;

mod core;


fn main() {
    let args = env::args();
    let file_name = args.skip(1).take(1).next().unwrap();
    
    let instructions = {
        let mut file = File::open(file_name).expect("Cannot read reader.");
        Instructions::try_from(file.read_into_string().as_bytes()).unwrap()
    };

    let mut vm = VirtualMachine::new(instructions, 128);

    while !vm.end_of_instruction() {
        vm.clock();
    }
}
