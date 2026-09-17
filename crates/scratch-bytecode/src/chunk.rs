use crate::opcode::{BytecodeValue, OpCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Chunk {
    pub instructions: Vec<OpCode>,
    pub constants: Vec<BytecodeValue>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, val: BytecodeValue) -> usize {
        self.constants.push(val);
        self.constants.len() - 1
    }

    pub fn emit(&mut self, op: OpCode) -> usize {
        self.instructions.push(op);
        self.instructions.len() - 1
    }

    pub fn emit_jump(&mut self, placeholder: OpCode) -> usize {
        self.instructions.push(placeholder);
        self.instructions.len() - 1
    }

    pub fn patch_jump(&mut self, index: usize) {
        let target = self.instructions.len();
        match &mut self.instructions[index] {
            OpCode::Jump(dest) => *dest = target,
            OpCode::JumpIfFalse(dest) => *dest = target,
            _ => panic!("Attempted to patch non-jump instruction at index {}", index),
        }
    }

    pub fn disassemble(&self, name: &str) -> String {
        let mut out = format!("=== Chunk: {} ===\n", name);
        for (i, op) in self.instructions.iter().enumerate() {
            out.push_str(&format!("{:04} ", i));
            match op {
                OpCode::PushConst(idx) => {
                    let val = self.constants.get(*idx).map(|v| format!("{}", v)).unwrap_or_default();
                    out.push_str(&format!("OP_PUSH_CONST {} ({})\n", idx, val));
                }
                OpCode::LoadVar(name) => out.push_str(&format!("OP_LOAD_VAR {}\n", name)),
                OpCode::StoreVar(name) => out.push_str(&format!("OP_STORE_VAR {}\n", name)),
                OpCode::Jump(dest) => out.push_str(&format!("OP_JUMP -> {:04}\n", dest)),
                OpCode::JumpIfFalse(dest) => out.push_str(&format!("OP_JUMP_IF_FALSE -> {:04}\n", dest)),
                OpCode::CallBlock { name, arg_count } => {
                    out.push_str(&format!("OP_CALL_BLOCK {} (args: {})\n", name, arg_count));
                }
                OpCode::CallFunc { name, arg_count } => {
                    out.push_str(&format!("OP_CALL_FUNC {} (args: {})\n", name, arg_count));
                }
                other => out.push_str(&format!("{:?}\n", other)),
            }
        }
        out
    }
}
