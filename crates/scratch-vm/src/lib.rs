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
}
