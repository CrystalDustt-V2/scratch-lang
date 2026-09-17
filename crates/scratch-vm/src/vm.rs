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
            "scene.switch" => {
                if let Some(scene_name) = args.first().and_then(|v| as_str(v)) {
                    world.set_var("__next_scene", RuntimeValue::String(scene_name.to_string()));
                }
                Ok(None)
            }
            "scene.restart" => {
                world.set_var("__restart_scene", RuntimeValue::Bool(true));
                Ok(None)
            }
            "change_x" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let dx = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.x += dx;
                    }
                }
                Ok(None)
            }
            "set_x" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let x = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.x = x;
                    }
                }
                Ok(None)
            }
            "change_y" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let dy = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.y += dy;
                    }
                }
                Ok(None)
            }
            "set_y" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let y = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.y = y;
                    }
                }
                Ok(None)
            }
            "turn_right" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let deg = args.get(1).and_then(|v| as_f64(v)).unwrap_or(15.0) as f32;
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.rotation += deg;
                    }
                }
                Ok(None)
            }
            "turn_left" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let deg = args.get(1).and_then(|v| as_f64(v)).unwrap_or(15.0) as f32;
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.rotation -= deg;
                    }
                }
                Ok(None)
            }
            "point_in_direction" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let deg = args.get(1).and_then(|v| as_f64(v)).unwrap_or(90.0) as f32;
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.rotation = deg;
                    }
                }
                Ok(None)
            }
            "bounce_on_edge" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        if ent.transform.x <= 0.0 || ent.transform.x >= 1280.0 {
                            ent.velocity.0 = -ent.velocity.0;
                        }
                        if ent.transform.y <= 0.0 || ent.transform.y >= 720.0 {
                            ent.velocity.1 = -ent.velocity.1;
                        }
                    }
                }
                Ok(None)
            }
            "x_position" => {
                let x = args
                    .first()
                    .and_then(|v| as_str(v))
                    .and_then(|name| world.get_entity_by_name(name))
                    .map(|ent| ent.transform.x as f64)
                    .unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(x)))
            }
            "y_position" => {
                let y = args
                    .first()
                    .and_then(|v| as_str(v))
                    .and_then(|name| world.get_entity_by_name(name))
                    .map(|ent| ent.transform.y as f64)
                    .unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(y)))
            }
            "distance_to" => {
                let a = args.first().and_then(|v| as_str(v)).and_then(|n| world.get_entity_by_name(n));
                let b = args.get(1).and_then(|v| as_str(v)).and_then(|n| world.get_entity_by_name(n));
                if let (Some(ea), Some(eb)) = (a, b) {
                    let dx = (ea.transform.x - eb.transform.x) as f64;
                    let dy = (ea.transform.y - eb.transform.y) as f64;
                    Ok(Some(BytecodeValue::Number((dx * dx + dy * dy).sqrt())))
                } else {
                    Ok(Some(BytecodeValue::Number(0.0)))
                }
            }
            "show" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.visible = true;
                    }
                }
                Ok(None)
            }
            "hide" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.visible = false;
                    }
                }
                Ok(None)
            }
            "set_size" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let pct = args.get(1).and_then(|v| as_f64(v)).unwrap_or(100.0) as f32;
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.scale_x = pct / 100.0;
                        ent.transform.scale_y = pct / 100.0;
                    }
                }
                Ok(None)
            }
            "change_size" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let delta = args.get(1).and_then(|v| as_f64(v)).unwrap_or(10.0) as f32;
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.scale_x += delta / 100.0;
                        ent.transform.scale_y += delta / 100.0;
                    }
                }
                Ok(None)
            }
            "mouse_x" => Ok(Some(BytecodeValue::Number(world.mouse_pos.0 as f64))),
            "mouse_y" => Ok(Some(BytecodeValue::Number(world.mouse_pos.1 as f64))),
            "mouse_down" => Ok(Some(BytecodeValue::Bool(world.mouse_down))),
            "key_pressed" => {
                let act = args.first().and_then(|v| as_str(v)).unwrap_or("");
                Ok(Some(BytecodeValue::Bool(world.input_actions_down.contains(act))))
            }
            "get_timer" => {
                let t = world.get_var("__runtime_timer").map(|v| match v { RuntimeValue::Number(n) => *n, _ => 0.0 }).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(t)))
            }
            "reset_timer" => {
                world.set_var("__runtime_timer", RuntimeValue::Number(0.0));
                Ok(None)
            }
            "random" => {
                let min = args.first().and_then(|v| as_f64(v)).unwrap_or(1.0);
                let max = args.get(1).and_then(|v| as_f64(v)).unwrap_or(10.0);
                let seed = world.get_var("__random_seed").map(|v| match v { RuntimeValue::Number(n) => *n, _ => 12345.0 }).unwrap_or(12345.0);
                let next_seed = (seed * 1103515245.0 + 12345.0) % 2147483648.0;
                world.set_var("__random_seed", RuntimeValue::Number(next_seed));
                let ratio = next_seed / 2147483648.0;
                let val = min + ratio * (max - min);
                Ok(Some(BytecodeValue::Number(val.round())))
            }
            "math.round" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(n.round())))
            }
            "math.abs" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(n.abs())))
            }
            "math.sqrt" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(if n < 0.0 { 0.0 } else { n.sqrt() })))
            }
            "math.sin" => {
                let deg = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(deg.to_radians().sin())))
            }
            "math.cos" => {
                let deg = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(deg.to_radians().cos())))
            }
            "math.floor" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(n.floor())))
            }
            "math.ceil" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(n.ceil())))
            }
            "text.join" => {
                let to_text = |v: Option<&BytecodeValue>| -> String {
                    match v {
                        Some(BytecodeValue::String(s)) | Some(BytecodeValue::Object(s)) => s.clone(),
                        Some(BytecodeValue::Number(n)) => n.to_string(),
                        Some(BytecodeValue::Bool(b)) => b.to_string(),
                        Some(BytecodeValue::Nil) | None => String::new(),
                    }
                };
                let a = to_text(args.first());
                let b = to_text(args.get(1));
                Ok(Some(BytecodeValue::String(format!("{}{}", a, b))))
            }
            "text.length" => {
                let s = args.first().and_then(|v| as_str(v)).unwrap_or("");
                Ok(Some(BytecodeValue::Number(s.chars().count() as f64)))
            }
            "text.contains" => {
                let s = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let sub = args.get(1).and_then(|v| as_str(v)).unwrap_or("");
                Ok(Some(BytecodeValue::Bool(s.contains(sub))))
            }
            "sound.stop_all" => {
                world.set_var("__audio_stopped", RuntimeValue::Bool(true));
                Ok(None)
            }
            "sound.set_volume" => {
                let vol = args.first().and_then(|v| as_f64(v)).unwrap_or(100.0);
                world.set_var("__master_volume", RuntimeValue::Number(vol));
                Ok(None)
            }
            "sound.change_volume" => {
                let delta = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                let cur = world.get_var("__master_volume").map(|v| match v { RuntimeValue::Number(n) => *n, _ => 100.0 }).unwrap_or(100.0);
                world.set_var("__master_volume", RuntimeValue::Number((cur + delta).clamp(0.0, 100.0)));
                Ok(None)
            }
            "stop_all" => {
                world.set_var("__game_stopped", RuntimeValue::Bool(true));
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
