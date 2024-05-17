use crate::core::ops::{OpValue, StackMemoryAccess};

use super::ops::{Instructions, Op};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-05-13 14:55:03
 * @modify date 2024-05-13 14:55:03
 * @desc [description]
*/



pub struct ComparisonRegister {
    pub values: (u32, u32)
}

pub struct StackPointerRegister {
    pub stack_pointer: u32
}

pub struct ProgramCounterRegister {
    pub program_counter: u32,
}

pub trait SysCallHandler {
    /// handle syscall from vm
    /// returns: return true if this syscall is handled success otherwice false.
    fn handle(&mut self, syscall_number: u32) -> bool;
}

pub struct VirtualMachine<SYSCALL_HANDLER> {
    pub ops: Vec<Op>,
    pub stack_memory: Vec<u8>,
    pub comparison_register: ComparisonRegister,
    pub stack_pointer_register: StackPointerRegister,
    pub program_counter: ProgramCounterRegister,
    pub syscall_handler: SYSCALL_HANDLER,
}

impl<SYSCALL_HANDLER : SysCallHandler> VirtualMachine<SYSCALL_HANDLER> {
    pub fn new(instructions: Instructions, stack_size: u32, syscall_handler: SYSCALL_HANDLER) -> Self {
        Self {
            ops: instructions.ops,
            comparison_register: ComparisonRegister { values: (0, 0) },
            stack_pointer_register: StackPointerRegister { stack_pointer: 0u32 },
            program_counter: ProgramCounterRegister { program_counter: 0u32 },
            stack_memory: {
                let mut vec = Vec::with_capacity(stack_size as usize);
                vec.append(&mut (0..stack_size).into_iter().map(|item|0).collect());
                vec
            },
            syscall_handler
        }
    }
    pub fn clock(&mut self) {
        let op = &self.ops[self.program_counter.program_counter as usize];
        self.program_counter.program_counter += 1;
        fn extract_op_value_as_mem_address<SYSCALL_HANDLER>(_self: &VirtualMachine<SYSCALL_HANDLER>, op_value: &OpValue) -> u32 {
            let address = match op_value {
                OpValue::Memory(addr) => {
                    match addr {
                        StackMemoryAccess::Absolute(value) => *value,
                        StackMemoryAccess::Relative(value) => _self.stack_pointer_register.stack_pointer - value,
                    }
                },
                OpValue::Number(_) => panic!("number cannot be converted to memory address directly, please use memory address."),
                OpValue::Register(_) => panic!("number cannot be converted to memory address directly, please use memory address."),
                OpValue::Label(_) => panic!("number cannot be converted to memory address directly, please use memory address."),
                
            };
            if address > _self.stack_pointer_register.stack_pointer {
                panic!("attampted to access memory outside of current stack, current stack: {}, attampted to access: {}", _self.stack_pointer_register.stack_pointer, address);
            }
            address
        }
        fn extract_op_value<HANDLER>(_self: &VirtualMachine<HANDLER>, op_value: &OpValue) -> u32 {
            match op_value {
                OpValue::Memory(value) => {
                    let mem_address = extract_op_value_as_mem_address(_self, op_value) as usize;
                    let mem_value = &_self.stack_memory[mem_address..mem_address + 4];
                    let mem_value = u32::from_le_bytes([mem_value[0], mem_value[1], mem_value[2], mem_value[3]]);
                    mem_value
                },
                OpValue::Number(value) => *value,
                OpValue::Register(register_name) => match register_name {
                    crate::core::ops::RegisterName::Pc => _self.program_counter.program_counter,
                    crate::core::ops::RegisterName::Sp => _self.stack_pointer_register.stack_pointer,
                },
                OpValue::Label(position) => *position,
                
            }
        }
        // fn store_value_into_mem(_self: &VirtualMachine, stack_memory: &mut [u8], mem_address: &OpValue, value: u32) {
        //     let address = extract_op_value_as_mem_address(_self, mem_address) as usize;
        //     let result = value.to_le_bytes();
        //     stack_memory[address..address + 4].copy_from_slice(&result);
        // }
        match op {
            Op::Label(label) => {},
            Op::Add(a, b, c) => {
                let address = extract_op_value_as_mem_address(self, c) as usize;
                let result = (extract_op_value(self, a) + extract_op_value(self, b)).to_le_bytes();
                (&mut self.stack_memory[address..address + 4]).copy_from_slice(&result);
            },
            Op::Subtract(a, b, c) => {
                let address = extract_op_value_as_mem_address(self, c) as usize;
                let result = (extract_op_value(self, a) - extract_op_value(self, b)).to_le_bytes();
                (&mut self.stack_memory[address..address + 4]).copy_from_slice(&result);
            },
            Op::Multiplication(a, b, c) => {
                let address = extract_op_value_as_mem_address(self, c) as usize;
                let result = (extract_op_value(self, a) * extract_op_value(self, b)).to_le_bytes();
                (&mut self.stack_memory[address..address + 4]).copy_from_slice(&result);
            },
            Op::Division(a, b, c) => {
                let address = extract_op_value_as_mem_address(self, c) as usize;
                let result = (extract_op_value(self, a) / extract_op_value(self, b)).to_le_bytes();
                (&mut self.stack_memory[address..address + 4]).copy_from_slice(&result);
            },
            Op::Modulo(a, b, c) => {
                let address = extract_op_value_as_mem_address(self, c) as usize;
                let result = (extract_op_value(self, a) % extract_op_value(self, b)).to_le_bytes();
                (&mut self.stack_memory[address..address + 4]).copy_from_slice(&result);
            }
            Op::And(a, b, c) => {
                let address = extract_op_value_as_mem_address(self, c) as usize;
                let result = (extract_op_value(self, a) & extract_op_value(self, b)).to_le_bytes();
                (&mut self.stack_memory[address..address + 4]).copy_from_slice(&result);
            }
            Op::Or(a, b, c) => {
                let address = extract_op_value_as_mem_address(self, c) as usize;
                let result = (extract_op_value(self, a) | extract_op_value(self, b)).to_le_bytes();
                (&mut self.stack_memory[address..address + 4]).copy_from_slice(&result);
            }
            Op::Not(a, b) => {
                let address = extract_op_value_as_mem_address(self, b) as usize;
                let result = (!extract_op_value(self, a)).to_le_bytes();
                (&mut self.stack_memory[address..address + 4]).copy_from_slice(&result);
            }
            Op::Jump(location) => {
                self.program_counter.program_counter = extract_op_value(self, location);
            },
            Op::Compare(a, b) => {
                self.comparison_register.values = (extract_op_value(self, a), extract_op_value(self, b));
            },
            Op::BranchEquals(label_location) => {
                if self.comparison_register.values.0 == self.comparison_register.values.1 {
                    self.program_counter.program_counter = *label_location;
                }
            },
            Op::BranchNotEquals(label_location) => {
                if self.comparison_register.values.0 != self.comparison_register.values.1 {
                    self.program_counter.program_counter = *label_location;
                }
            },
            Op::BranchGreaterThan(label_location) => {
                if self.comparison_register.values.0 > self.comparison_register.values.1 {
                    self.program_counter.program_counter = *label_location;
                }
            },
            Op::BranchLessThan(label_location) => {
                if self.comparison_register.values.0 < self.comparison_register.values.1 {
                    self.program_counter.program_counter = *label_location;
                }
            },
            Op::Push(count) => {
                // for i in 0..*count {
                //     self.stack_memory.push(0);
                // }
                self.stack_pointer_register.stack_pointer += count;
            },
            Op::Pop(count) => {
                // for i in 0..*count {
                //     self.stack_memory.pop();
                // }
                self.stack_pointer_register.stack_pointer -= count;
            },
            Op::Store(op_value, memory_location) => {
                let address = extract_op_value_as_mem_address(self, memory_location) as usize;
                let result = extract_op_value(self, op_value).to_le_bytes();
                (&mut self.stack_memory[address..address + 4]).copy_from_slice(&result);
            },
            Op::Call(op_value) => {
                //save next instruction address into memory.
                let next_instruction_addr = self.program_counter.program_counter;
                let mem_addr = self.stack_pointer_register.stack_pointer as usize;
                let result = next_instruction_addr.to_le_bytes();
                (&mut self.stack_memory[mem_addr..mem_addr + 4]).copy_from_slice(&result);
                self.stack_pointer_register.stack_pointer += 4;
                //jump to destination address.
                self.program_counter.program_counter = extract_op_value(self, op_value);
            },
            Op::Ret => {
                self.stack_pointer_register.stack_pointer -= 4;//pop the resumable instruction from stack memory
                let sp = self.stack_pointer_register.stack_pointer as usize;
                let instruction_addr_bytes = &self.stack_memory[sp..sp + 4];
                let instruction_addr = u32::from_le_bytes([instruction_addr_bytes[0], instruction_addr_bytes[1], instruction_addr_bytes[2], instruction_addr_bytes[3]]);
                self.program_counter.program_counter = instruction_addr;//and resume execution from jumping back.
            },
            Op::SysCall(value) => {

            },
            
        }
    }

    pub fn reset(&mut self) {
        self.program_counter.program_counter = 0;
        self.comparison_register.values = (0, 0);
        self.stack_pointer_register.stack_pointer = 0;
        self.stack_memory.fill(0);
    }

    pub fn end_of_instruction(&self) -> bool {
        self.program_counter.program_counter as usize >= self.ops.len()
    }
}
