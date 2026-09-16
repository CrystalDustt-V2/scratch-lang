pub mod scanner;
pub mod token;

pub use scanner::{LexerError, Scanner};
pub use token::{Token, TokenKind};

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
    let mut scanner = Scanner::new(source);
    scanner.tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple_event() {
        let code = r#"
when start:
    score = 0
"#;
        let tokens = tokenize(code).expect("tokenization succeeds");
        let kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();

        assert_eq!(
            kinds,
            vec![
                TokenKind::When,
                TokenKind::Start,
                TokenKind::Colon,
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Ident("score".into()),
                TokenKind::Assign,
                TokenKind::Number(0.0),
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_lex_action_down() {
        let code = r#"
when action.down("right"):
    move(Player, 5)
"#;
        let tokens = tokenize(code).expect("tokenization succeeds");
        let kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();

        assert_eq!(
            kinds,
            vec![
                TokenKind::When,
                TokenKind::Ident("action".into()),
                TokenKind::Dot,
                TokenKind::Ident("down".into()),
                TokenKind::LParen,
                TokenKind::StringLit("right".into()),
                TokenKind::RParen,
                TokenKind::Colon,
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Ident("move".into()),
                TokenKind::LParen,
                TokenKind::Ident("Player".into()),
                TokenKind::Comma,
                TokenKind::Number(5.0),
                TokenKind::RParen,
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_lex_touches_event() {
        let code = "when Player touches Coin:\n    collect(Coin)";
        let tokens = tokenize(code).expect("tokenization succeeds");
        let kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();

        assert_eq!(
            kinds,
            vec![
                TokenKind::When,
                TokenKind::Ident("Player".into()),
                TokenKind::Touches,
                TokenKind::Ident("Coin".into()),
                TokenKind::Colon,
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Ident("collect".into()),
                TokenKind::LParen,
                TokenKind::Ident("Coin".into()),
                TokenKind::RParen,
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Eof,
            ]
        );
    }
}
