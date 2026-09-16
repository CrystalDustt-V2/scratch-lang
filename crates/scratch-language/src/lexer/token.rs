use crate::ast::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Virtual Indentation
    Indent,
    Dedent,
    Newline,

    // Keywords
    When,
    Start,
    Update,
    If,
    Else,
    Repeat,
    For,
    Function,
    Return,
    And,
    Or,
    Not,
    Assert,
    Test,
    True,
    False,

    // Event keywords
    Touches,
    Enters,
    Every,
    After,
    Seconds,

    // Literals & Identifiers
    Ident(String),
    Number(f64),
    StringLit(String),

    // Symbols
    Colon,        // :
    LParen,       // (
    RParen,       // )
    Comma,        // ,
    Dot,          // .
    LBracket,     // [
    RBracket,     // ]
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Assign,       // =
    PlusAssign,   // +=
    MinusAssign,  // -=
    StarAssign,   // *=
    SlashAssign,  // /=
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
