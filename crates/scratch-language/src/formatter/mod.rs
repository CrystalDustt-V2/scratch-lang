use crate::ast::*;

pub struct Formatter {
    indent_level: usize,
    output: String,
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            output: String::new(),
        }
    }

    pub fn format_program(program: &Program) -> String {
        let mut formatter = Self::new();

        // Top level statements
        for (i, stmt) in program.top_level.iter().enumerate() {
            formatter.format_statement(stmt);
            formatter.output.push('\n');
            if i + 1 == program.top_level.len() && !program.events.is_empty() {
                formatter.output.push('\n');
            }
        }

        // Functions
        for (i, func) in program.functions.iter().enumerate() {
            formatter.format_function(func);
            if i + 1 < program.functions.len() || !program.events.is_empty() {
                formatter.output.push('\n');
                formatter.output.push('\n');
            }
        }

        // Events
        for (i, event) in program.events.iter().enumerate() {
            formatter.format_event(event);
            if i + 1 < program.events.len() {
                formatter.output.push('\n');
            }
        }

        if !formatter.output.ends_with('\n') {
            formatter.output.push('\n');
        }

        formatter.output
    }

    fn format_function(&mut self, func: &FunctionDef) {
        self.write_indent();
        self.output.push_str(&format!("function {}({}):", func.name, func.params.join(", ")));
        self.output.push('\n');

        self.indent_level += 1;
        for stmt in &func.body {
            self.format_statement(stmt);
            self.output.push('\n');
        }
        self.indent_level -= 1;
    }

    fn format_event(&mut self, event: &EventDef) {
        self.write_indent();
        match &event.kind {
            EventKind::Start => self.output.push_str("when start:"),
            EventKind::Update => self.output.push_str("when update:"),
            EventKind::ActionDown(action) => {
                self.output.push_str(&format!("when action.down(\"{}\"):", action));
            }
            EventKind::ActionPress(action) => {
                self.output.push_str(&format!("when action.press(\"{}\"):", action));
            }
            EventKind::ActionUp(action) => {
                self.output.push_str(&format!("when action.up(\"{}\"):", action));
            }
            EventKind::Touches { object_a, object_b } => {
                self.output.push_str(&format!("when {} touches {}:", object_a, object_b));
            }
            EventKind::EverySeconds(s) => {
                self.output.push_str(&format!("every {} seconds:", s));
            }
            EventKind::AfterSeconds(s) => {
                self.output.push_str(&format!("after {} seconds:", s));
            }
            EventKind::Custom(c) => {
                self.output.push_str(&format!("when {}:", c));
            }
            EventKind::Message(m) => {
                self.output.push_str(&format!("when message(\"{}\"):", m));
            }
        }
        self.output.push('\n');

        self.indent_level += 1;
        for stmt in &event.body {
            self.format_statement(stmt);
            self.output.push('\n');
        }
        self.indent_level -= 1;
    }

    fn format_statement(&mut self, stmt: &Statement) {
        self.write_indent();
        match stmt {
            Statement::Assign { target, op, value, .. } => {
                let op_str = match op {
                    AssignOp::Assign => "=",
                    AssignOp::AddAssign => "+=",
                    AssignOp::SubAssign => "-=",
                    AssignOp::MulAssign => "*=",
                    AssignOp::DivAssign => "/=",
                };
                let val_str = self.format_expr(value);
                self.output.push_str(&format!("{} {} {}", target, op_str, val_str));
            }
            Statement::Call { target, name, args, .. } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                if let Some(t) = target {
                    self.output.push_str(&format!("{}.{}({})", t, name, args_str.join(", ")));
                } else {
                    self.output.push_str(&format!("{}({})", name, args_str.join(", ")));
                }
            }
            Statement::If { condition, then_body, else_body, .. } => {
                let cond_str = self.format_expr(condition);
                self.output.push_str(&format!("if {}:", cond_str));
                self.output.push('\n');

                self.indent_level += 1;
                for s in then_body {
                    self.format_statement(s);
                    self.output.push('\n');
                }
                self.indent_level -= 1;

                if let Some(else_stmts) = else_body {
                    self.write_indent();
                    self.output.push_str("else:");
                    self.output.push('\n');
                    self.indent_level += 1;
                    for s in else_stmts {
                        self.format_statement(s);
                        self.output.push('\n');
                    }
                    self.indent_level -= 1;
                }
                // Strip the trailing newline from the last statement because the caller adds one
                if self.output.ends_with('\n') {
                    self.output.pop();
                }
            }
            Statement::Repeat { count, body, .. } => {
                let count_str = self.format_expr(count);
                self.output.push_str(&format!("repeat {}:", count_str));
                self.output.push('\n');

                self.indent_level += 1;
                for s in body {
                    self.format_statement(s);
                    self.output.push('\n');
                }
                self.indent_level -= 1;

                if self.output.ends_with('\n') {
                    self.output.pop();
                }
            }
            Statement::Return { value, .. } => {
                if let Some(v) = value {
                    let val_str = self.format_expr(v);
                    self.output.push_str(&format!("return {}", val_str));
                } else {
                    self.output.push_str("return");
                }
            }
            Statement::Assert { expr, .. } => {
                let expr_str = self.format_expr(expr);
                self.output.push_str(&format!("assert {}", expr_str));
            }
        }
    }

    fn format_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n, _) => {
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    format!("{}", n)
                }
            }
            Expr::String(s, _) => format!("\"{}\"", s),
            Expr::Bool(b, _) => format!("{}", b),
            Expr::Ident(id, _) => id.clone(),
            Expr::Binary { left, op, right, .. } => {
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::Mod => "%",
                    BinaryOp::Equal => "==",
                    BinaryOp::NotEqual => "!=",
                    BinaryOp::Less => "<",
                    BinaryOp::LessEqual => "<=",
                    BinaryOp::Greater => ">",
                    BinaryOp::GreaterEqual => ">=",
                    BinaryOp::And => "and",
                    BinaryOp::Or => "or",
                };
                format!("{} {} {}", self.format_expr(left), op_str, self.format_expr(right))
            }
            Expr::Unary { op, expr, .. } => {
                match op {
                    UnaryOp::Not => format!("not {}", self.format_expr(expr)),
                    UnaryOp::Neg => format!("-{}", self.format_expr(expr)),
                }
            }
            Expr::Call { target, name, args, .. } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                if let Some(t) = target {
                    format!("{}.{}({})", t, name, args_str.join(", "))
                } else {
                    format!("{}({})", name, args_str.join(", "))
                }
            }
        }
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("    ");
        }
    }
}

pub fn format_source(source: &str) -> Result<String, String> {
    let program = crate::parse(source)?;
    Ok(Formatter::format_program(&program))
}
