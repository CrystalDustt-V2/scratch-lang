use crate::ast::Span;
use crate::lexer::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub span: Span,
}

pub struct Scanner<'a> {
    source: &'a str,
    chars: Vec<(usize, char)>,
    cursor: usize,
    line: usize,
    col: usize,
    indent_stack: Vec<usize>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars: Vec<(usize, char)> = source.char_indices().collect();
        Self {
            source,
            chars,
            cursor: 0,
            line: 1,
            col: 1,
            indent_stack: vec![0],
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        let mut at_line_start = true;
        let mut line_has_tokens = false;

        while self.cursor < self.chars.len() {
            if at_line_start {
                // Measure indentation
                let indent_start_col = self.col;
                let indent_start_pos = self.current_pos();
                let mut spaces = 0;

                while let Some(&(_, ch)) = self.peek() {
                    if ch == ' ' {
                        spaces += 1;
                        self.advance();
                    } else if ch == '\t' {
                        spaces += 4;
                        self.advance();
                    } else {
                        break;
                    }
                }

                // Check if line is empty or comment
                if let Some(&(_, ch)) = self.peek() {
                    if ch == '\n' || ch == '\r' || ch == '#' {
                        // Skip blank or comment-only line
                        if ch == '#' {
                            self.skip_comment();
                        }
                        if let Some(&(_, '\r')) = self.peek() {
                            self.advance();
                        }
                        if let Some(&(_, '\n')) = self.peek() {
                            self.advance();
                            self.line += 1;
                            self.col = 1;
                        }
                        continue;
                    }
                } else {
                    // Reached end of file while looking at indentation
                    break;
                }

                // Non-empty line: reconcile indentation
                let current_indent = *self.indent_stack.last().unwrap_or(&0);
                let span = Span::new(self.line, indent_start_col, indent_start_pos, self.current_pos());

                if spaces > current_indent {
                    self.indent_stack.push(spaces);
                    tokens.push(Token::new(TokenKind::Indent, span));
                } else if spaces < current_indent {
                    while let Some(&top) = self.indent_stack.last() {
                        if top <= spaces {
                            break;
                        }
                        self.indent_stack.pop();
                        tokens.push(Token::new(TokenKind::Dedent, span));
                    }

                    if *self.indent_stack.last().unwrap_or(&0) != spaces {
                        return Err(LexerError {
                            message: format!("Inconsistent indentation: {} spaces does not match any outer indentation level", spaces),
                            span,
                        });
                    }
                }

                at_line_start = false;
                line_has_tokens = false;
            }

            // Skip inline whitespace
            while let Some(&(_, ch)) = self.peek() {
                if ch == ' ' || ch == '\t' {
                    self.advance();
                } else {
                    break;
                }
            }

            if self.cursor >= self.chars.len() {
                break;
            }

            let (_, ch) = *self.peek().unwrap();

            // Handle comments
            if ch == '#' {
                self.skip_comment();
                continue;
            }

            // Handle newline
            if ch == '\n' || ch == '\r' {
                if ch == '\r' {
                    self.advance();
                }
                if let Some(&(_, '\n')) = self.peek() {
                    let start_pos = self.current_pos();
                    let line = self.line;
                    let col = self.col;
                    self.advance();
                    self.line += 1;
                    self.col = 1;
                    at_line_start = true;

                    if line_has_tokens {
                        tokens.push(Token::new(TokenKind::Newline, Span::new(line, col, start_pos, start_pos + 1)));
                        line_has_tokens = false;
                    }
                }
                continue;
            }

            // Scan token
            let token = self.scan_token()?;
            tokens.push(token);
            line_has_tokens = true;
        }

        if line_has_tokens {
            tokens.push(Token::new(
                TokenKind::Newline,
                Span::new(self.line, self.col, self.current_pos(), self.current_pos()),
            ));
        }

        // Pop any remaining indent levels
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.push(Token::new(
                TokenKind::Dedent,
                Span::new(self.line, self.col, self.current_pos(), self.current_pos()),
            ));
        }

        tokens.push(Token::new(
            TokenKind::Eof,
            Span::new(self.line, self.col, self.current_pos(), self.current_pos()),
        ));

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_pos = self.current_pos();
        let start_line = self.line;
        let start_col = self.col;

        let (_, ch) = self.advance().unwrap();

        let kind = match ch {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '+' => {
                if self.match_char('=') {
                    TokenKind::PlusAssign
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.match_char('=') {
                    TokenKind::MinusAssign
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if self.match_char('=') {
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }
            '/' => {
                if self.match_char('=') {
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }
            '%' => TokenKind::Percent,
            '=' => {
                if self.match_char('=') {
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }
            '!' => {
                if self.match_char('=') {
                    TokenKind::NotEqual
                } else {
                    return Err(LexerError {
                        message: format!("Unexpected character '!'. Did you mean '!='?"),
                        span: Span::new(start_line, start_col, start_pos, start_pos + 1),
                    });
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '"' => self.scan_string(start_line, start_col, start_pos)?,
            c if c.is_ascii_digit() => self.scan_number(c, start_line, start_col, start_pos)?,
            c if is_ident_start(c) => self.scan_ident_or_keyword(c),
            unexpected => {
                return Err(LexerError {
                    message: format!("Unexpected character '{}'", unexpected),
                    span: Span::new(start_line, start_col, start_pos, start_pos + 1),
                });
            }
        };

        let end_pos = self.current_pos();
        Ok(Token::new(kind, Span::new(start_line, start_col, start_pos, end_pos)))
    }

    fn scan_string(&mut self, start_line: usize, start_col: usize, start_pos: usize) -> Result<TokenKind, LexerError> {
        let mut value = String::new();

        while let Some(&(_, ch)) = self.peek() {
            if ch == '"' {
                self.advance();
                return Ok(TokenKind::StringLit(value));
            } else if ch == '\\' {
                self.advance();
                if let Some(&(_, escaped)) = self.peek() {
                    self.advance();
                    match escaped {
                        'n' => value.push('\n'),
                        'r' => value.push('\r'),
                        't' => value.push('\t'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        other => value.push(other),
                    }
                }
            } else if ch == '\n' {
                return Err(LexerError {
                    message: "Unterminated string literal: unexpected newline in string".to_string(),
                    span: Span::new(start_line, start_col, start_pos, self.current_pos()),
                });
            } else {
                value.push(ch);
                self.advance();
            }
        }

        Err(LexerError {
            message: "Unterminated string literal: reached end of file".to_string(),
            span: Span::new(start_line, start_col, start_pos, self.current_pos()),
        })
    }

    fn scan_number(&mut self, first: char, _start_line: usize, _start_col: usize, _start_pos: usize) -> Result<TokenKind, LexerError> {
        let mut text = String::new();
        text.push(first);

        while let Some(&(_, ch)) = self.peek() {
            if ch.is_ascii_digit() {
                text.push(ch);
                self.advance();
            } else if ch == '.' {
                // Peek ahead to ensure after '.' is a digit, otherwise might be method call
                if let Some(&(_, next_ch)) = self.chars.get(self.cursor + 1) {
                    if next_ch.is_ascii_digit() {
                        text.push('.');
                        self.advance();
                        while let Some(&(_, d)) = self.peek() {
                            if d.is_ascii_digit() {
                                text.push(d);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let num: f64 = text.parse().map_err(|e| LexerError {
            message: format!("Failed to parse number: {}", e),
            span: Span::new(self.line, self.col, self.current_pos(), self.current_pos()),
        })?;

        Ok(TokenKind::Number(num))
    }

    fn scan_ident_or_keyword(&mut self, first: char) -> TokenKind {
        let mut text = String::new();
        text.push(first);

        while let Some(&(_, ch)) = self.peek() {
            if is_ident_continue(ch) {
                text.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match text.as_str() {
            "when" => TokenKind::When,
            "start" => TokenKind::Start,
            "update" => TokenKind::Update,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "repeat" => TokenKind::Repeat,
            "for" => TokenKind::For,
            "function" => TokenKind::Function,
            "return" => TokenKind::Return,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "assert" => TokenKind::Assert,
            "test" => TokenKind::Test,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "touches" => TokenKind::Touches,
            "enters" => TokenKind::Enters,
            "every" => TokenKind::Every,
            "after" => TokenKind::After,
            "seconds" => TokenKind::Seconds,
            _ => TokenKind::Ident(text),
        }
    }

    fn skip_comment(&mut self) {
        while let Some(&(_, ch)) = self.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.advance();
        }
    }

    fn peek(&self) -> Option<&(usize, char)> {
        self.chars.get(self.cursor)
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        if self.cursor < self.chars.len() {
            let item = self.chars[self.cursor];
            self.cursor += 1;
            self.col += 1;
            Some(item)
        } else {
            None
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if let Some(&(_, ch)) = self.peek() {
            if ch == expected {
                self.advance();
                return true;
            }
        }
        false
    }

    fn current_pos(&self) -> usize {
        if self.cursor < self.chars.len() {
            self.chars[self.cursor].0
        } else {
            self.source.len()
        }
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
