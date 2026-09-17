use crate::chunk::Chunk;
use crate::opcode::{BytecodeValue, OpCode};
use scratch_ir::{
    IrBinaryOp, IrEventHandler, IrExpr, IrInstruction, IrProgram, IrTrigger, IrUnaryOp, IrValue,
};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BytecodeProgram {
    pub events: Vec<BytecodeEventHandler>,
    pub functions: HashMap<String, Chunk>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BytecodeEventHandler {
    pub trigger: IrTrigger,
    pub chunk: Chunk,
}

pub struct BytecodeCompiler {
    loop_counter: usize,
}

impl Default for BytecodeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        Self { loop_counter: 0 }
    }

    pub fn compile_program(&mut self, ir: &IrProgram) -> BytecodeProgram {
        let mut events = Vec::new();
        for event in &ir.events {
            events.push(self.compile_event(event));
        }

        let mut functions = HashMap::new();
        for func in &ir.functions {
            let mut chunk = Chunk::new();
            for stmt in &func.instructions {
                self.compile_instruction(stmt, &mut chunk);
            }
            chunk.emit(OpCode::PushNil);
            chunk.emit(OpCode::Return);
            functions.insert(func.name.clone(), chunk);
        }

        BytecodeProgram { events, functions }
    }

    fn compile_event(&mut self, event: &IrEventHandler) -> BytecodeEventHandler {
        let mut chunk = Chunk::new();
        for stmt in &event.instructions {
            self.compile_instruction(stmt, &mut chunk);
        }
        chunk.emit(OpCode::Halt);

        BytecodeEventHandler {
            trigger: event.trigger.clone(),
            chunk,
        }
    }

    fn compile_instruction(&mut self, instr: &IrInstruction, chunk: &mut Chunk) {
        match instr {
            IrInstruction::Assign { target, expr } => {
                self.compile_expr(expr, chunk);
                chunk.emit(OpCode::StoreVar(target.clone()));
            }
            IrInstruction::CallBlock { name, args } => {
                for arg in args {
                    self.compile_expr(arg, chunk);
                }
                chunk.emit(OpCode::CallBlock {
                    name: name.clone(),
                    arg_count: args.len(),
                });
            }
            IrInstruction::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.compile_expr(condition, chunk);
                let jump_false = chunk.emit_jump(OpCode::JumpIfFalse(0));

                for s in then_branch {
                    self.compile_instruction(s, chunk);
                }

                if !else_branch.is_empty() {
                    let jump_end = chunk.emit_jump(OpCode::Jump(0));
                    chunk.patch_jump(jump_false);

                    for s in else_branch {
                        self.compile_instruction(s, chunk);
                    }
                    chunk.patch_jump(jump_end);
                } else {
                    chunk.patch_jump(jump_false);
                }
            }
            IrInstruction::Repeat { count, body } => {
                self.loop_counter += 1;
                let counter_var = format!("__loop_{}", self.loop_counter);

                self.compile_expr(count, chunk);
                chunk.emit(OpCode::StoreVar(counter_var.clone()));

                let loop_start = chunk.instructions.len();

                // Condition: counter > 0
                chunk.emit(OpCode::LoadVar(counter_var.clone()));
                let zero_idx = chunk.add_constant(BytecodeValue::Number(0.0));
                chunk.emit(OpCode::PushConst(zero_idx));
                chunk.emit(OpCode::Greater);

                let exit_jump = chunk.emit_jump(OpCode::JumpIfFalse(0));

                // Loop body
                for s in body {
                    self.compile_instruction(s, chunk);
                }

                // Decrement counter: counter = counter - 1
                chunk.emit(OpCode::LoadVar(counter_var.clone()));
                let one_idx = chunk.add_constant(BytecodeValue::Number(1.0));
                chunk.emit(OpCode::PushConst(one_idx));
                chunk.emit(OpCode::Sub);
                chunk.emit(OpCode::StoreVar(counter_var));

                // Loop back
                chunk.emit(OpCode::Jump(loop_start));

                // Exit
                chunk.patch_jump(exit_jump);
            }
            IrInstruction::Return(value) => {
                if let Some(v) = value {
                    self.compile_expr(v, chunk);
                } else {
                    chunk.emit(OpCode::PushNil);
                }
                chunk.emit(OpCode::Return);
            }
            IrInstruction::Assert(expr) => {
                self.compile_expr(expr, chunk);
                chunk.emit(OpCode::Assert);
            }
        }
    }

    fn compile_expr(&mut self, expr: &IrExpr, chunk: &mut Chunk) {
        match expr {
            IrExpr::Constant(val) => {
                let bc_val = match val {
                    IrValue::Number(n) => BytecodeValue::Number(*n),
                    IrValue::String(s) => BytecodeValue::String(s.clone()),
                    IrValue::Bool(b) => BytecodeValue::Bool(*b),
                };
                let idx = chunk.add_constant(bc_val);
                chunk.emit(OpCode::PushConst(idx));
            }
            IrExpr::Var(name) => {
                chunk.emit(OpCode::LoadVar(name.clone()));
            }
            IrExpr::Binary { left, op, right } => {
                self.compile_expr(left, chunk);
                self.compile_expr(right, chunk);
                match op {
                    IrBinaryOp::Add => chunk.emit(OpCode::Add),
                    IrBinaryOp::Sub => chunk.emit(OpCode::Sub),
                    IrBinaryOp::Mul => chunk.emit(OpCode::Mul),
                    IrBinaryOp::Div => chunk.emit(OpCode::Div),
                    IrBinaryOp::Mod => chunk.emit(OpCode::Mod),
                    IrBinaryOp::Equal => chunk.emit(OpCode::Equal),
                    IrBinaryOp::NotEqual => chunk.emit(OpCode::NotEqual),
                    IrBinaryOp::Less => chunk.emit(OpCode::Less),
                    IrBinaryOp::LessEqual => chunk.emit(OpCode::LessEqual),
                    IrBinaryOp::Greater => chunk.emit(OpCode::Greater),
                    IrBinaryOp::GreaterEqual => chunk.emit(OpCode::GreaterEqual),
                    IrBinaryOp::And => chunk.emit(OpCode::And),
                    IrBinaryOp::Or => chunk.emit(OpCode::Or),
                };
            }
            IrExpr::Unary { op, expr } => {
                self.compile_expr(expr, chunk);
                match op {
                    IrUnaryOp::Not => chunk.emit(OpCode::Not),
                    IrUnaryOp::Neg => chunk.emit(OpCode::Neg),
                };
            }
            IrExpr::CallBlock { name, args } => {
                for arg in args {
                    self.compile_expr(arg, chunk);
                }
                chunk.emit(OpCode::CallBlock {
                    name: name.clone(),
                    arg_count: args.len(),
                });
            }
        }
    }
}
