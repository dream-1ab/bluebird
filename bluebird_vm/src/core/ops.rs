use std::{collections::HashMap, fmt::format, io::Read, iter::Map, str::FromStr};

use tree_sitter::{Node, Tree};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-05-13 01:55:42
 * @modify date 2024-05-13 01:55:42
 * @desc [description]
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackMemoryAccess {
    Absolute(u32),
    Relative(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterName {
    Pc,
    Sp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpValue {
    Memory(StackMemoryAccess),
    Number(u32),
    Register(RegisterName),
    Label(u32),
}

impl<'tree> From<(Node<'tree>, &'tree [u8])> for OpValue {
    fn from(mut value: (Node<'tree>, &'tree [u8])) -> Self {
        value = (value.0.child(0).unwrap(), value.1);
        let kind = value.0.kind();
        if kind == "decimal_number" {
            return Self::Number(decimal_number_node_to_u32(value.0, value.1));
        }
        if kind == "absolute_memory_address" {
            let text = value.0.utf8_text(value.1).unwrap();
            let text = &text[1..text.len() - 1];
            return Self::Memory(StackMemoryAccess::Absolute(text.parse().unwrap()));
        }
        if kind == "relative_memory_address" {
            let text = value.0.utf8_text(value.1).unwrap();
            let text = &text[2..text.len() - 1];
            return Self::Memory(StackMemoryAccess::Relative(text.parse().unwrap()));
        }
        if kind == "label" {
            let source = value.1;
            let node = value.0;
            return OpValue::Label(get_index_of_label(node, &source));
        }
        if kind == "register" {
            let register_name = value.0.utf8_text(value.1).unwrap();
            return OpValue::Register(match register_name {
                "PC" => RegisterName::Pc,
                "SP" => RegisterName::Sp,
                _ => panic!("unknown register {}", register_name)
            });
        }
        panic!("unknown op value: {}", kind);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Label(u32),
    Add(OpValue, OpValue, OpValue),
    Subtract(OpValue, OpValue, OpValue),
    Multiplication(OpValue, OpValue, OpValue),
    Division(OpValue, OpValue, OpValue),
    Modulo(OpValue, OpValue, OpValue),
    And(OpValue, OpValue, OpValue),
    Or(OpValue, OpValue, OpValue),
    Not(OpValue, OpValue),
    Jump(OpValue),
    Compare(OpValue, OpValue),
    BranchEquals(u32),
    BranchNotEquals(u32),
    BranchGreaterThan(u32),
    BranchLessThan(u32),
    Push(u32),
    Pop(u32),
    Store(OpValue, OpValue),
    Call(OpValue),
    Ret,
    SysCall(OpValue),
}

fn decimal_number_node_to_u32(node: Node, source: &[u8]) -> u32 {
    let text = node.utf8_text(source).unwrap();
    u32::from_str(text).expect(format!("Cannot parse number: {}", text).as_str())
}

fn get_index_of_label (node: Node, source: &[u8]) -> u32 {
    let label_text = node.utf8_text(source).unwrap();
    let mut root = node.parent().unwrap();
    loop {
        let parent = root.parent();
        if let None = &parent {
            break;
        }
        root = parent.unwrap();
    }
    for i in 0..root.child_count() {
        let child = root.child(i).unwrap();
        if child.kind() == "label" {
            let child_text = child.utf8_text(source).unwrap();
            if child_text == label_text {
                return i as u32;
            }
        }
    }
    panic!("label: {} is not found in source code.", label_text);
}

impl<'tree> From<(Node<'tree>, &'tree [u8])> for Op {
    fn from((node, source): (Node<'tree>, &'tree [u8])) -> Self {
        let kind = node.kind();
        if kind == "label" {
            return Self::Label(get_index_of_label(node, source));
        }
        if kind == "add_op" {
            return Self::Add(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)), OpValue::from((node.child(5).unwrap(), source)));
        }
        if kind == "subtract_op" {
            return Self::Subtract(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)), OpValue::from((node.child(5).unwrap(), source)));
        }
        if kind == "multiplication_op" {
            return Self::Multiplication(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)), OpValue::from((node.child(5).unwrap(), source)));
        }
        if kind == "division_op" {
            return Self::Division(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)), OpValue::from((node.child(5).unwrap(), source)));
        }
        if kind == "modulo_op" {
            return Self::Modulo(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)), OpValue::from((node.child(5).unwrap(), source)));
        }
        if kind == "and_op" {
            return Self::And(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)), OpValue::from((node.child(5).unwrap(), source)));
        }
        if kind == "or_op" {
            return Self::Or(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)), OpValue::from((node.child(5).unwrap(), source)));
        }
        if kind == "not_op" {
            return Self::Not(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)));
        }
        if kind == "jump_op" {
            return Self::Jump(OpValue::from((node.child(1).unwrap(), source)));
        }
        if kind == "compare" {
            return Self::Compare(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)));
        }
        if kind == "branch_equals" {
            return Self::BranchEquals(get_index_of_label(node.child(1).unwrap(), source));
        }
        if kind == "branch_not_equals" {
            return Self::BranchNotEquals(get_index_of_label(node.child(1).unwrap(), source));
        }
        if kind == "branch_greater_than" {
            return Self::BranchGreaterThan(get_index_of_label(node.child(1).unwrap(), source));
        }
        if kind == "branch_less_than" {
            return Self::BranchLessThan(get_index_of_label(node.child(1).unwrap(), source));
        }
        if kind == "push_op" {
            return Self::Push(decimal_number_node_to_u32(node.child(1).unwrap(), source));
        }
        if kind == "pop_op" {
            return Self::Pop(decimal_number_node_to_u32(node.child(1).unwrap(), source));
        }
        if kind == "store_op" {
            return Self::Store(OpValue::from((node.child(1).unwrap(), source)), OpValue::from((node.child(3).unwrap(), source)));
        }
        if kind == "call_op" {
            return Self::Call(OpValue::from((node.child(1).unwrap(), source)));
        }
        if kind == "ret_op" {
            return Self::Ret;
        }
        if kind == "syscall_op" {
            return Self::SysCall(OpValue::from((node.child(1).unwrap(), source)));
        }

        panic!("Unknown instruction set: ${}", node);
    }
}

