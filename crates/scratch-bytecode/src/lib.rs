pub mod chunk;
pub mod compiler;
pub mod opcode;

pub use chunk::Chunk;
pub use compiler::{BytecodeCompiler, BytecodeEventHandler, BytecodeProgram};
pub use opcode::{BytecodeValue, OpCode};

#[cfg(test)]
mod tests {
    use super::*;
    use scratch_blocks::BlockRegistry;
    use scratch_ir::lower_ast_to_ir;
    use scratch_language::parse;

    #[test]
    fn test_compile_milestone1_to_bytecode() {
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

        assert_eq!(program.events.len(), 2);

        // Start event: PushConst(0), StoreVar("score"), Halt
        let start_chunk = &program.events[0].chunk;
        assert_eq!(start_chunk.instructions.len(), 3);
        assert!(matches!(start_chunk.instructions[0], OpCode::PushConst(_)));
        assert_eq!(start_chunk.instructions[1], OpCode::StoreVar("score".into()));
        assert_eq!(start_chunk.instructions[2], OpCode::Halt);

        // ActionDown event: LoadVar("Player"), PushConst(5), CallBlock("move", 2), Halt
        let action_chunk = &program.events[1].chunk;
        assert_eq!(action_chunk.instructions.len(), 4);
        assert_eq!(action_chunk.instructions[0], OpCode::LoadVar("Player".into()));
        assert!(matches!(action_chunk.instructions[1], OpCode::PushConst(_)));
        assert_eq!(
            action_chunk.instructions[2],
            OpCode::CallBlock {
                name: "move".into(),
                arg_count: 2
            }
        );
        assert_eq!(action_chunk.instructions[3], OpCode::Halt);
    }
}
