use scratch_blocks::BlockRegistry;
use scratch_bytecode::{BytecodeValue, Chunk, OpCode};
use scratch_runtime::{MovementSystem, RuntimeValue, World};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VmError {
    #[error("Execution exceeded step limit of {0} instructions (infinite loop protection)")]
    OutOfFuel(usize),

    #[error("Stack underflow")]
    StackUnderflow,

    #[error("Assertion failed")]
    AssertionFailed,

    #[error("Runtime error: {0}")]
    Runtime(String),
}

pub struct Vm {
    pub stack: Vec<BytecodeValue>,
    pub max_fuel: usize,
    pub fuel: usize,
}

impl Default for Vm {
    fn default() -> Self {
        Self::new(100_000)
    }
}

impl Vm {
    pub fn new(max_fuel: usize) -> Self {
        Self {
            stack: Vec::with_capacity(256),
            max_fuel,
            fuel: max_fuel,
        }
    }

    pub fn reset_fuel(&mut self) {
        self.fuel = self.max_fuel;
    }

    pub fn execute(
        &mut self,
        chunk: &Chunk,
        world: &mut World,
        registry: &BlockRegistry,
    ) -> Result<(), VmError> {
        self.reset_fuel();
        self.stack.clear();
        let mut ip = 0;

        while ip < chunk.instructions.len() {
            if self.fuel == 0 {
                return Err(VmError::OutOfFuel(self.max_fuel));
            }
            self.fuel -= 1;

            let op = &chunk.instructions[ip];
            ip += 1;

            match op {
                OpCode::PushConst(idx) => {
                    let val = chunk.constants.get(*idx).cloned().unwrap_or(BytecodeValue::Nil);
                    self.stack.push(val);
                }
                OpCode::PushNil => self.stack.push(BytecodeValue::Nil),
                OpCode::PushTrue => self.stack.push(BytecodeValue::Bool(true)),
                OpCode::PushFalse => self.stack.push(BytecodeValue::Bool(false)),
                OpCode::LoadVar(name) => {
                    if world.get_entity_by_name(name).is_some() {
                        self.stack.push(BytecodeValue::Object(name.clone()));
                    } else if let Some(rv) = world.get_var(name) {
                        self.stack.push(runtime_to_bytecode(rv));
                    } else {
                        self.stack.push(BytecodeValue::Nil);
                    }
                }
                OpCode::StoreVar(name) => {
                    let val = self.pop()?;
                    world.set_var(name, bytecode_to_runtime(&val));
                }
                OpCode::Pop => {
                    self.pop()?;
                }
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (&a, &b) {
                        (BytecodeValue::Number(n1), BytecodeValue::Number(n2)) => {
                            self.stack.push(BytecodeValue::Number(n1 + n2));
                        }
                        _ => {
                            self.stack.push(BytecodeValue::String(format!("{}{}", a, b)));
                        }
                    }
                }
                OpCode::Sub => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(BytecodeValue::Number(a - b));
                }
                OpCode::Mul => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(BytecodeValue::Number(a * b));
                }
                OpCode::Div => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    let res = if b == 0.0 { 0.0 } else { a / b };
                    self.stack.push(BytecodeValue::Number(res));
                }
                OpCode::Mod => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    let res = if b == 0.0 { 0.0 } else { a % b };
                    self.stack.push(BytecodeValue::Number(res));
                }
                OpCode::Equal => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(BytecodeValue::Bool(a == b));
                }
                OpCode::NotEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(BytecodeValue::Bool(a != b));
                }
                OpCode::Less => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(BytecodeValue::Bool(a < b));
                }
                OpCode::LessEqual => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(BytecodeValue::Bool(a <= b));
                }
                OpCode::Greater => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(BytecodeValue::Bool(a > b));
                }
                OpCode::GreaterEqual => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(BytecodeValue::Bool(a >= b));
                }
                OpCode::And => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(BytecodeValue::Bool(is_truthy(&a) && is_truthy(&b)));
                }
                OpCode::Or => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(BytecodeValue::Bool(is_truthy(&a) || is_truthy(&b)));
                }
                OpCode::Not => {
                    let a = self.pop()?;
                    self.stack.push(BytecodeValue::Bool(!is_truthy(&a)));
                }
                OpCode::Neg => {
                    let a = self.pop_number()?;
                    self.stack.push(BytecodeValue::Number(-a));
                }
                OpCode::Jump(dest) => {
                    ip = *dest;
                }
                OpCode::JumpIfFalse(dest) => {
                    let cond = self.pop()?;
                    if !is_truthy(&cond) {
                        ip = *dest;
                    }
                }
                OpCode::CallBlock { name, arg_count } => {
                    let mut args = Vec::with_capacity(*arg_count);
                    for _ in 0..*arg_count {
                        args.push(self.pop()?);
                    }
                    args.reverse();

                    let ret_val = self.invoke_block(name, &args, world, registry)?;
                    if let Some(v) = ret_val {
                        self.stack.push(v);
                    }
                }
                OpCode::CallFunc { .. } => {
                    // For user defined functions
                }
                OpCode::Assert => {
                    let val = self.pop()?;
                    if !is_truthy(&val) {
                        return Err(VmError::AssertionFailed);
                    }
                }
                OpCode::Return | OpCode::Halt => {
                    break;
                }
            }
        }

        Ok(())
    }

    fn pop(&mut self) -> Result<BytecodeValue, VmError> {
        self.stack.pop().ok_or(VmError::StackUnderflow)
    }

    fn pop_number(&mut self) -> Result<f64, VmError> {
        let val = self.pop()?;
        match val {
            BytecodeValue::Number(n) => Ok(n),
            _ => Ok(0.0),
        }
    }

    fn invoke_block(
        &mut self,
        name: &str,
        args: &[BytecodeValue],
        world: &mut World,
        _registry: &BlockRegistry,
    ) -> Result<Option<BytecodeValue>, VmError> {
        match name {
            "move" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let dx = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    let dy = args.get(2).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    MovementSystem::execute_move(world, target, dx, dy);
                }
                Ok(None)
            }
            "jump" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let force = args.get(1).and_then(|v| as_f64(v)).unwrap_or(10.0) as f32;
                    MovementSystem::execute_jump(world, target, force);
                }
                Ok(None)
            }
            "stop" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    MovementSystem::execute_stop(world, target);
                }
                Ok(None)
            }
            "teleport" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let x = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    let y = args.get(2).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    MovementSystem::execute_teleport(world, target, x, y);
                }
                Ok(None)
            }
            "touching" => {
                let target_a = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let target_b = args.get(1).and_then(|v| as_str(v)).unwrap_or("");

                if let (Some(a), Some(b)) = (
                    world.get_entity_by_name(target_a),
                    world.get_entity_by_name(target_b),
                ) {
                    let half_w_a = a.size[0] / 2.0;
                    let half_h_a = a.size[1] / 2.0;
                    let half_w_b = b.size[0] / 2.0;
                    let half_h_b = b.size[1] / 2.0;

                    let overlap_x = (a.transform.x - b.transform.x).abs() <= (half_w_a + half_w_b);
                    let overlap_y = (a.transform.y - b.transform.y).abs() <= (half_h_a + half_h_b);

                    Ok(Some(BytecodeValue::Bool(overlap_x && overlap_y && a.visible && b.visible)))
                } else {
                    Ok(Some(BytecodeValue::Bool(false)))
                }
            }
            "damage" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let amount = args.get(1).and_then(|v| as_f64(v)).unwrap_or(1.0);
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.health -= amount;
                    }
                }
                Ok(None)
            }
            "heal" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let amount = args.get(1).and_then(|v| as_f64(v)).unwrap_or(1.0);
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.health += amount;
                    }
                }
                Ok(None)
            }
            "respawn" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.x = 0.0;
                        ent.transform.y = 0.0;
                        ent.velocity = (0.0, 0.0);
                        ent.health = 3.0;
                    }
                }
                Ok(None)
            }
            "collect" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.visible = false;
                    }
                }
                Ok(None)
            }
            "background.set" => {
                if let Some(bg) = args.first().and_then(|v| as_str(v)) {
                    world.background = bg.to_string();
                }
                Ok(None)
            }
            "camera.follow" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    world.camera_follow_target = Some(target.to_string());
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}