#[derive(Clone, Debug)]
pub struct Instructions {
    pub ops: Vec<Op>,
}

impl TryFrom<&[u8]> for Instructions {
    type Error = String;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_bluebird_asm::language()).expect("Cannot add bluebird asm language into tree-sitter parser.");
        let parsed = parser.parse(bytes, None).expect("Cannot parse source file.");
        Instructions::try_from((parsed, bytes))
    }
}

impl TryFrom<(Tree, &[u8])> for Instructions {
    type Error = String;

    fn try_from((tree, bytes): (Tree, &[u8])) -> Result<Self, Self::Error> {
        let root = tree.root_node();
        let mut ops = vec![];
        for i in 0..root.child_count() {
            let node = root.child(i).unwrap();
            if node.kind() == "ERROR" {
                return Result::Err(format!("instruction {} is error, line: {}, column: {}", node, node.end_position().row, node.end_position().column));
            }
            ops.push(Op::from((node, bytes)));
        }
        Result::Ok(Instructions { ops: ops })
    }
}

pub trait ReadIntoString {
    fn read_into_string(&mut self) -> String;
}

// impl<READ> ReadIntoString for READ where READ : Read {
//     fn read_into_string(&mut self) -> String {
//         let mut buffer = String::new();
//         self.read_to_string(&mut buffer).expect("Cannot read 'Read' into String.");
//         buffer
//     }
// }

impl ToString for StackMemoryAccess {
    fn to_string(&self) -> String {
        match self {
            StackMemoryAccess::Absolute(value) => format!("[{}]", value),
            StackMemoryAccess::Relative(value) => format!("[-{}]", value),
        }
    }
}

impl ToString for OpValue {
    fn to_string(&self) -> String {
        match self {
            OpValue::Memory(address) => address.to_string(),
            OpValue::Number(value) => format!("{}", value),
            OpValue::Register(register_name) => match register_name {
                RegisterName::Pc => format!("PC"),
                RegisterName::Sp => format!("SP"),
            },
            OpValue::Label(address) => format!("label_{}", address),
            
        }
    }
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Label(index) => format!("label_{index}:"),
            Op::Add(a, b, c) => format!("ADD {}, {}, {}", a.to_string(), b.to_string(), c.to_string()),
            Op::Subtract(a, b, c) => format!("SUB {}, {}, {}", a.to_string(), b.to_string(), c.to_string()),
            Op::Multiplication(a, b, c) => format!("MUL {}, {}, {}", a.to_string(), b.to_string(), c.to_string()),
            Op::Division(a, b, c) => format!("DIV {}, {}, {}", a.to_string(), b.to_string(), c.to_string()),
            Op::Modulo(a, b, c) => format!("MOD {}, {}, {}", a.to_string(), b.to_string(), c.to_string()),
            Op::And(a, b, c) => format!("AND {}, {}, {}", a.to_string(), b.to_string(), c.to_string()),
            Op::Or(a, b, c) => format!("OR {}, {}, {}", a.to_string(), b.to_string(), c.to_string()),
            Op::Not(a, b) => format!("NOT {}, {}", a.to_string(), b.to_string()),
            Op::Jump(addr) => format!("JMP {}:", addr.to_string()),
            Op::Compare(a, b) => format!("CMP {}, {}", a.to_string(), b.to_string()),
            Op::BranchEquals(addr) => format!("BE {}", addr),
            Op::BranchNotEquals(addr) => format!("BNE {}", addr),
            Op::BranchGreaterThan(addr) => format!("BGT {}", addr),
            Op::BranchLessThan(addr) => format!("BLT {}", addr),
            Op::Push(size) => format!("PUSH {}", size),
            Op::Pop(size) => format!("POP {}", size),
            Op::Store(value, addr) => format!("STORE {}, {}", value.to_string(), addr.to_string()),
            Op::Call(value) => format!("CALL {}", value.to_string()),
            Op::Ret => "RET".to_string(),
            Op::SysCall(number) => format!("SYSCALL {}", number.to_string()),
            
        }
    }
}
