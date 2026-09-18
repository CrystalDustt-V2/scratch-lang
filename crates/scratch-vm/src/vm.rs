use scratch_blocks::BlockRegistry;
use scratch_bytecode::{BytecodeValue, Chunk, OpCode};
use scratch_runtime::{ListSystem, MovementSystem, RuntimeValue, SensingSystem, World};
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
            "move_up" | "up" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let steps = args.get(1).and_then(|v| as_f64(v)).unwrap_or(5.0) as f32;
                    MovementSystem::execute_move_up(world, target, steps);
                }
                Ok(None)
            }
            "move_down" | "down" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let steps = args.get(1).and_then(|v| as_f64(v)).unwrap_or(5.0) as f32;
                    MovementSystem::execute_move_down(world, target, steps);
                }
                Ok(None)
            }
            "move_left" | "left" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let steps = args.get(1).and_then(|v| as_f64(v)).unwrap_or(5.0) as f32;
                    MovementSystem::execute_move_left(world, target, steps);
                }
                Ok(None)
            }
            "move_right" | "right" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let steps = args.get(1).and_then(|v| as_f64(v)).unwrap_or(5.0) as f32;
                    MovementSystem::execute_move_right(world, target, steps);
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
            "glide" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let secs = args.get(1).and_then(|v| as_f64(v)).unwrap_or(1.0) as f32;
                    let x = args.get(2).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    let y = args.get(3).and_then(|v| as_f64(v)).unwrap_or(0.0) as f32;
                    MovementSystem::start_glide(world, target, secs, x, y);
                }
                Ok(None)
            }
            "go_to" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let dest = args.get(1).and_then(|v| as_str(v)).unwrap_or("mouse");
                    MovementSystem::execute_go_to(world, target, dest);
                }
                Ok(None)
            }
            "set_rotation_style" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let style = args.get(1).and_then(|v| as_str(v)).unwrap_or("all-around");
                    MovementSystem::execute_set_rotation_style(world, target, style);
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
                        Some(BytecodeValue::List(l)) => format!("[{}]", l.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(", ")),
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
            "math.tan" => {
                let deg = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(deg.to_radians().tan())))
            }
            "math.asin" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(n.clamp(-1.0, 1.0).asin().to_degrees())))
            }
            "math.acos" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(n.clamp(-1.0, 1.0).acos().to_degrees())))
            }
            "math.atan" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(n.atan().to_degrees())))
            }
            "math.ln" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(1.0);
                Ok(Some(BytecodeValue::Number(if n > 0.0 { n.ln() } else { 0.0 })))
            }
            "math.log" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(1.0);
                Ok(Some(BytecodeValue::Number(if n > 0.0 { n.log10() } else { 0.0 })))
            }
            "math.exp" => {
                let n = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(n.exp())))
            }
            "math.pow" => {
                let base = args.first().and_then(|v| as_f64(v)).unwrap_or(1.0);
                let exp = args.get(1).and_then(|v| as_f64(v)).unwrap_or(1.0);
                Ok(Some(BytecodeValue::Number(base.powf(exp))))
            }
            "math.mod" => {
                let a = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                let b = args.get(1).and_then(|v| as_f64(v)).unwrap_or(1.0);
                let res = if b != 0.0 { ((a % b) + b) % b } else { 0.0 };
                Ok(Some(BytecodeValue::Number(res)))
            }
            "math.min" => {
                let a = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                let b = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(a.min(b))))
            }
            "math.max" => {
                let a = args.first().and_then(|v| as_f64(v)).unwrap_or(0.0);
                let b = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(a.max(b))))
            }
            "text.letter_at" => {
                let s = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let idx = args.get(1).and_then(|v| as_f64(v)).unwrap_or(1.0) as usize;
                let ch = if idx >= 1 {
                    s.chars().nth(idx - 1).map(|c| c.to_string()).unwrap_or_default()
                } else {
                    String::new()
                };
                Ok(Some(BytecodeValue::String(ch)))
            }
            "text.upper" => {
                let s = args.first().and_then(|v| as_str(v)).unwrap_or("");
                Ok(Some(BytecodeValue::String(s.to_uppercase())))
            }
            "text.lower" => {
                let s = args.first().and_then(|v| as_str(v)).unwrap_or("");
                Ok(Some(BytecodeValue::String(s.to_lowercase())))
            }
            "get_direction" => {
                let rot = args
                    .first()
                    .and_then(|v| as_str(v))
                    .and_then(|name| world.get_entity_by_name(name))
                    .map(|ent| ent.transform.rotation as f64)
                    .unwrap_or(0.0);
                Ok(Some(BytecodeValue::Number(rot)))
            }
            "get_size" => {
                let size = args
                    .first()
                    .and_then(|v| as_str(v))
                    .and_then(|name| world.get_entity_by_name(name))
                    .map(|ent| (ent.transform.scale_x * 100.0).round() as f64)
                    .unwrap_or(100.0);
                Ok(Some(BytecodeValue::Number(size)))
            }
            "get_costume_number" => {
                let num = args
                    .first()
                    .and_then(|v| as_str(v))
                    .and_then(|name| world.get_entity_by_name(name))
                    .map(|_| 1.0)
                    .unwrap_or(1.0);
                Ok(Some(BytecodeValue::Number(num)))
            }
            "get_backdrop_name" => {
                Ok(Some(BytecodeValue::String(world.background.clone())))
            }
            "think_for" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let msg = args.get(1).and_then(|v| as_str(v)).unwrap_or("");
                    world.set_var(&format!("__{}_thought", target), RuntimeValue::String(msg.to_string()));
                }
                Ok(None)
            }
            "change_effect" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let effect = args.get(1).and_then(|v| as_str(v)).unwrap_or("color");
                    let delta = args.get(2).and_then(|v| as_f64(v)).unwrap_or(25.0);
                    let var_name = format!("__{}_effect_{}", target, effect);
                    let cur = world.get_var(&var_name).map(|v| match v { RuntimeValue::Number(n) => *n, _ => 0.0 }).unwrap_or(0.0);
                    world.set_var(&var_name, RuntimeValue::Number(cur + delta));
                }
                Ok(None)
            }
            "sound.get_volume" => {
                let cur = world.get_var("__master_volume").map(|v| match v { RuntimeValue::Number(n) => *n, _ => 100.0 }).unwrap_or(100.0);
                Ok(Some(BytecodeValue::Number(cur)))
            }
            "music.get_tempo" => {
                let bpm = world.get_var("__music_tempo").map(|v| match v { RuntimeValue::Number(n) => *n, _ => 60.0 }).unwrap_or(60.0);
                Ok(Some(BytecodeValue::Number(bpm)))
            }
            "music.change_tempo" => {
                let delta = args.first().and_then(|v| as_f64(v)).unwrap_or(20.0);
                let cur = world.get_var("__music_tempo").map(|v| match v { RuntimeValue::Number(n) => *n, _ => 60.0 }).unwrap_or(60.0);
                world.set_var("__music_tempo", RuntimeValue::Number((cur + delta).max(20.0)));
                Ok(None)
            }
            "music.set_instrument" => {
                let inst = args.first().and_then(|v| as_f64(v)).unwrap_or(1.0);
                world.set_var("__music_instrument", RuntimeValue::Number(inst));
                Ok(None)
            }
            "music.play_drum" => {
                let drum = args.first().and_then(|v| as_f64(v)).unwrap_or(1.0);
                let beats = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.25);
                world.set_var("__last_drum", RuntimeValue::Number(drum));
                world.set_var("__last_drum_beats", RuntimeValue::Number(beats));
                Ok(None)
            }
            "music.rest" => {
                Ok(None)
            }
            "pen.change_size" => {
                let delta = args.first().and_then(|v| as_f64(v)).unwrap_or(1.0);
                let cur = world.get_var("__pen_size").map(|v| match v { RuntimeValue::Number(n) => *n, _ => 1.0 }).unwrap_or(1.0);
                world.set_var("__pen_size", RuntimeValue::Number((cur + delta).max(1.0)));
                Ok(None)
            }
            "pen.change_color" => {
                let delta = args.first().and_then(|v| as_f64(v)).unwrap_or(10.0);
                let cur = world.get_var("__pen_color_hue").map(|v| match v { RuntimeValue::Number(n) => *n, _ => 0.0 }).unwrap_or(0.0);
                world.set_var("__pen_color_hue", RuntimeValue::Number((cur + delta) % 200.0));
                Ok(None)
            }
            "pen.set_shade" => {
                let shade = args.first().and_then(|v| as_f64(v)).unwrap_or(50.0);
                world.set_var("__pen_shade", RuntimeValue::Number(shade.clamp(0.0, 100.0)));
                Ok(None)
            }
            "pen.change_shade" => {
                let delta = args.first().and_then(|v| as_f64(v)).unwrap_or(10.0);
                let cur = world.get_var("__pen_shade").map(|v| match v { RuntimeValue::Number(n) => *n, _ => 50.0 }).unwrap_or(50.0);
                world.set_var("__pen_shade", RuntimeValue::Number((cur + delta).clamp(0.0, 100.0)));
                Ok(None)
            }
            "current_time" => {
                let unit = args.first().and_then(|v| as_str(v)).unwrap_or("second");
                let dur = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                let total_secs = dur.as_secs();
                let val = match unit.to_lowercase().as_str() {
                    "second" | "seconds" => (total_secs % 60) as f64,
                    "minute" | "minutes" => ((total_secs / 60) % 60) as f64,
                    "hour" | "hours" => ((total_secs / 3600) % 24) as f64,
                    "day_of_week" => (((total_secs / 86400) + 4) % 7 + 1) as f64,
                    "days" | "date" => {
                        let days_since_1970 = total_secs / 86400;
                        ((days_since_1970 % 30) + 1) as f64
                    }
                    "month" => {
                        let days_since_1970 = total_secs / 86400;
                        (((days_since_1970 / 30) % 12) + 1) as f64
                    }
                    "year" => {
                        let years = 1970 + total_secs / (86400 * 365);
                        years as f64
                    }
                    _ => total_secs as f64,
                };
                Ok(Some(BytecodeValue::Number(val)))
            }
            "days_since_2000" => {
                let dur = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                let secs = dur.as_secs_f64();
                let days = (secs - 946684800.0) / 86400.0;
                Ok(Some(BytecodeValue::Number(days)))
            }
            "get_username" => {
                let name = std::env::var("USERNAME")
                    .or_else(|_| std::env::var("USER"))
                    .unwrap_or_else(|_| "player".to_string());
                Ok(Some(BytecodeValue::String(name)))
            }
            "broadcast" | "broadcast_and_wait" => {
                if let Some(msg) = args.first().and_then(|v| as_str(v)) {
                    world.broadcast(msg);
                }
                Ok(None)
            }
            "stop_all" => {
                world.set_var("__game_stopped", RuntimeValue::Bool(true));
                Ok(None)
            }
            "list.add" | "add_to_list" => {
                if let Some(list_name) = args.first().and_then(|v| as_str(v)) {
                    let item = args.get(1).map(bytecode_to_runtime).unwrap_or(RuntimeValue::Nil);
                    ListSystem::add(world, list_name, item);
                }
                Ok(None)
            }
            "list.delete" | "delete_of_list" => {
                if let Some(list_name) = args.first().and_then(|v| as_str(v)) {
                    let idx_val = args.get(1).map(bytecode_to_runtime).unwrap_or(RuntimeValue::Number(1.0));
                    ListSystem::delete(world, list_name, &idx_val);
                }
                Ok(None)
            }
            "list.insert" | "insert_at_list" => {
                if let Some(list_name) = args.first().and_then(|v| as_str(v)) {
                    let idx_val = args.get(1).map(bytecode_to_runtime).unwrap_or(RuntimeValue::Number(1.0));
                    let item = args.get(2).map(bytecode_to_runtime).unwrap_or(RuntimeValue::Nil);
                    ListSystem::insert(world, list_name, &idx_val, item);
                }
                Ok(None)
            }
            "list.replace" | "replace_item_of_list" => {
                if let Some(list_name) = args.first().and_then(|v| as_str(v)) {
                    let idx_val = args.get(1).map(bytecode_to_runtime).unwrap_or(RuntimeValue::Number(1.0));
                    let item = args.get(2).map(bytecode_to_runtime).unwrap_or(RuntimeValue::Nil);
                    ListSystem::replace(world, list_name, &idx_val, item);
                }
                Ok(None)
            }
            "list.item" | "item_of_list" => {
                let list_name = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let idx_val = args.get(1).map(bytecode_to_runtime).unwrap_or(RuntimeValue::Number(1.0));
                let res = ListSystem::item(world, list_name, &idx_val);
                Ok(Some(runtime_to_bytecode(&res)))
            }
            "list.length" | "length_of_list" => {
                let list_name = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let len = ListSystem::length(world, list_name);
                Ok(Some(BytecodeValue::Number(len)))
            }
            "list.contains" | "list_contains" => {
                let list_name = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let item = args.get(1).map(bytecode_to_runtime).unwrap_or(RuntimeValue::Nil);
                let found = ListSystem::contains(world, list_name, &item);
                Ok(Some(BytecodeValue::Bool(found)))
            }
            "list.clear" => {
                if let Some(list_name) = args.first().and_then(|v| as_str(v)) {
                    ListSystem::clear(world, list_name);
                }
                Ok(None)
            }
            "list.show" => {
                if let Some(list_name) = args.first().and_then(|v| as_str(v)) {
                    ListSystem::show(world, list_name);
                }
                Ok(None)
            }
            "list.hide" => {
                if let Some(list_name) = args.first().and_then(|v| as_str(v)) {
                    ListSystem::hide(world, list_name);
                }
                Ok(None)
            }
            "ask" => {
                if let Some(q) = args.first().and_then(|v| as_str(v)) {
                    world.ask(q);
                }
                Ok(None)
            }
            "get_answer" | "answer" => {
                Ok(Some(BytecodeValue::String(world.get_answer().to_string())))
            }
            "variable.show" | "show_variable" => {
                if let Some(name) = args.first().and_then(|v| as_str(v)) {
                    world.show_variable(name);
                }
                Ok(None)
            }
            "variable.hide" | "hide_variable" => {
                if let Some(name) = args.first().and_then(|v| as_str(v)) {
                    world.hide_variable(name);
                }
                Ok(None)
            }
            "touching_color" => {
                let target = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let color = args.get(1).and_then(|v| as_str(v)).unwrap_or("");
                let res = SensingSystem::touching_color(world, target, color);
                Ok(Some(BytecodeValue::Bool(res)))
            }
            "color_touching_color" => {
                let c1 = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let c2 = args.get(1).and_then(|v| as_str(v)).unwrap_or("");
                let res = SensingSystem::color_touching_color(world, c1, c2);
                Ok(Some(BytecodeValue::Bool(res)))
            }
            "get_loudness" | "loudness" => {
                let l = SensingSystem::get_loudness(world);
                Ok(Some(BytecodeValue::Number(l)))
            }
            "go_to_front" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    SensingSystem::go_to_front(world, target);
                }
                Ok(None)
            }
            "go_back_layers" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    let count = args.get(1).and_then(|v| as_f64(v)).unwrap_or(1.0) as i32;
                    SensingSystem::go_back_layers(world, target, count);
                }
                Ok(None)
            }
            "property_of" => {
                let target = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let prop = args.get(1).and_then(|v| as_str(v)).unwrap_or("");
                let val = SensingSystem::property_of(world, target, prop);
                Ok(Some(runtime_to_bytecode(&val)))
            }
            "switch_backdrop" | "switch_backdrop_and_wait" => {
                if let Some(bg) = args.first().and_then(|v| as_str(v)) {
                    world.switch_backdrop(bg);
                    world.broadcast(format!("__scene_switched:{}", bg));
                }
                Ok(None)
            }
            "next_backdrop" => {
                world.next_backdrop();
                let cur = world.get_backdrop_name().to_string();
                world.broadcast(format!("__scene_switched:{}", cur));
                Ok(None)
            }
            "get_backdrop_number" => {
                Ok(Some(BytecodeValue::Number(world.get_backdrop_number() as f64)))
            }
            "get_costume_name" => {
                let target = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let name = world.get_entity_by_name(target).map(|e| e.costume_name.clone()).unwrap_or_else(|| "costume1".to_string());
                Ok(Some(BytecodeValue::String(name)))
            }
            "set_drag_mode" => {
                if let (Some(target), Some(mode)) = (
                    args.first().and_then(|v| as_str(v)),
                    args.get(1).and_then(|v| as_str(v)),
                ) {
                    SensingSystem::set_drag_mode(world, target, mode);
                }
                Ok(None)
            }
            "sound.set_effect" => {
                if let (Some(effect), Some(val)) = (
                    args.first().and_then(|v| as_str(v)),
                    args.get(1).and_then(|v| as_f64(v)),
                ) {
                    world.sound_set_effect(effect, val as f32);
                }
                Ok(None)
            }
            "sound.change_effect" => {
                if let (Some(effect), Some(delta)) = (
                    args.first().and_then(|v| as_str(v)),
                    args.get(1).and_then(|v| as_f64(v)),
                ) {
                    world.sound_change_effect(effect, delta as f32);
                }
                Ok(None)
            }
            "sound.clear_effects" => {
                world.sound_clear_effects();
                Ok(None)
            }
            "pen.clear" | "pen.erase_all" => {
                world.pen_clear();
                Ok(None)
            }
            "pen.stamp" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    world.pen_stamp(target);
                }
                Ok(None)
            }
            "pen.down" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    world.pen_down(target);
                }
                Ok(None)
            }
            "pen.up" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    world.pen_up(target);
                }
                Ok(None)
            }
            "pen.set_color" => {
                if let Some(color_val) = args.first() {
                    let c = match color_val {
                        BytecodeValue::String(s) if s.starts_with('#') && s.len() == 7 => {
                            let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0) as f32 / 255.0;
                            let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0) as f32 / 255.0;
                            let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0) as f32 / 255.0;
                            [r, g, b, 1.0]
                        }
                        BytecodeValue::String(s) => match s.to_lowercase().as_str() {
                            "red" => [1.0, 0.0, 0.0, 1.0],
                            "green" => [0.0, 1.0, 0.0, 1.0],
                            "blue" => [0.0, 0.0, 1.0, 1.0],
                            "black" => [0.0, 0.0, 0.0, 1.0],
                            "white" => [1.0, 1.0, 1.0, 1.0],
                            "yellow" => [1.0, 1.0, 0.0, 1.0],
                            _ => [0.0, 0.5, 1.0, 1.0],
                        },
                        _ => [0.0, 0.0, 1.0, 1.0],
                    };
                    world.pen_set_color(c);
                }
                Ok(None)
            }
            "pen.set_size" => {
                if let Some(sz) = args.first().and_then(|v| as_f64(v)) {
                    world.pen_set_size(sz as f32);
                }
                Ok(None)
            }
            "pen.set_param" => {
                if let (Some(param), Some(val)) = (
                    args.first().and_then(|v| as_str(v)),
                    args.get(1).and_then(|v| as_f64(v)),
                ) {
                    world.pen_set_param(param, val as f32);
                }
                Ok(None)
            }
            "pen.change_param" => {
                if let (Some(param), Some(delta)) = (
                    args.first().and_then(|v| as_str(v)),
                    args.get(1).and_then(|v| as_f64(v)),
                ) {
                    world.pen_change_param(param, delta as f32);
                }
                Ok(None)
            }
            "music.play_note" => {
                let note = args.first().and_then(|v| as_f64(v)).unwrap_or(60.0) as f32;
                let beats = args.get(1).and_then(|v| as_f64(v)).unwrap_or(0.5) as f32;
                world.music_play_note(note, beats);
                Ok(None)
            }
            "music.set_tempo" => {
                if let Some(bpm) = args.first().and_then(|v| as_f64(v)) {
                    world.music_set_tempo(bpm as f32);
                }
                Ok(None)
            }
            "current" => {
                let unit = args.first().and_then(|v| as_str(v)).unwrap_or("second");
                Ok(Some(BytecodeValue::Number(SensingSystem::current(unit))))
            }
            "list.index_of" | "item_num_of_list" => {
                let list_name = args.first().and_then(|v| as_str(v)).unwrap_or("");
                let item = args.get(1).map(bytecode_to_runtime).unwrap_or(RuntimeValue::Nil);
                Ok(Some(BytecodeValue::Number(ListSystem::index_of(world, list_name, &item))))
            }
            "list.to_string" => {
                let list_name = args.first().and_then(|v| as_str(v)).unwrap_or("");
                Ok(Some(BytecodeValue::String(ListSystem::to_string(world, list_name))))
            }
            "tts.speak" => {
                if let Some(txt) = args.first().and_then(|v| as_str(v)) {
                    world.tts_speak(txt);
                }
                Ok(None)
            }
            "tts.set_voice" => {
                if let Some(voice) = args.first().and_then(|v| as_str(v)) {
                    world.tts_set_voice(voice);
                }
                Ok(None)
            }
            "tts.set_language" => {
                if let Some(lang) = args.first().and_then(|v| as_str(v)) {
                    world.tts_set_language(lang);
                }
                Ok(None)
            }
            "translate.text" => {
                let s = args.first().map(|v| match v { BytecodeValue::String(str) => str.clone(), _ => v.to_string() }).unwrap_or_default();
                Ok(Some(BytecodeValue::String(s)))
            }
            "translate.get_language" => {
                Ok(Some(BytecodeValue::String("en".to_string())))
            }
            "stop_other_scripts" => {
                if let Some(target) = args.first().and_then(|v| as_str(v)) {
                    world.active_tweens.retain(|t| t.target != target);
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
        BytecodeValue::List(l) => !l.is_empty(),
    }
}

fn runtime_to_bytecode(rv: &RuntimeValue) -> BytecodeValue {
    match rv {
        RuntimeValue::Nil => BytecodeValue::Nil,
        RuntimeValue::Number(n) => BytecodeValue::Number(*n),
        RuntimeValue::String(s) => BytecodeValue::String(s.clone()),
        RuntimeValue::Bool(b) => BytecodeValue::Bool(*b),
        RuntimeValue::Object(o) => BytecodeValue::Object(o.clone()),
        RuntimeValue::List(l) => BytecodeValue::List(l.iter().map(runtime_to_bytecode).collect()),
    }
}

fn bytecode_to_runtime(bv: &BytecodeValue) -> RuntimeValue {
    match bv {
        BytecodeValue::Nil => RuntimeValue::Nil,
        BytecodeValue::Number(n) => RuntimeValue::Number(*n),
        BytecodeValue::String(s) => RuntimeValue::String(s.clone()),
        BytecodeValue::Bool(b) => RuntimeValue::Bool(*b),
        BytecodeValue::Object(o) => RuntimeValue::Object(o.clone()),
        BytecodeValue::List(l) => RuntimeValue::List(l.iter().map(bytecode_to_runtime).collect()),
    }
}
