pub mod runtime;
pub mod vm;

pub use runtime::VmRuntime;
pub use vm::{Vm, VmError};

#[cfg(test)]
mod tests {
    use super::*;
    use scratch_blocks::BlockRegistry;
    use scratch_bytecode::BytecodeCompiler;
    use scratch_ir::lower_ast_to_ir;
    use scratch_language::parse;
    use scratch_runtime::World;

    #[test]
    fn test_vm_executes_start_and_move() {
        let code = r#"
when start:
    score = 10
    score += 5

when action.down("right"):
    move(Player, 5)
"#;
        let ast = parse(code).expect("parse ok");
        let reg = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &reg).expect("ir ok");

        let mut compiler = BytecodeCompiler::new();
        let program = compiler.compile_program(&ir);

        let mut world = World::new();
        world.spawn_entity("Player");

        let mut vm = Vm::default();

        // Execute start event chunk
        vm.execute(&program.events[0].chunk, &mut world, &reg).expect("vm execute start");
        assert_eq!(world.get_var("score").unwrap().as_number(), Some(15.0));

        // Execute action.down("right") chunk
        let player = world.get_entity_by_name("Player").unwrap();
        assert_eq!(player.transform.x, 0.0);

        vm.execute(&program.events[1].chunk, &mut world, &reg).expect("vm execute move");
        let player = world.get_entity_by_name("Player").unwrap();
        assert_eq!(player.transform.x, 5.0);
    }

    #[test]
    fn test_vm_infinite_loop_protection() {
        let code = r#"
when start:
    repeat 1000000:
        score += 1
"#;
        let ast = parse(code).expect("parse ok");
        let reg = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &reg).expect("ir ok");

        let mut compiler = BytecodeCompiler::new();
        let program = compiler.compile_program(&ir);

        let mut world = World::new();
        // Give VM limited fuel of 500 steps
        let mut vm = Vm::new(500);

        let result = vm.execute(&program.events[0].chunk, &mut world, &reg);
        assert!(matches!(result, Err(VmError::OutOfFuel(500))));
    }

    #[test]
    fn test_vm_runtime_tick() {
        let code = r#"
when start:
    score = 0

when action.down("right"):
    move(Player, 5)
"#;
        let ast = parse(code).expect("parse ok");
        let reg = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &reg).expect("ir ok");

        let mut compiler = BytecodeCompiler::new();
        let program = compiler.compile_program(&ir);

        let mut vm_runtime = VmRuntime::new(program, reg);
        vm_runtime.start().expect("start ok");
        assert_eq!(vm_runtime.world.get_var("score").unwrap().as_number(), Some(0.0));

        vm_runtime.world.set_action_down("right", true);
        vm_runtime.tick(0.016).expect("tick ok");

        let player = vm_runtime.world.get_entity_by_name("Player").unwrap();
        assert_eq!(player.transform.x, 5.0);
    }

    #[test]
    fn test_vm_runtime_timers() {
        let code = r#"
when start:
    ticks = 0

every 2 seconds:
    ticks += 1
"#;
        let ast = parse(code).expect("parse ok");
        let reg = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &reg).expect("ir ok");

        let mut compiler = BytecodeCompiler::new();
        let program = compiler.compile_program(&ir);

        let mut vm_runtime = VmRuntime::new(program, reg);
        vm_runtime.start().expect("start ok");
        assert_eq!(vm_runtime.world.get_var("ticks").unwrap().as_number(), Some(0.0));

        // Advance 1 second -> should NOT have fired yet
        vm_runtime.tick(1.0).expect("tick");
        assert_eq!(vm_runtime.world.get_var("ticks").unwrap().as_number(), Some(0.0));

        // Advance another 1.1 seconds -> now total 2.1s >= 2.0s -> should fire!
        vm_runtime.tick(1.1).expect("tick");
        assert_eq!(vm_runtime.world.get_var("ticks").unwrap().as_number(), Some(1.0));

        // Advance another 2.0 seconds -> should fire second time!
        vm_runtime.tick(2.0).expect("tick");
        assert_eq!(vm_runtime.world.get_var("ticks").unwrap().as_number(), Some(2.0));
    }

    #[test]
    fn test_vm_runtime_scene_switching() {
        let code = r#"
when start:
    scene.switch("Level2")
"#;
        let ast = parse(code).expect("parse ok");
        let reg = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &reg).expect("ir ok");

        let mut compiler = BytecodeCompiler::new();
        let program = compiler.compile_program(&ir);

        let mut scene_mgr = scratch_scenes::SceneManager::new();
        let mut level2 = scratch_scenes::SceneData::default();
        level2.name = "Level2".into();
        level2.background = "night".into();
        scene_mgr.register_scene(level2);

        let mut vm_runtime = VmRuntime::new(program, reg).with_scene_manager(scene_mgr);
        vm_runtime.start().expect("start ok");

        // World background should now be switched to "night"!
        assert_eq!(vm_runtime.world.background, "night");
    }

    #[test]
    fn test_vm_executes_expanded_scratch_blocks() {
        let code = r#"
when start:
    set_x(Player, 20)
    change_x(Player, 15)
    set_y(Player, 50)
    change_y(Player, -10)
    turn_right(Player, 90)
    turn_left(Player, 30)
    set_size(Player, 120)
    change_size(Player, -10)
    hide(Player)
    show(Player)
    val1 = math.sqrt(25)
    val2 = math.round(4.8)
    val3 = math.floor(7.9)
    val4 = math.ceil(2.1)
    val5 = math.abs(-42)
    joined = text.join("Scratch", "Lang")
    len = text.length("hello")
    has_sub = text.contains("banana", "nana")
    dist = distance_to(Player, Target)
"#;
        let ast = parse(code).expect("parse ok");
        let reg = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &reg).expect("ir ok");

        let mut compiler = BytecodeCompiler::new();
        let program = compiler.compile_program(&ir);

        let mut world = World::new();
        world.spawn_entity("Player");
        world.spawn_entity("Target");
        let target_entity = world.get_entity_by_name_mut("Target").unwrap();
        target_entity.transform.x = 38.0;
        target_entity.transform.y = 44.0;

        let mut vm = Vm::default();
        vm.execute(&program.events[0].chunk, &mut world, &reg).expect("vm execute start");

        let player = world.get_entity_by_name("Player").unwrap();
        assert_eq!(player.transform.x, 35.0); // 20 + 15
        assert_eq!(player.transform.y, 40.0); // 50 - 10
        assert_eq!(player.transform.rotation, 60.0); // 90 - 30
        assert_eq!(player.transform.scale_x, 1.1); // 120% - 10% = 110% = 1.1
        assert!(player.visible);

        assert_eq!(world.get_var("val1").unwrap().as_number(), Some(5.0));
        assert_eq!(world.get_var("val2").unwrap().as_number(), Some(5.0));
        assert_eq!(world.get_var("val3").unwrap().as_number(), Some(7.0));
        assert_eq!(world.get_var("val4").unwrap().as_number(), Some(3.0));
        assert_eq!(world.get_var("val5").unwrap().as_number(), Some(42.0));
        assert_eq!(world.get_var("joined").unwrap().as_string(), Some("ScratchLang"));
        assert_eq!(world.get_var("len").unwrap().as_number(), Some(5.0));
        assert_eq!(world.get_var("has_sub").unwrap().as_bool(), true);
        assert!((world.get_var("dist").unwrap().as_number().unwrap() - 5.0).abs() < 1e-4);
    }

    #[test]
    fn test_vm_executes_extended_math_and_reporters() {
        let code = r#"
when start:
    set_x(Player, 10)
    set_y(Player, 20)
    turn_right(Player, 45)
    set_size(Player, 150)
    
    dir = get_direction(Player)
    sz = get_size(Player)
    costume = get_costume_number(Player)
    bg = get_backdrop_name()
    
    tan_val = math.tan(45)
    asin_val = math.asin(1)
    acos_val = math.acos(1)
    atan_val = math.atan(1)
    ln_val = math.ln(1)
    log_val = math.log(100)
    exp_val = math.exp(0)
    pow_val = math.pow(2, 3)
    mod_val = math.mod(14, 5)
    min_val = math.min(10, 20)
    max_val = math.max(10, 20)
    
    first_letter = text.letter_at("Scratch", 1)
    third_letter = text.letter_at("Scratch", 3)
    upper_txt = text.upper("hello")
    lower_txt = text.lower("WORLD")
    
    vol = sound.get_volume()
    tempo = music.get_tempo()
    user = get_username()
"#;
        let ast = parse(code).expect("parse ok");
        let reg = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &reg).expect("ir ok");

        let mut compiler = BytecodeCompiler::new();
        let program = compiler.compile_program(&ir);

        let mut world = World::new();
        world.background = "SkyWorld".into();
        world.spawn_entity("Player");

        let mut vm = Vm::default();
        vm.execute(&program.events[0].chunk, &mut world, &reg).expect("vm execute");

        assert_eq!(world.get_var("dir").unwrap().as_number(), Some(45.0));
        assert_eq!(world.get_var("sz").unwrap().as_number(), Some(150.0));
        assert_eq!(world.get_var("costume").unwrap().as_number(), Some(1.0));
        assert_eq!(world.get_var("bg").unwrap().as_string(), Some("SkyWorld"));

        assert!((world.get_var("tan_val").unwrap().as_number().unwrap() - 1.0).abs() < 1e-4);
        assert!((world.get_var("asin_val").unwrap().as_number().unwrap() - 90.0).abs() < 1e-4);
        assert!((world.get_var("acos_val").unwrap().as_number().unwrap() - 0.0).abs() < 1e-4);
        assert!((world.get_var("atan_val").unwrap().as_number().unwrap() - 45.0).abs() < 1e-4);
        assert_eq!(world.get_var("ln_val").unwrap().as_number(), Some(0.0));
        assert_eq!(world.get_var("log_val").unwrap().as_number(), Some(2.0));
        assert_eq!(world.get_var("exp_val").unwrap().as_number(), Some(1.0));
        assert_eq!(world.get_var("pow_val").unwrap().as_number(), Some(8.0));
        assert_eq!(world.get_var("mod_val").unwrap().as_number(), Some(4.0));
        assert_eq!(world.get_var("min_val").unwrap().as_number(), Some(10.0));
        assert_eq!(world.get_var("max_val").unwrap().as_number(), Some(20.0));

        assert_eq!(world.get_var("first_letter").unwrap().as_string(), Some("S"));
        assert_eq!(world.get_var("third_letter").unwrap().as_string(), Some("r"));
        assert_eq!(world.get_var("upper_txt").unwrap().as_string(), Some("HELLO"));
        assert_eq!(world.get_var("lower_txt").unwrap().as_string(), Some("world"));

        assert_eq!(world.get_var("vol").unwrap().as_number(), Some(100.0));
        assert_eq!(world.get_var("tempo").unwrap().as_number(), Some(60.0));
        assert!(!world.get_var("user").unwrap().as_string().unwrap().is_empty());
    }

    #[test]
    fn test_vm_broadcast_and_message_dispatch() {
        let code = r#"
when start:
    status = "running"
    broadcast("game_over")

when message("game_over"):
    status = "stopped"
    score = 100
"#;
        let ast = parse(code).expect("parse ok");
        let reg = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &reg).expect("ir ok");

        let mut compiler = BytecodeCompiler::new();
        let program = compiler.compile_program(&ir);

        let mut runtime = VmRuntime::new(program, reg);
        runtime.start().expect("start ok");

        assert_eq!(runtime.world.get_var("status").unwrap().as_string(), Some("stopped"));
        assert_eq!(runtime.world.get_var("score").unwrap().as_number(), Some(100.0));
    }
}

