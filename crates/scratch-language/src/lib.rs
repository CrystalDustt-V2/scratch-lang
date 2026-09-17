pub mod ast;
pub mod diagnostics;
pub mod formatter;
pub mod lexer;
pub mod linter;
pub mod parser;

pub use ast::*;
pub use diagnostics::Diagnostic;
pub use formatter::{format_source, Formatter};
pub use lexer::{tokenize, LexerError, Token, TokenKind};
pub use linter::{lint_source, Linter};
pub use parser::{ParseError, Parser};

pub fn parse(source: &str) -> Result<Program, String> {
    let tokens = tokenize(source).map_err(|e| format!("Lexer error at line {}:{}: {}", e.span.line, e.span.col, e.message))?;
    let mut parser = Parser::new(tokens);
    parser.parse_program().map_err(|e| format!("Parse error at line {}:{}: {}", e.span.line, e.span.col, e.message))
}

#[cfg(test)]
mod tests {
    use super::*;
    use scratch_blocks::BlockRegistry;

    #[test]
    fn test_parse_milestone1_program() {
        let code = r#"
when start:
    score = 0

when action.down("right"):
    move(Player, 5)
"#;
        let program = parse(code).expect("parsing succeeds");
        assert_eq!(program.events.len(), 2);

        // Check first event: when start: score = 0
        let ev1 = &program.events[0];
        assert_eq!(ev1.kind, EventKind::Start);
        assert_eq!(ev1.body.len(), 1);
        match &ev1.body[0] {
            Statement::Assign { target, op, value, .. } => {
                assert_eq!(target, "score");
                assert_eq!(*op, AssignOp::Assign);
                match value {
                    Expr::Number(n, _) => assert_eq!(*n, 0.0),
                    _ => panic!("Expected number"),
                }
            }
            _ => panic!("Expected assignment"),
        }

        // Check second event: when action.down("right"): move(Player, 5)
        let ev2 = &program.events[1];
        assert_eq!(ev2.kind, EventKind::ActionDown("right".into()));
        assert_eq!(ev2.body.len(), 1);
        match &ev2.body[0] {
            Statement::Call { target, name, args, .. } => {
                assert_eq!(target, &None);
                assert_eq!(name, "move");
                assert_eq!(args.len(), 2);
                match &args[0] {
                    Expr::Ident(id, _) => assert_eq!(id, "Player"),
                    _ => panic!("Expected identifier Player"),
                }
                match &args[1] {
                    Expr::Number(n, _) => assert_eq!(*n, 5.0),
                    _ => panic!("Expected number 5"),
                }
            }
            _ => panic!("Expected call statement"),
        }
    }

    #[test]
    fn test_parse_if_condition() {
        let code = r#"
when update:
    if health <= 0:
        respawn(Player)
        health = 3
"#;
        let program = parse(code).expect("parsing succeeds");
        assert_eq!(program.events.len(), 1);
        let ev = &program.events[0];
        assert_eq!(ev.kind, EventKind::Update);
        assert_eq!(ev.body.len(), 1);
        match &ev.body[0] {
            Statement::If { condition, then_body, else_body, .. } => {
                assert!(else_body.is_none());
                assert_eq!(then_body.len(), 2);
                match condition {
                    Expr::Binary { op, .. } => assert_eq!(*op, BinaryOp::LessEqual),
                    _ => panic!("Expected binary condition"),
                }
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn test_formatter_roundtrip() {
        let code = r#"when start:
    score = 0

when action.down("right"):
    move(Player, 5)
"#;
        let formatted = format_source(code).expect("format ok");
        assert_eq!(formatted.trim(), code.trim());
    }

    #[test]
    fn test_linter_catches_typo() {
        let code = r#"
when start:
    move(Plyer, 5)
"#;
        let registry = BlockRegistry::core();
        let diags = lint_source(code, &registry).expect("lint ok");
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "SL001");
        assert_eq!(diags[0].suggestion, Some("Player".to_string()));
    }

    #[test]
    fn test_linter_catches_unknown_command() {
        let code = r#"
when start:
    moove(Player, 5)
"#;
        let registry = BlockRegistry::core();
        let diags = lint_source(code, &registry).expect("lint ok");
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "SL003");
        assert_eq!(diags[0].suggestion, Some("move".to_string()));
    }

    #[test]
    fn test_linter_catches_missing_asset() {
        let code = r#"
when start:
    sound.play("coin")
"#;
        let registry = BlockRegistry::core();
        let mut index = scratch_assets::AssetIndex::new();
        index.register(scratch_assets::AssetInfo {
            name: "coin_pickup".into(),
            category: scratch_assets::AssetCategory::Sound,
            relative_path: "assets/sounds/coin_pickup.wav".into(),
            extension: "wav".into(),
        });

        let program = parse(code).expect("parse ok");
        let mut linter = Linter::new(&registry).with_assets(&index);
        let diags = linter.lint_program(&program);

        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "SL005");
        assert!(diags[0].message.contains("coin"));
    }
}
