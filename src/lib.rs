use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VMError {
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u8),
    #[error("Out of memory at address: {0}")]
    OutOfMemory(usize),
    #[error("Division by zero")]
    DivisionByZero,
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Push = 0x01,
    Pop = 0x02,
    Add = 0x03,
    Sub = 0x04,
    Mul = 0x05,
    Div = 0x06,
    Load = 0x07,
    Store = 0x08,
    Jump = 0x09,
    JumpIf = 0x0A,
    Equal = 0x0B,
    Less = 0x0C,
    Print = 0x0D,
    Halt = 0xFF,
    LessEqual = 0x0E,    
    GreaterEqual = 0x0F, 
}

impl TryFrom<u8> for Opcode {
    type Error = VMError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Opcode::Push),
            0x02 => Ok(Opcode::Pop),
            0x03 => Ok(Opcode::Add),
            0x04 => Ok(Opcode::Sub),
            0x05 => Ok(Opcode::Mul),
            0x06 => Ok(Opcode::Div),
            0x07 => Ok(Opcode::Load),
            0x08 => Ok(Opcode::Store),
            0x09 => Ok(Opcode::Jump),
            0x0A => Ok(Opcode::JumpIf),
            0x0B => Ok(Opcode::Equal),
            0x0C => Ok(Opcode::Less),
            0x0D => Ok(Opcode::Print),
            0xFF => Ok(Opcode::Halt),
            0x0E => Ok(Opcode::LessEqual),
            0x0F => Ok(Opcode::GreaterEqual),
            _ => Err(VMError::InvalidOpcode(value)),
            _ => Err(VMError::InvalidOpcode(value)),
        }
    }
}

pub struct VM {
    /// Program counter
    pc: usize,
    /// Stack for operands
    stack: Vec<i64>,
    /// Program memory (bytecode)
    program: Vec<u8>,
    /// Data memory (heap)
    memory: HashMap<usize, i64>,
    /// Maximum stack size
    stack_limit: usize,
    /// Whether the VM is running
    running: bool,
}

impl VM {
    pub fn new(program: Vec<u8>, stack_limit: usize) -> Self {
        VM {
            pc: 0,
            stack: Vec::with_capacity(stack_limit),
            program,
            memory: HashMap::new(),
            stack_limit,
            running: false,
        }
    }

    fn push(&mut self, value: i64) -> Result<(), VMError> {
        if self.stack.len() >= self.stack_limit {
            return Err(VMError::StackOverflow);
        }
        self.stack.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<i64, VMError> {
        self.stack.pop().ok_or(VMError::StackUnderflow)
    }

    fn fetch(&mut self) -> Option<u8> {
        if self.pc < self.program.len() {
            let opcode = self.program[self.pc];
            self.pc += 1;
            Some(opcode)
        } else {
            None
        }
    }

    fn fetch_i64(&mut self) -> Option<i64> {
        if self.pc + 8 <= self.program.len() {
            let bytes = &self.program[self.pc..self.pc + 8];
            self.pc += 8;
            Some(i64::from_le_bytes(bytes.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn execute_next(&mut self) -> Result<bool, VMError> {
        let opcode = self.fetch().ok_or(VMError::InvalidOpcode(0))?;
        match Opcode::try_from(opcode)? {
            Opcode::Push => {
                let value = self.fetch_i64().ok_or(VMError::InvalidOpcode(opcode))?;
                self.push(value)?;
            }
            Opcode::Pop => {
                self.pop()?;
            }
            Opcode::Add => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a + b)?;
            }
            Opcode::Sub => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a - b)?;
            }
            Opcode::Mul => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a * b)?;
            }
            Opcode::Div => {
                let b = self.pop()?;
                let a = self.pop()?;
                if b == 0 {
                    return Err(VMError::DivisionByZero);
                }
                self.push(a / b)?;
            }
            Opcode::Load => {
                let addr = self.pop()? as usize;
                let value = *self.memory.get(&addr).unwrap_or(&0);
                self.push(value)?;
            }
            Opcode::Store => {
                let value = self.pop()?;
                let addr = self.pop()? as usize;
                self.memory.insert(addr, value);
            }
            Opcode::Jump => {
                let addr = self.pop()? as usize;
                if addr >= self.program.len() {
                    return Err(VMError::OutOfMemory(addr));
                }
                self.pc = addr;
            }
            Opcode::JumpIf => {
                let addr = self.pop()? as usize;
                let condition = self.pop()?;
                if condition != 0 {
                    if addr >= self.program.len() {
                        return Err(VMError::OutOfMemory(addr));
                    }
                    self.pc = addr;
                }
            }
            Opcode::Equal => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(if a == b { 1 } else { 0 })?;
            }
            Opcode::Less => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(if a < b { 1 } else { 0 })?;
            }
            Opcode::Print => {
                let value = self.pop()?;
                println!("Output: {}", value);
            }
            Opcode::Halt => {
                self.running = false;
                return Ok(false);
            },
            Opcode::LessEqual => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(if a <= b { 1 } else { 0 })?;
            }
            Opcode::GreaterEqual => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(if a >= b { 1 } else { 0 })?;
            }
        }
        Ok(true)
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        self.running = true;
        while self.running {
            if !self.execute_next()? {
                break;
            }
        }
        Ok(())
    }

    pub fn get_stack(&self) -> &[i64] {
        &self.stack
    }

    pub fn get_memory(&self) -> &HashMap<usize, i64> {
        &self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let program = vec![
            Opcode::Push as u8,
            42, 0, 0, 0, 0, 0, 0, 0,  // Push 42
            Opcode::Push as u8,
            123, 0, 0, 0, 0, 0, 0, 0,  // Push 123
            Opcode::Pop as u8,
            Opcode::Halt as u8,
        ];

        let mut vm = VM::new(program, 100);
        vm.run().unwrap();
        
        assert_eq!(vm.get_stack(), &[42]);
    }

    #[test]
    fn test_arithmetic() {
        let program = vec![
            Opcode::Push as u8,
            10, 0, 0, 0, 0, 0, 0, 0,   // Push 10
            Opcode::Push as u8,
            5, 0, 0, 0, 0, 0, 0, 0,    // Push 5
            Opcode::Add as u8,         // 10 + 5
            Opcode::Push as u8,
            2, 0, 0, 0, 0, 0, 0, 0,    // Push 2
            Opcode::Mul as u8,         // (10 + 5) * 2
            Opcode::Halt as u8,
        ];

        let mut vm = VM::new(program, 100);
        vm.run().unwrap();
        
        assert_eq!(vm.get_stack(), &[30]);
    }
}