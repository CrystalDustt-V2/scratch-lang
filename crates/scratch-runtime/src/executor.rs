use crate::list::ListSystem;
use crate::movement::MovementSystem;
use crate::value::RuntimeValue;
use crate::world::World;
use scratch_blocks::BlockRegistry;
use scratch_ir::{IrBinaryOp, IrExpr, IrInstruction, IrUnaryOp, IrValue};

pub struct Executor;

impl Executor {
    pub fn execute_instructions(
        instructions: &[IrInstruction],
        world: &mut World,
        registry: &BlockRegistry,
    ) {
        for instruction in instructions {
            Self::execute_instruction(instruction, world, registry);
        }
    }

    pub fn execute_instruction(
        instruction: &IrInstruction,
        world: &mut World,
        registry: &BlockRegistry,
    ) {
        match instruction {
            IrInstruction::Assign { target, expr } => {
                let val = Self::eval_expr(expr, world, registry);
                world.set_var(target, val);
            }
            IrInstruction::CallBlock { name, args } => {
                let evaled_args: Vec<RuntimeValue> = args
                    .iter()
                    .map(|a| Self::eval_expr(a, world, registry))
                    .collect();
                Self::call_block(name, &evaled_args, world);
            }
            IrInstruction::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = Self::eval_expr(condition, world, registry);
                if cond_val.as_bool() {
                    Self::execute_instructions(then_branch, world, registry);
                } else {
                    Self::execute_instructions(else_branch, world, registry);
                }
            }
            IrInstruction::Repeat { count, body } => {
                let count_val = Self::eval_expr(count, world, registry);
                let count_int = count_val.as_number().unwrap_or(0.0) as i64;
                for _ in 0..count_int.max(0) {
                    Self::execute_instructions(body, world, registry);
                }
            }
            IrInstruction::Return(_) => {
                // Handled in function context
            }
            IrInstruction::Assert(expr) => {
                let val = Self::eval_expr(expr, world, registry);
                assert!(val.as_bool(), "Assertion failed: {:?}", expr);
            }
        }
    }

    pub fn eval_expr(expr: &IrExpr, world: &World, registry: &BlockRegistry) -> RuntimeValue {
        match expr {
            IrExpr::Constant(val) => match val {
                IrValue::Number(n) => RuntimeValue::Number(*n),
                IrValue::String(s) => RuntimeValue::String(s.clone()),
                IrValue::Bool(b) => RuntimeValue::Bool(*b),
            },
            IrExpr::Var(name) => {
                // Check if it's an entity name
                if world.get_entity_by_name(name).is_some() {
                    RuntimeValue::Object(name.clone())
                } else {
                    world.get_var_or_nil(name)
                }
            }
            IrExpr::Binary { left, op, right } => {
                let l = Self::eval_expr(left, world, registry);
                let r = Self::eval_expr(right, world, registry);
                Self::eval_binary_op(&l, *op, &r)
            }
            IrExpr::Unary { op, expr } => {
                let v = Self::eval_expr(expr, world, registry);
                match op {
                    IrUnaryOp::Not => RuntimeValue::Bool(!v.as_bool()),
                    IrUnaryOp::Neg => {
                        let num = v.as_number().unwrap_or(0.0);
                        RuntimeValue::Number(-num)
                    }
                }
            }
            IrExpr::CallBlock { name, args } => {
                let evaled_args: Vec<RuntimeValue> = args
                    .iter()
                    .map(|a| Self::eval_expr(a, world, registry))
                    .collect();
                Self::eval_block_expr(name, &evaled_args, world)
            }
        }
    }

    fn eval_binary_op(left: &RuntimeValue, op: IrBinaryOp, right: &RuntimeValue) -> RuntimeValue {
        match op {
            IrBinaryOp::Add => {
                if let (Some(l), Some(r)) = (left.as_number(), right.as_number()) {
                    RuntimeValue::Number(l + r)
                } else {
                    RuntimeValue::String(format!("{}{}", left, right))
                }
            }
            IrBinaryOp::Sub => {
                let l = left.as_number().unwrap_or(0.0);
                let r = right.as_number().unwrap_or(0.0);
                RuntimeValue::Number(l - r)
            }
            IrBinaryOp::Mul => {
                let l = left.as_number().unwrap_or(0.0);
                let r = right.as_number().unwrap_or(0.0);
                RuntimeValue::Number(l * r)
            }
            IrBinaryOp::Div => {
                let l = left.as_number().unwrap_or(0.0);
                let r = right.as_number().unwrap_or(1.0);
                if r == 0.0 {
                    RuntimeValue::Number(0.0)
                } else {
                    RuntimeValue::Number(l / r)
                }
            }
            IrBinaryOp::Mod => {
                let l = left.as_number().unwrap_or(0.0);
                let r = right.as_number().unwrap_or(1.0);
                RuntimeValue::Number(l % r)
            }
            IrBinaryOp::Equal => RuntimeValue::Bool(left == right),
            IrBinaryOp::NotEqual => RuntimeValue::Bool(left != right),
            IrBinaryOp::Less => {
                let l = left.as_number().unwrap_or(0.0);
                let r = right.as_number().unwrap_or(0.0);
                RuntimeValue::Bool(l < r)
            }
            IrBinaryOp::LessEqual => {
                let l = left.as_number().unwrap_or(0.0);
                let r = right.as_number().unwrap_or(0.0);
                RuntimeValue::Bool(l <= r)
            }
            IrBinaryOp::Greater => {
                let l = left.as_number().unwrap_or(0.0);
                let r = right.as_number().unwrap_or(0.0);
                RuntimeValue::Bool(l > r)
            }
            IrBinaryOp::GreaterEqual => {
                let l = left.as_number().unwrap_or(0.0);
                let r = right.as_number().unwrap_or(0.0);
                RuntimeValue::Bool(l >= r)
            }
            IrBinaryOp::And => RuntimeValue::Bool(left.as_bool() && right.as_bool()),
            IrBinaryOp::Or => RuntimeValue::Bool(left.as_bool() || right.as_bool()),
        }
    }

    fn call_block(name: &str, args: &[RuntimeValue], world: &mut World) {
        match name {
            "move" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    let dx = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.0) as f32;
                    let dy = args.get(2).and_then(|v| v.as_number()).unwrap_or(0.0) as f32;
                    MovementSystem::execute_move(world, target, dx, dy);
                }
            }
            "jump" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    let force = args.get(1).and_then(|v| v.as_number()).unwrap_or(10.0) as f32;
                    MovementSystem::execute_jump(world, target, force);
                }
            }
            "stop" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    MovementSystem::execute_stop(world, target);
                }
            }
            "teleport" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    let x = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.0) as f32;
                    let y = args.get(2).and_then(|v| v.as_number()).unwrap_or(0.0) as f32;
                    MovementSystem::execute_teleport(world, target, x, y);
                }
            }
            "camera.follow" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    world.camera_follow_target = Some(target.to_string());
                }
            }
            "background.set" => {
                if let Some(bg) = args.first().and_then(|v| v.as_string()) {
                    world.background = bg.to_string();
                }
            }
            "damage" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    let amount = args.get(1).and_then(|v| v.as_number()).unwrap_or(1.0);
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.health -= amount;
                    }
                }
            }
            "heal" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    let amount = args.get(1).and_then(|v| v.as_number()).unwrap_or(1.0);
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.health += amount;
                    }
                }
            }
            "respawn" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.transform.x = 0.0;
                        ent.transform.y = 0.0;
                        ent.velocity = (0.0, 0.0);
                        ent.health = 3.0;
                    }
                }
            }
            "collect" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    if let Some(ent) = world.get_entity_by_name_mut(target) {
                        ent.visible = false;
                    }
                }
            }
            "glide" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    let secs = args.get(1).and_then(|v| v.as_number()).unwrap_or(1.0) as f32;
                    let x = args.get(2).and_then(|v| v.as_number()).unwrap_or(0.0) as f32;
                    let y = args.get(3).and_then(|v| v.as_number()).unwrap_or(0.0) as f32;
                    MovementSystem::start_glide(world, target, secs, x, y);
                }
            }
            "go_to" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    let dest = args.get(1).and_then(|v| v.as_string()).unwrap_or("mouse");
                    MovementSystem::execute_go_to(world, target, dest);
                }
            }
            "set_rotation_style" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    let style = args.get(1).and_then(|v| v.as_string()).unwrap_or("all-around");
                    MovementSystem::execute_set_rotation_style(world, target, style);
                }
            }
            "list.add" | "add_to_list" => {
                if let Some(list_name) = args.first().and_then(|v| v.as_string()) {
                    let item = args.get(1).cloned().unwrap_or(RuntimeValue::Nil);
                    ListSystem::add(world, list_name, item);
                }
            }
            "list.delete" | "delete_of_list" => {
                if let Some(list_name) = args.first().and_then(|v| v.as_string()) {
                    let idx = args.get(1).unwrap_or(&RuntimeValue::Number(1.0));
                    ListSystem::delete(world, list_name, idx);
                }
            }
            "list.insert" | "insert_at_list" => {
                if let Some(list_name) = args.first().and_then(|v| v.as_string()) {
                    let idx = args.get(1).unwrap_or(&RuntimeValue::Number(1.0));
                    let item = args.get(2).cloned().unwrap_or(RuntimeValue::Nil);
                    ListSystem::insert(world, list_name, idx, item);
                }
            }
            "list.replace" | "replace_item_of_list" => {
                if let Some(list_name) = args.first().and_then(|v| v.as_string()) {
                    let idx = args.get(1).unwrap_or(&RuntimeValue::Number(1.0));
                    let item = args.get(2).cloned().unwrap_or(RuntimeValue::Nil);
                    ListSystem::replace(world, list_name, idx, item);
                }
            }
            "list.clear" => {
                if let Some(list_name) = args.first().and_then(|v| v.as_string()) {
                    ListSystem::clear(world, list_name);
                }
            }
            "list.show" => {
                if let Some(list_name) = args.first().and_then(|v| v.as_string()) {
                    ListSystem::show(world, list_name);
                }
            }
            "list.hide" => {
                if let Some(list_name) = args.first().and_then(|v| v.as_string()) {
                    ListSystem::hide(world, list_name);
                }
            }
            _ => {
                // Other runtime blocks like sound.play
            }
        }
    }

    fn eval_block_expr(name: &str, args: &[RuntimeValue], world: &World) -> RuntimeValue {
        match name {
            "touching" => {
                let target_a = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let target_b = args.get(1).and_then(|v| v.as_string()).unwrap_or("");

                if let (Some(a), Some(b)) = (
                    world.get_entity_by_name(target_a),
                    world.get_entity_by_name(target_b),
                ) {
                    // AABB collision check
                    let half_w_a = a.size[0] / 2.0;
                    let half_h_a = a.size[1] / 2.0;
                    let half_w_b = b.size[0] / 2.0;
                    let half_h_b = b.size[1] / 2.0;

                    let overlap_x = (a.transform.x - b.transform.x).abs() <= (half_w_a + half_w_b);
                    let overlap_y = (a.transform.y - b.transform.y).abs() <= (half_h_a + half_h_b);

                    RuntimeValue::Bool(overlap_x && overlap_y && a.visible && b.visible)
                } else {
                    RuntimeValue::Bool(false)
                }
            }
            "list.item" | "item_of_list" => {
                let list_name = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let idx = args.get(1).unwrap_or(&RuntimeValue::Number(1.0));
                ListSystem::item(world, list_name, idx)
            }
            "list.length" | "length_of_list" => {
                let list_name = args.first().and_then(|v| v.as_string()).unwrap_or("");
                RuntimeValue::Number(ListSystem::length(world, list_name))
            }
            "list.contains" | "list_contains" => {
                let list_name = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let item = args.get(1).unwrap_or(&RuntimeValue::Nil);
                RuntimeValue::Bool(ListSystem::contains(world, list_name, item))
            }
            _ => RuntimeValue::Nil,
        }
    }
}
