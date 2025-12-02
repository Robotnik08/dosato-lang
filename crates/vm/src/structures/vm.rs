use std::rc::Rc;

use crate::structures::value::Value;
use crate::structures::opcode::OpCode;

pub struct VM {
    pub globals: Vec<Value>,
    pub frames: Vec<CallFrame>,
}

pub struct CallFrame {
    pub ip: usize,
    pub registers: Vec<Value>,   // locals for THIS function call
    pub code: Rc<Vec<OpCode>>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            globals: Vec::new(),
            frames: Vec::new(),
        }
    }
}

impl CallFrame {
    pub fn new(code: Rc<Vec<OpCode>>, num_registers: usize) -> Self {
        CallFrame {
            ip: 0,
            registers: vec![Value::Null; num_registers],
            code,
        }
    }
}

impl std::fmt::Debug for CallFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallFrame")
            .field("ip", &self.ip)
            .field("registers", &self.registers)
            .field("code", &self.code)
            .finish()
    }
}

impl std::fmt::Debug for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VM")
            .field("globals", &self.globals)
            .field("frames", &self.frames)
            .finish()
    }
}