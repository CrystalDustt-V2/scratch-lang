use crate::ast::*;
use crate::diagnostics::Diagnostic;
use scratch_blocks::BlockRegistry;
use std::collections::HashSet;

pub struct Linter<'a> {
    registry: &'a BlockRegistry,
    known_objects: HashSet<String>,
    known_actions: HashSet<String>,
    diagnostics: Vec<Diagnostic>,
    defined_vars: HashSet<String>,
    declared_functions: HashSet<String>,
}

impl<'a> Linter<'a> {
    pub fn new(registry: &'a BlockRegistry) -> Self {
        let mut known_objects = HashSet::new();
        // Default objects
        known_objects.insert("Player".to_string());
        known_objects.insert("Enemy".to_string());
        known_objects.insert("Coin".to_string());
        known_objects.insert("Goal".to_string());
        known_objects.insert("Door".to_string());

        let mut known_actions = HashSet::new();
        known_actions.insert("left".to_string());
        known_actions.insert("right".to_string());
        known_actions.insert("up".to_string());
        known_actions.insert("down".to_string());
        known_actions.insert("jump".to_string());
        known_actions.insert("action".to_string());
        known_actions.insert("pause".to_string());

        Self {
            registry,
            known_objects,
            known_actions,
            diagnostics: Vec::new(),
            defined_vars: HashSet::new(),
            declared_functions: HashSet::new(),
        }
    }

    pub fn with_objects(mut self, objects: impl IntoIterator<Item = String>) -> Self {
        self.known_objects.extend(objects);
        self
    }

    pub fn lint_program(&mut self, program: &Program) -> Vec<Diagnostic> {
        self.diagnostics.clear();
        self.defined_vars.clear();
        self.declared_functions.clear();

        // Pass 1: Record declared functions
        for func in &program.functions {
            self.declared_functions.insert(func.name.clone());
        }

        // Pass 2: Top-level statements
        for stmt in &program.top_level {
            self.lint_statement(stmt, false);
        }

        // Pass 3: Functions
        for func in &program.functions {
            let outer_vars = self.defined_vars.clone();
            for param in &func.params {
                self.defined_vars.insert(param.clone());
            }
            for stmt in &func.body {
                self.lint_statement(stmt, false);
            }
            self.defined_vars = outer_vars;
        }

        // Pass 4: Events
        for event in &program.events {
            self.lint_event(event);
        }

        self.diagnostics.clone()
    }

    fn lint_event(&mut self, event: &EventDef) {
        match &event.kind {
            EventKind::ActionDown(action)
            | EventKind::ActionPress(action)
            | EventKind::ActionUp(action) => {
                if !self.known_actions.contains(action) {
                    let mut diag = Diagnostic::new(
                        "SL010",
                        format!("Unknown input action \"{}\".", action),
                        event.span,
                    );
                    if let Some(closest) = find_closest(action, &self.known_actions) {
                        diag = diag.with_suggestion(closest);
                    }
                    self.diagnostics.push(diag);
                }
            }
            EventKind::Touches { object_a, object_b } => {
                self.check_object(object_a, event.span);
                self.check_object(object_b, event.span);
            }
            _ => {}
        }

        let is_update = matches!(event.kind, EventKind::Update);
        let mut unreachable = false;

        for stmt in &event.body {
            if unreachable {
                self.diagnostics.push(Diagnostic::new(
                    "SL006",
                    "Unreachable code detected.",
                    stmt_span(stmt),
                ));
            }

            self.lint_statement(stmt, is_update);

            if matches!(stmt, Statement::Return { .. }) {
                unreachable = true;
            }
        }
    }