fn as_str<'a>(val: &'a BytecodeValue) -> Option<&'a str> {
    match val {
        BytecodeValue::String(s) => Some(s.as_str()),
        BytecodeValue::Object(s) => Some(s.as_str()),
        _ => None,
    }
}

fn as_f64(val: &BytecodeValue) -> Option<f64> {
    match val {
        BytecodeValue::Number(n) => Some(*n),
        _ => None,
    }
}

fn is_truthy(val: &BytecodeValue) -> bool {
    match val {
        BytecodeValue::Bool(b) => *b,
        BytecodeValue::Nil => false,
        BytecodeValue::Number(n) => *n != 0.0,
        BytecodeValue::String(s) => !s.is_empty(),
        BytecodeValue::Object(_) => true,
    }
}

fn runtime_to_bytecode(rv: &RuntimeValue) -> BytecodeValue {
    match rv {
        RuntimeValue::Nil => BytecodeValue::Nil,
        RuntimeValue::Number(n) => BytecodeValue::Number(*n),
        RuntimeValue::String(s) => BytecodeValue::String(s.clone()),
        RuntimeValue::Bool(b) => BytecodeValue::Bool(*b),
        RuntimeValue::Object(o) => BytecodeValue::Object(o.clone()),
    }
}

fn bytecode_to_runtime(bv: &BytecodeValue) -> RuntimeValue {
    match bv {
        BytecodeValue::Nil => RuntimeValue::Nil,
        BytecodeValue::Number(n) => RuntimeValue::Number(*n),
        BytecodeValue::String(s) => RuntimeValue::String(s.clone()),
        BytecodeValue::Bool(b) => RuntimeValue::Bool(*b),
        BytecodeValue::Object(o) => RuntimeValue::Object(o.clone()),
    }
}
