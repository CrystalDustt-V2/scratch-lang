use crate::ast::*;
use crate::lexer::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut events = Vec::new();
        let mut functions = Vec::new();
        let mut top_level = Vec::new();

        self.skip_newlines();

        while !self.is_at_end() {
            match self.peek_kind() {
                TokenKind::When | TokenKind::Every | TokenKind::After => {
                    events.push(self.parse_event()?);
                }
                TokenKind::Function => {
                    functions.push(self.parse_function()?);
                }
                _ => {
                    top_level.push(self.parse_statement()?);
                }
            }
            self.skip_newlines();
        }

        Ok(Program {
            events,
            functions,
            top_level,
        })
    }

    fn parse_event(&mut self) -> Result<EventDef, ParseError> {
        let start_span = self.peek_span();

        let kind = if self.match_token(TokenKind::When) {
            match self.peek_kind() {
                TokenKind::Start => {
                    self.advance();
                    EventKind::Start
                }
                TokenKind::Update => {
                    self.advance();
                    EventKind::Update
                }
                TokenKind::Ident(id) if id == "action" || id == "key" => {
                    self.advance();
                    self.consume(TokenKind::Dot, "Expected '.' after 'action' or 'key'")?;

                    let mode = match self.peek_kind() {
                        TokenKind::Ident(sub) => {
                            let s = sub.clone();
                            self.advance();
                            s
                        }
                        _ => return Err(self.error("Expected 'down', 'press', or 'up' after action dot")),
                    };

                    self.consume(TokenKind::LParen, "Expected '(' after action method")?;
                    let action_name = match self.peek_kind() {
                        TokenKind::StringLit(s) => {
                            let val = s.clone();
                            self.advance();
                            val
                        }
                        TokenKind::Ident(s) => {
                            let val = s.clone();
                            self.advance();
                            val
                        }
                        _ => return Err(self.error("Expected action name string in parentheses")),
                    };
                    self.consume(TokenKind::RParen, "Expected ')' after action name")?;

                    match mode.as_str() {
                        "down" => EventKind::ActionDown(action_name),
                        "press" => EventKind::ActionPress(action_name),
                        "up" => EventKind::ActionUp(action_name),
                        other => return Err(self.error(&format!("Unknown action event '{}'", other))),
                    }
                }
                TokenKind::Ident(first_obj) => {
                    let obj_a = first_obj.clone();
                    self.advance();

                    if self.match_token(TokenKind::Touches) {
                        let obj_b = match self.peek_kind() {
                            TokenKind::Ident(second_obj) => {
                                let val = second_obj.clone();
                                self.advance();
                                val
                            }
                            _ => return Err(self.error("Expected target object identifier after 'touches'")),
                        };
                        EventKind::Touches { object_a: obj_a, object_b: obj_b }
                    } else {
                        return Err(self.error(&format!("Expected 'touches' or action after object '{}'", obj_a)));
                    }
                }
                _ => return Err(self.error("Expected event condition after 'when'")),
            }
        } else if self.match_token(TokenKind::Every) {
            let seconds = self.parse_number_literal()? as u64;
            self.consume(TokenKind::Seconds, "Expected 'seconds' after number in every event")?;
            EventKind::EverySeconds(seconds)
        } else if self.match_token(TokenKind::After) {
            let seconds = self.parse_number_literal()? as u64;
            self.consume(TokenKind::Seconds, "Expected 'seconds' after number in after event")?;
            EventKind::AfterSeconds(seconds)
        } else {
            return Err(self.error("Expected 'when', 'every', or 'after'"));
        };

        self.consume(TokenKind::Colon, "Expected ':' after event declaration")?;
        let body = self.parse_block()?;

        Ok(EventDef {
            kind,
            body,
            span: start_span,
        })
    }

    fn parse_function(&mut self) -> Result<FunctionDef, ParseError> {
        let start_span = self.peek_span();
        self.consume(TokenKind::Function, "Expected 'function'")?;

        let name = match self.peek_kind() {
            TokenKind::Ident(id) => {
                let s = id.clone();
                self.advance();
                s
            }
            _ => return Err(self.error("Expected function name")),
        };

        self.consume(TokenKind::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                match self.peek_kind() {
                    TokenKind::Ident(p) => {
                        params.push(p.clone());
                        self.advance();
                    }
                    _ => return Err(self.error("Expected parameter name")),
                }
                if self.match_token(TokenKind::Comma) {
                    continue;
                }
                break;
            }
        }
        self.consume(TokenKind::RParen, "Expected ')' after parameter list")?;
        self.consume(TokenKind::Colon, "Expected ':' after function signature")?;

        let body = self.parse_block()?;
        Ok(FunctionDef {
            name,
            params,
            body,
            span: start_span,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.consume(TokenKind::Newline, "Expected newline after ':'")?;
        self.skip_newlines();

        self.consume(TokenKind::Indent, "Expected indented block")?;
        let mut statements = Vec::new();

        self.skip_newlines();
        while !self.check(TokenKind::Dedent) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.consume(TokenKind::Dedent, "Expected end of indented block")?;
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.skip_newlines();
        let span = self.peek_span();

        if self.match_token(TokenKind::If) {
            let condition = self.parse_expression()?;
            self.consume(TokenKind::Colon, "Expected ':' after if condition")?;
            let then_body = self.parse_block()?;

            let else_body = if self.match_token(TokenKind::Else) {
                self.consume(TokenKind::Colon, "Expected ':' after else")?;
                Some(self.parse_block()?)
            } else {
                None
            };

            return Ok(Statement::If {
                condition,
                then_body,
                else_body,
                span,
            });
        }

        if self.match_token(TokenKind::Repeat) {
            let count = self.parse_expression()?;
            self.consume(TokenKind::Colon, "Expected ':' after repeat count")?;
            let body = self.parse_block()?;
            return Ok(Statement::Repeat { count, body, span });
        }

        if self.match_token(TokenKind::Return) {
            let value = if self.check(TokenKind::Newline) || self.check(TokenKind::Dedent) || self.is_at_end() {
                None
            } else {
                Some(self.parse_expression()?)
            };
            self.match_token(TokenKind::Newline);
            return Ok(Statement::Return { value, span });
        }

        if self.match_token(TokenKind::Assert) {
            let expr = self.parse_expression()?;
            self.match_token(TokenKind::Newline);
            return Ok(Statement::Assert { expr, span });
        }

        // Lookahead to differentiate Assignment vs Call vs Expression
        if let TokenKind::Ident(name) = self.peek_kind().clone() {
            // Check next token
            if let Some(next) = self.tokens.get(self.cursor + 1) {
                match next.kind {
                    TokenKind::Assign => {
                        self.advance(); // consume ident
                        self.advance(); // consume '='
                        let value = self.parse_expression()?;
                        self.match_token(TokenKind::Newline);
                        return Ok(Statement::Assign {
                            target: name,
                            op: AssignOp::Assign,
                            value,
                            span,
                        });
                    }
                    TokenKind::PlusAssign => {
                        self.advance();
                        self.advance();
                        let value = self.parse_expression()?;
                        self.match_token(TokenKind::Newline);
                        return Ok(Statement::Assign {
                            target: name,
                            op: AssignOp::AddAssign,
                            value,
                            span,
                        });
                    }
                    TokenKind::MinusAssign => {
                        self.advance();
                        self.advance();
                        let value = self.parse_expression()?;
                        self.match_token(TokenKind::Newline);
                        return Ok(Statement::Assign {
                            target: name,
                            op: AssignOp::SubAssign,
                            value,
                            span,
                        });
                    }
                    TokenKind::StarAssign => {
                        self.advance();
                        self.advance();
                        let value = self.parse_expression()?;
                        self.match_token(TokenKind::Newline);
                        return Ok(Statement::Assign {
                            target: name,
                            op: AssignOp::MulAssign,
                            value,
                            span,
                        });
                    }
                    TokenKind::SlashAssign => {
                        self.advance();
                        self.advance();
                        let value = self.parse_expression()?;
                        self.match_token(TokenKind::Newline);
                        return Ok(Statement::Assign {
                            target: name,
                            op: AssignOp::DivAssign,
                            value,
                            span,
                        });
                    }
                    TokenKind::Dot => {
                        // target.method(args)
                        self.advance(); // consume target ident
                        self.advance(); // consume '.'
                        let method = match self.peek_kind() {
                            TokenKind::Ident(m) => {
                                let s = m.clone();
                                self.advance();
                                s
                            }
                            _ => return Err(self.error("Expected method name after '.'")),
                        };
                        self.consume(TokenKind::LParen, "Expected '(' after method name")?;
                        let args = self.parse_arg_list()?;
                        self.match_token(TokenKind::Newline);
                        return Ok(Statement::Call {
                            target: Some(name),
                            name: method,
                            args,
                            span,
                        });
                    }
                    TokenKind::LParen => {
                        // func(args)
                        self.advance(); // consume ident
                        self.advance(); // consume '('
                        let args = self.parse_arg_list()?;
                        self.match_token(TokenKind::Newline);
                        return Ok(Statement::Call {
                            target: None,
                            name,
                            args,
                            span,
                        });
                    }
                    _ => {}
                }
            }
        }

        // Fallback: parse as expression
        let expr = self.parse_expression()?;
        self.match_token(TokenKind::Newline);
        match expr {
            Expr::Call { target, name, args, span } => Ok(Statement::Call {
                target,
                name,
                args,
                span,
            }),
            other => Err(self.error_at("Unexpected expression statement", other.span())),
        }
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                args.push(self.parse_expression()?);
                if self.match_token(TokenKind::Comma) {
                    continue;
                }
                break;
            }
        }
        self.consume(TokenKind::RParen, "Expected ')' after arguments")?;
        Ok(args)
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_logical_and()?;

        while self.match_token(TokenKind::Or) {
            let op_span = self.peek_span();
            let right = self.parse_logical_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
                span: op_span,
            };
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;

        while self.match_token(TokenKind::And) {
            let op_span = self.peek_span();
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
                span: op_span,
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = self.match_equality_op() {
            let op_span = self.peek_span();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: op_span,
            };
        }

        Ok(expr)
    }

    fn match_equality_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenKind::Equal) {
            Some(BinaryOp::Equal)
        } else if self.match_token(TokenKind::NotEqual) {
            Some(BinaryOp::NotEqual)
        } else {
            None
        }
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;

        while let Some(op) = self.match_comparison_op() {
            let op_span = self.peek_span();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: op_span,
            };
        }

        Ok(expr)
    }

    fn match_comparison_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenKind::Less) {
            Some(BinaryOp::Less)
        } else if self.match_token(TokenKind::LessEqual) {
            Some(BinaryOp::LessEqual)
        } else if self.match_token(TokenKind::Greater) {
            Some(BinaryOp::Greater)
        } else if self.match_token(TokenKind::GreaterEqual) {
            Some(BinaryOp::GreaterEqual)
        } else {
            None
        }
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = self.match_term_op() {
            let op_span = self.peek_span();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: op_span,
            };
        }

        Ok(expr)
    }

    fn match_term_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenKind::Plus) {
            Some(BinaryOp::Add)
        } else if self.match_token(TokenKind::Minus) {
            Some(BinaryOp::Sub)
        } else {
            None
        }
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = self.match_factor_op() {
            let op_span = self.peek_span();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: op_span,
            };
        }

        Ok(expr)
    }

    fn match_factor_op(&mut self) -> Option<BinaryOp> {
        if self.match_token(TokenKind::Star) {
            Some(BinaryOp::Mul)
        } else if self.match_token(TokenKind::Slash) {
            Some(BinaryOp::Div)
        } else if self.match_token(TokenKind::Percent) {
            Some(BinaryOp::Mod)
        } else {
            None
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        let span = self.peek_span();
        if self.match_token(TokenKind::Not) {
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
                span,
            });
        }
        if self.match_token(TokenKind::Minus) {
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
                span,
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let span = self.peek_span();

        match self.peek_kind() {
            TokenKind::Number(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Number(val, span))
            }
            TokenKind::StringLit(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::String(val, span))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Bool(true, span))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Bool(false, span))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RParen, "Expected ')' after grouped expression")?;
                Ok(expr)
            }
            TokenKind::Ident(name) => {
                let ident_name = name.clone();
                self.advance();

                if self.match_token(TokenKind::Dot) {
                    let method = match self.peek_kind() {
                        TokenKind::Ident(m) => {
                            let s = m.clone();
                            self.advance();
                            s
                        }
                        _ => return Err(self.error("Expected property or method name after '.'")),
                    };

                    if self.match_token(TokenKind::LParen) {
                        let args = self.parse_arg_list()?;
                        Ok(Expr::Call {
                            target: Some(ident_name),
                            name: method,
                            args,
                            span,
                        })
                    } else {
                        // For now, property access can be modeled or treated
                        Ok(Expr::Ident(format!("{}.{}", ident_name, method), span))
                    }
                } else if self.match_token(TokenKind::LParen) {
                    let args = self.parse_arg_list()?;
                    Ok(Expr::Call {
                        target: None,
                        name: ident_name,
                        args,
                        span,
                    })
                } else {
                    Ok(Expr::Ident(ident_name, span))
                }
            }
            _ => Err(self.error(&format!("Unexpected token: {:?}", self.peek_kind()))),
        }
    }

    fn parse_number_literal(&mut self) -> Result<f64, ParseError> {
        match self.peek_kind() {
            TokenKind::Number(n) => {
                let val = *n;
                self.advance();
                Ok(val)
            }
            _ => Err(self.error("Expected number literal")),
        }
    }

    fn check(&self, expected: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.tokens[self.cursor].kind == expected
        }
    }

    fn match_token(&mut self, expected: TokenKind) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, expected: TokenKind, msg: &str) -> Result<&Token, ParseError> {
        if self.check(expected) {
            Ok(self.advance().unwrap())
        } else {
            Err(self.error(msg))
        }
    }

    fn skip_newlines(&mut self) {
        while self.check(TokenKind::Newline) {
            self.advance();
        }
    }

    fn peek_kind(&self) -> &TokenKind {
        if self.cursor < self.tokens.len() {
            &self.tokens[self.cursor].kind
        } else {
            &TokenKind::Eof
        }
    }

    fn peek_span(&self) -> Span {
        if self.cursor < self.tokens.len() {
            self.tokens[self.cursor].span
        } else {
            Span::default()
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.cursor < self.tokens.len() {
            let tok = &self.tokens[self.cursor];
            self.cursor += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len() || self.peek_kind() == &TokenKind::Eof
    }

    fn error(&self, msg: &str) -> ParseError {
        ParseError {
            message: msg.to_string(),
            span: self.peek_span(),
        }
    }

    fn error_at(&self, msg: &str, span: Span) -> ParseError {
        ParseError {
            message: msg.to_string(),
            span,
        }
    }
}