    fn lint_statement(&mut self, stmt: &Statement, is_update: bool) {
        match stmt {
            Statement::Assign { target, value, .. } => {
                self.lint_expr(value);
                self.defined_vars.insert(target.clone());
            }
            Statement::Call { target, name, args, span } => {
                let full_name = match target {
                    Some(t) => format!("{}.{}", t, name),
                    None => name.clone(),
                };

                // Check SL003: Unknown command
                if !self.registry.contains(&full_name) && !self.declared_functions.contains(&full_name) {
                    let mut diag = Diagnostic::new(
                        "SL003",
                        format!("Unknown game command or function \"{}\".", full_name),
                        *span,
                    );
                    if let Some(similar) = self.registry.find_similar(&full_name) {
                        diag = diag.with_suggestion(similar);
                    }
                    self.diagnostics.push(diag);
                } else if let Some(block_def) = self.registry.get(&full_name) {
                    // Check SL004: Parameter count check
                    let min_params = block_def.parameters.iter().filter(|p| !p.optional).count();
                    let max_params = block_def.parameters.len();
                    if args.len() < min_params || args.len() > max_params {
                        self.diagnostics.push(Diagnostic::new(
                            "SL004",
                            format!(
                                "Command \"{}\" expects between {} and {} arguments, but got {}.",
                                full_name, min_params, max_params, args.len()
                            ),
                            *span,
                        ));
                    }

                    // Check SL007: Potential per-frame misuse (e.g. spawning or resetting scene inside update)
                    if is_update && (full_name == "scene.switch" || full_name == "scene.restart") {
                        self.diagnostics.push(Diagnostic::new(
                            "SL007",
                            format!("Potential per-frame misuse: Calling \"{}\" inside 'when update' will trigger every frame.", full_name),
                            *span,
                        ));
                    }
                }

                let block_def = self.registry.get(&full_name);
                for (i, arg) in args.iter().enumerate() {
                    let is_object_param = block_def.and_then(|def| def.parameters.get(i)).map_or(false, |p| p.param_type == scratch_blocks::BlockType::Object);
                    if is_object_param {
                        if let Expr::Ident(obj_name, arg_span) = arg {
                            self.check_object(obj_name, *arg_span);
                            continue;
                        }
                    }
                    self.lint_expr(arg);
                }
            }
            Statement::If { condition, then_body, else_body, .. } => {
                self.lint_expr(condition);
                for s in then_body {
                    self.lint_statement(s, is_update);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.lint_statement(s, is_update);
                    }
                }
            }
            Statement::Repeat { count, body, span } => {
                self.lint_expr(count);
                if let Expr::Number(n, _) = count {
                    if *n <= 0.0 {
                        self.diagnostics.push(Diagnostic::new(
                            "SL008",
                            format!("Repeat count is {} <= 0, body will never execute.", n),
                            *span,
                        ));
                    }
                }
                for s in body {
                    self.lint_statement(s, is_update);
                }
            }
            Statement::Return { value, .. } => {
                if let Some(v) = value {
                    self.lint_expr(v);
                }
            }
            Statement::Assert { expr, .. } => {
                self.lint_expr(expr);
            }
        }
    }

    fn lint_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(id, span) => {
                // If it's not a known object, not a registered block, not a declared function, and not a defined var
                if !self.defined_vars.contains(id)
                    && !self.known_objects.contains(id)
                    && !self.registry.contains(id)
                    && !self.declared_functions.contains(id)
                {
                    let mut diag = Diagnostic::new(
                        "SL002",
                        format!("Variable \"{}\" used before being assigned a value.", id),
                        *span,
                    );
                    if let Some(closest) = find_closest(id, &self.defined_vars) {
                        diag = diag.with_suggestion(closest);
                    }
                    self.diagnostics.push(diag);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.lint_expr(left);
                self.lint_expr(right);
            }
            Expr::Unary { expr, .. } => {
                self.lint_expr(expr);
            }
            Expr::Call { target, name, args, span } => {
                let full_name = match target {
                    Some(t) => format!("{}.{}", t, name),
                    None => name.clone(),
                };
                if !self.registry.contains(&full_name) && !self.declared_functions.contains(&full_name) {
                    let mut diag = Diagnostic::new(
                        "SL003",
                        format!("Unknown game function \"{}\".", full_name),
                        *span,
                    );
                    if let Some(similar) = self.registry.find_similar(&full_name) {
                        diag = diag.with_suggestion(similar);
                    }
                    self.diagnostics.push(diag);
                }
                for arg in args {
                    self.lint_expr(arg);
                }
            }
            _ => {}
        }
    }

    fn check_object(&mut self, name: &str, span: Span) {
        if !self.known_objects.contains(name) {
            let mut diag = Diagnostic::new(
                "SL001",
                format!("Unknown object \"{}\".", name),
                span,
            );
            if let Some(closest) = find_closest(name, &self.known_objects) {
                diag = diag.with_suggestion(closest);
            }
            self.diagnostics.push(diag);
        }
    }
}

fn stmt_span(stmt: &Statement) -> Span {
    match stmt {
        Statement::Assign { span, .. } => *span,
        Statement::Call { span, .. } => *span,
        Statement::If { span, .. } => *span,
        Statement::Repeat { span, .. } => *span,
        Statement::Return { span, .. } => *span,
        Statement::Assert { span, .. } => *span,
    }
}

fn find_closest<'a>(query: &str, candidates: &'a HashSet<String>) -> Option<&'a str> {
    let mut closest = None;
    let mut min_dist = usize::MAX;

    for cand in candidates {
        let dist = levenshtein(query, cand);
        if dist < min_dist && dist <= 3 {
            min_dist = dist;
            closest = Some(cand.as_str());
        }
    }

    closest
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let (m, n) = (a_chars.len(), b_chars.len());

    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1].to_ascii_lowercase() == b_chars[j - 1].to_ascii_lowercase() {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}

pub fn lint_source(source: &str, registry: &BlockRegistry) -> Result<Vec<Diagnostic>, String> {
    let program = crate::parse(source)?;
    let mut linter = Linter::new(registry);
    Ok(linter.lint_program(&program))
}
