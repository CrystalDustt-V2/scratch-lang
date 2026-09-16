pub mod ir;
pub mod lower;

pub use ir::*;
pub use lower::{lower_ast_to_ir, LowerError};

#[cfg(test)]
mod tests {
    use super::*;
    use scratch_blocks::BlockRegistry;
    use scratch_language::parse;

    #[test]
    fn test_lower_milestone1_program() {
        let code = r#"
when start:
    score = 0

when action.down("right"):
    move(Player, 5)
"#;
        let ast = parse(code).expect("parse ok");
        let registry = BlockRegistry::core();
        let ir = lower_ast_to_ir(&ast, &registry).expect("lowering ok");

        assert_eq!(ir.events.len(), 2);
        assert_eq!(ir.events[0].trigger, IrTrigger::OnStart);
        assert_eq!(ir.events[1].trigger, IrTrigger::OnActionDown("right".into()));

        // Check instructions
        match &ir.events[0].instructions[0] {
            IrInstruction::Assign { target, expr } => {
                assert_eq!(target, "score");
                assert_eq!(*expr, IrExpr::Constant(IrValue::Number(0.0)));
            }
            _ => panic!("Expected assign instruction"),
        }

        match &ir.events[1].instructions[0] {
            IrInstruction::CallBlock { name, args } => {
                assert_eq!(name, "move");
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], IrExpr::Var("Player".into()));
                assert_eq!(args[1], IrExpr::Constant(IrValue::Number(5.0)));
            }
            _ => panic!("Expected CallBlock instruction"),
        }
    }
}
