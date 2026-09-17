use crate::list::ListSystem;
use crate::movement::MovementSystem;
use crate::sensing::SensingSystem;
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
            "ask" => {
                if let Some(q) = args.first().and_then(|v| v.as_string()) {
                    world.ask(q);
                }
            }
            "variable.show" | "show_variable" => {
                if let Some(name) = args.first().and_then(|v| v.as_string()) {
                    world.show_variable(name);
                }
            }
            "variable.hide" | "hide_variable" => {
                if let Some(name) = args.first().and_then(|v| v.as_string()) {
                    world.hide_variable(name);
                }
            }
            "go_to_front" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    SensingSystem::go_to_front(world, target);
                }
            }
            "go_back_layers" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    let count = args.get(1).and_then(|v| v.as_number()).unwrap_or(1.0) as i32;
                    SensingSystem::go_back_layers(world, target, count);
                }
            }
            "say" => {
                if let (Some(target), Some(text)) = (args.first().and_then(|v| v.as_string()), args.get(1).and_then(|v| v.as_string())) {
                    world.set_var(format!("__say_{}", target), RuntimeValue::String(text.to_string()));
                }
            }
            "say_for" => {
                if let (Some(target), Some(text)) = (args.first().and_then(|v| v.as_string()), args.get(1).and_then(|v| v.as_string())) {
                    world.set_var(format!("__say_{}", target), RuntimeValue::String(text.to_string()));
                }
            }
            "think" | "think_for" => {
                if let (Some(target), Some(text)) = (args.first().and_then(|v| v.as_string()), args.get(1).and_then(|v| v.as_string())) {
                    world.set_var(format!("__think_{}", target), RuntimeValue::String(text.to_string()));
                }
            }
            "sound.play" | "sound.play_until_done" => {
                if let Some(snd) = args.first().and_then(|v| v.as_string()) {
                    world.broadcast(format!("__sound:{}", snd));
                }
            }
            "broadcast" | "broadcast_and_wait" => {
                if let Some(msg) = args.first().and_then(|v| v.as_string()) {
                    world.broadcast(msg);
                }
            }
            "scene.switch" => {
                if let Some(scene) = args.first().and_then(|v| v.as_string()) {
                    world.broadcast(format!("__scene_switch:{}", scene));
                }
            }
            "switch_backdrop" | "switch_backdrop_and_wait" => {
                if let Some(bg) = args.first().and_then(|v| v.as_string()) {
                    world.switch_backdrop(bg);
                    world.broadcast(format!("__scene_switched:{}", bg));
                }
            }
            "next_backdrop" => {
                world.next_backdrop();
                let cur = world.get_backdrop_name().to_string();
                world.broadcast(format!("__scene_switched:{}", cur));
            }
            "set_drag_mode" => {
                if let (Some(target), Some(mode)) = (
                    args.first().and_then(|v| v.as_string()),
                    args.get(1).and_then(|v| v.as_string()),
                ) {
                    SensingSystem::set_drag_mode(world, target, mode);
                }
            }
            "sound.set_effect" => {
                if let (Some(effect), Some(val)) = (
                    args.first().and_then(|v| v.as_string()),
                    args.get(1).and_then(|v| v.as_number()),
                ) {
                    world.sound_set_effect(effect, val as f32);
                }
            }
            "sound.change_effect" => {
                if let (Some(effect), Some(delta)) = (
                    args.first().and_then(|v| v.as_string()),
                    args.get(1).and_then(|v| v.as_number()),
                ) {
                    world.sound_change_effect(effect, delta as f32);
                }
            }
            "sound.clear_effects" => {
                world.sound_clear_effects();
            }
            "pen.clear" | "pen.erase_all" => {
                world.pen_clear();
            }
            "pen.stamp" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    world.pen_stamp(target);
                }
            }
            "pen.down" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    world.pen_down(target);
                }
            }
            "pen.up" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    world.pen_up(target);
                }
            }
            "pen.set_color" => {
                if let Some(color_val) = args.first() {
                    let c = match color_val {
                        RuntimeValue::String(s) if s.starts_with('#') && s.len() == 7 => {
                            let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0) as f32 / 255.0;
                            let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0) as f32 / 255.0;
                            let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0) as f32 / 255.0;
                            [r, g, b, 1.0]
                        }
                        RuntimeValue::String(s) => match s.to_lowercase().as_str() {
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
            }
            "pen.set_size" => {
                if let Some(sz) = args.first().and_then(|v| v.as_number()) {
                    world.pen_set_size(sz as f32);
                }
            }
            "pen.change_size" => {
                if let Some(delta) = args.first().and_then(|v| v.as_number()) {
                    world.pen_change_size(delta as f32);
                }
            }
            "pen.set_param" => {
                if let (Some(param), Some(val)) = (
                    args.first().and_then(|v| v.as_string()),
                    args.get(1).and_then(|v| v.as_number()),
                ) {
                    world.pen_set_param(param, val as f32);
                }
            }
            "pen.change_param" => {
                if let (Some(param), Some(delta)) = (
                    args.first().and_then(|v| v.as_string()),
                    args.get(1).and_then(|v| v.as_number()),
                ) {
                    world.pen_change_param(param, delta as f32);
                }
            }
            "music.play_note" => {
                let note = args.first().and_then(|v| v.as_number()).unwrap_or(60.0) as f32;
                let beats = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.5) as f32;
                world.music_play_note(note, beats);
            }
            "music.play_drum" => {
                let drum = args.first().and_then(|v| v.as_number()).unwrap_or(1.0) as f32;
                let beats = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.25) as f32;
                world.music_play_drum(drum, beats);
            }
            "music.rest" => {
                // Audio rest pause
            }
            "music.set_instrument" => {
                if let Some(inst) = args.first().and_then(|v| v.as_number()) {
                    world.music_set_instrument(inst as i32);
                }
            }
            "music.set_tempo" => {
                if let Some(bpm) = args.first().and_then(|v| v.as_number()) {
                    world.music_set_tempo(bpm as f32);
                }
            }
            "music.change_tempo" => {
                if let Some(delta) = args.first().and_then(|v| v.as_number()) {
                    world.music_change_tempo(delta as f32);
                }
            }
            "tts.speak" => {
                if let Some(txt) = args.first().and_then(|v| v.as_string()) {
                    world.tts_speak(txt);
                }
            }
            "tts.set_voice" => {
                if let Some(voice) = args.first().and_then(|v| v.as_string()) {
                    world.tts_set_voice(voice);
                }
            }
            "tts.set_language" => {
                if let Some(lang) = args.first().and_then(|v| v.as_string()) {
                    world.tts_set_language(lang);
                }
            }
            "stop_other_scripts" => {
                if let Some(target) = args.first().and_then(|v| v.as_string()) {
                    world.active_tweens.retain(|t| t.target != target);
                }
            }
            _ => {
                // Other runtime blocks
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
            "get_answer" | "answer" => {
                RuntimeValue::String(world.get_answer().to_string())
            }
            "touching_color" => {
                let target = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let color = args.get(1).and_then(|v| v.as_string()).unwrap_or("");
                RuntimeValue::Bool(SensingSystem::touching_color(world, target, color))
            }
            "color_touching_color" => {
                let c1 = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let c2 = args.get(1).and_then(|v| v.as_string()).unwrap_or("");
                RuntimeValue::Bool(SensingSystem::color_touching_color(world, c1, c2))
            }
            "get_loudness" | "loudness" => {
                RuntimeValue::Number(SensingSystem::get_loudness(world))
            }
            "property_of" => {
                let target = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let prop = args.get(1).and_then(|v| v.as_string()).unwrap_or("");
                SensingSystem::property_of(world, target, prop)
            }
            "mouse_x" => RuntimeValue::Number(world.mouse_pos.0 as f64),
            "mouse_y" => RuntimeValue::Number(world.mouse_pos.1 as f64),
            "mouse_down" => RuntimeValue::Bool(world.mouse_down),
            "key_pressed" => {
                let act = args.first().and_then(|v| v.as_string()).unwrap_or("");
                RuntimeValue::Bool(world.input_actions_down.contains(act))
            }
            "get_timer" => {
                let t = world.get_var("__runtime_timer").and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(t)
            }
            "random" => {
                let min = args.first().and_then(|v| v.as_number()).unwrap_or(1.0);
                let max = args.get(1).and_then(|v| v.as_number()).unwrap_or(10.0);
                let seed = world.get_var("__random_seed").and_then(|v| v.as_number()).unwrap_or(12345.0);
                let next_seed = (seed * 1103515245.0 + 12345.0) % 2147483648.0;
                let ratio = next_seed / 2147483648.0;
                let val = min + ratio * (max - min);
                RuntimeValue::Number(val.round())
            }
            "math.round" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(n.round())
            }
            "math.abs" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(n.abs())
            }
            "math.sqrt" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(if n < 0.0 { 0.0 } else { n.sqrt() })
            }
            "math.sin" => {
                let deg = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(deg.to_radians().sin())
            }
            "math.cos" => {
                let deg = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(deg.to_radians().cos())
            }
            "math.floor" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(n.floor())
            }
            "math.ceil" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(n.ceil())
            }
            "math.tan" => {
                let deg = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(deg.to_radians().tan())
            }
            "math.asin" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(n.clamp(-1.0, 1.0).asin().to_degrees())
            }
            "math.acos" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(n.clamp(-1.0, 1.0).acos().to_degrees())
            }
            "math.atan" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(n.atan().to_degrees())
            }
            "math.ln" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(1.0);
                RuntimeValue::Number(if n > 0.0 { n.ln() } else { 0.0 })
            }
            "math.log" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(1.0);
                RuntimeValue::Number(if n > 0.0 { n.log10() } else { 0.0 })
            }
            "math.exp" => {
                let n = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(n.exp())
            }
            "math.pow" => {
                let base = args.first().and_then(|v| v.as_number()).unwrap_or(1.0);
                let exp = args.get(1).and_then(|v| v.as_number()).unwrap_or(1.0);
                RuntimeValue::Number(base.powf(exp))
            }
            "math.mod" => {
                let a = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                let b = args.get(1).and_then(|v| v.as_number()).unwrap_or(1.0);
                RuntimeValue::Number(if b == 0.0 { 0.0 } else { a % b })
            }
            "math.min" => {
                let a = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                let b = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(a.min(b))
            }
            "math.max" => {
                let a = args.first().and_then(|v| v.as_number()).unwrap_or(0.0);
                let b = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.0);
                RuntimeValue::Number(a.max(b))
            }
            "text.join" => {
                let a = args.first().map(|v| v.to_string()).unwrap_or_default();
                let b = args.get(1).map(|v| v.to_string()).unwrap_or_default();
                RuntimeValue::String(format!("{}{}", a, b))
            }
            "text.length" => {
                let s = args.first().and_then(|v| v.as_string()).unwrap_or("");
                RuntimeValue::Number(s.chars().count() as f64)
            }
            "text.contains" => {
                let s = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let sub = args.get(1).and_then(|v| v.as_string()).unwrap_or("");
                RuntimeValue::Bool(s.contains(sub))
            }
            "text.letter_at" => {
                let s = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let idx = args.get(1).and_then(|v| v.as_number()).unwrap_or(1.0) as usize;
                if idx == 0 {
                    RuntimeValue::String(String::new())
                } else if let Some(ch) = s.chars().nth(idx - 1) {
                    RuntimeValue::String(ch.to_string())
                } else {
                    RuntimeValue::String(String::new())
                }
            }
            "text.upper" => {
                let s = args.first().and_then(|v| v.as_string()).unwrap_or("");
                RuntimeValue::String(s.to_uppercase())
            }
            "text.lower" => {
                let s = args.first().and_then(|v| v.as_string()).unwrap_or("");
                RuntimeValue::String(s.to_lowercase())
            }
            "sound.get_volume" => {
                let cur = world.get_var("__master_volume").and_then(|v| v.as_number()).unwrap_or(100.0);
                RuntimeValue::Number(cur)
            }
            "music.get_tempo" => {
                let bpm = world.get_var("__music_tempo").and_then(|v| v.as_number()).unwrap_or(60.0);
                RuntimeValue::Number(bpm)
            }
            "current_time" => {
                let unit = args.first().and_then(|v| v.as_string()).unwrap_or("second");
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
                    _ => 0.0,
                };
                RuntimeValue::Number(val)
            }
            "days_since_2000" => {
                let dur = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                let secs_since_1970 = dur.as_secs() as f64;
                let secs_from_1970_to_2000 = 946684800.0;
                let days = (secs_since_1970 - secs_from_1970_to_2000) / 86400.0;
                RuntimeValue::Number(days.max(0.0))
            }
            "get_username" => {
                RuntimeValue::String("scratch_dev".to_string())
            }
            "distance_to" => {
                let target_a = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let target_b = args.get(1).and_then(|v| v.as_string()).unwrap_or("");
                if let (Some(a), Some(b)) = (
                    world.get_entity_by_name(target_a),
                    world.get_entity_by_name(target_b),
                ) {
                    let dx = (a.transform.x - b.transform.x) as f64;
                    let dy = (a.transform.y - b.transform.y) as f64;
                    RuntimeValue::Number((dx * dx + dy * dy).sqrt())
                } else {
                    RuntimeValue::Number(0.0)
                }
            }
            "get_direction" => {
                let target = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let rot = world.get_entity_by_name(target).map(|e| e.transform.rotation as f64).unwrap_or(0.0);
                RuntimeValue::Number(rot)
            }
            "get_size" => {
                let target = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let sz = world.get_entity_by_name(target).map(|e| (e.transform.scale_x * 100.0).round() as f64).unwrap_or(100.0);
                RuntimeValue::Number(sz)
            }
            "get_costume_number" => {
                RuntimeValue::Number(1.0)
            }
            "get_costume_name" => {
                let target = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let name = world.get_entity_by_name(target).map(|e| e.costume_name.clone()).unwrap_or_else(|| "costume1".to_string());
                RuntimeValue::String(name)
            }
            "get_backdrop_number" => {
                RuntimeValue::Number(world.get_backdrop_number() as f64)
            }
            "get_backdrop_name" => {
                RuntimeValue::String(world.get_backdrop_name().to_string())
            }
            "current" => {
                let unit = args.first().and_then(|v| v.as_string()).unwrap_or("second");
                RuntimeValue::Number(SensingSystem::current(unit))
            }
            "list.index_of" | "item_num_of_list" => {
                let list_name = args.first().and_then(|v| v.as_string()).unwrap_or("");
                let item = args.get(1).unwrap_or(&RuntimeValue::Nil);
                RuntimeValue::Number(ListSystem::index_of(world, list_name, item))
            }
            "list.to_string" => {
                let list_name = args.first().and_then(|v| v.as_string()).unwrap_or("");
                RuntimeValue::String(ListSystem::to_string(world, list_name))
            }
            "translate.text" => {
                let s = args.first().map(|v| v.to_string()).unwrap_or_default();
                RuntimeValue::String(s)
            }
            "translate.get_language" => {
                RuntimeValue::String("en".to_string())
            }
            _ => RuntimeValue::Nil,
        }
    }
}
