use crate::ir::*;
use scratch_blocks::BlockRegistry;
use scratch_language::ast::{
    AssignOp, BinaryOp, Expr, Program, Statement, UnaryOp,
    EventDef, EventKind,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LowerError {
    #[error("Unknown block '{name}'. {help}")]
    UnknownBlock { name: String, help: String },

    #[error("Semantic error: {0}")]
    Generic(String),
}

pub fn lower_ast_to_ir(program: &Program, registry: &BlockRegistry) -> Result<IrProgram, LowerError> {
    let mut ir_events = Vec::new();

    // Top-level statements become an OnStart event if present
    if !program.top_level.is_empty() {
        let mut instructions = Vec::new();
        for stmt in &program.top_level {
            instructions.push(lower_statement(stmt, registry)?);
        }
        ir_events.push(IrEventHandler {
            trigger: IrTrigger::OnStart,
            instructions,
        });
    }

    for event in &program.events {
        ir_events.push(lower_event(event, registry)?);
    }

    let mut ir_functions = Vec::new();
    for func in &program.functions {
        let mut instructions = Vec::new();
        for stmt in &func.body {
            instructions.push(lower_statement(stmt, registry)?);
        }
        ir_functions.push(IrFunction {
            name: func.name.clone(),
            params: func.params.clone(),
            instructions,
        });
    }

    Ok(IrProgram {
        events: ir_events,
        functions: ir_functions,
    })
}

fn lower_event(event: &EventDef, registry: &BlockRegistry) -> Result<IrEventHandler, LowerError> {
    let trigger = match &event.kind {
        EventKind::Start => IrTrigger::OnStart,
        EventKind::Update => IrTrigger::OnUpdate,
        EventKind::ActionDown(a) => IrTrigger::OnActionDown(a.clone()),
        EventKind::ActionPress(a) => IrTrigger::OnActionPress(a.clone()),
        EventKind::ActionUp(a) => IrTrigger::OnActionUp(a.clone()),
        EventKind::Touches { object_a, object_b } => IrTrigger::OnTouches {
            object_a: object_a.clone(),
            object_b: object_b.clone(),
        },
        EventKind::EverySeconds(s) => IrTrigger::OnTimer { seconds: *s, repeating: true },
        EventKind::AfterSeconds(s) => IrTrigger::OnTimer { seconds: *s, repeating: false },
        EventKind::Custom(c) => IrTrigger::OnActionDown(c.clone()),
        EventKind::Message(m) => IrTrigger::OnMessage(m.clone()),
    };

    let mut instructions = Vec::new();
    for stmt in &event.body {
        instructions.push(lower_statement(stmt, registry)?);
    }

    Ok(IrEventHandler {
        trigger,
        instructions,
    })
}

fn lower_statement(stmt: &Statement, registry: &BlockRegistry) -> Result<IrInstruction, LowerError> {
    match stmt {
        Statement::Assign { target, op, value, .. } => {
            let val_expr = lower_expr(value, registry)?;
            let final_expr = match op {
                AssignOp::Assign => val_expr,
                AssignOp::AddAssign => IrExpr::Binary {
                    left: Box::new(IrExpr::Var(target.clone())),
                    op: IrBinaryOp::Add,
                    right: Box::new(val_expr),
                },
                AssignOp::SubAssign => IrExpr::Binary {
                    left: Box::new(IrExpr::Var(target.clone())),
                    op: IrBinaryOp::Sub,
                    right: Box::new(val_expr),
                },
                AssignOp::MulAssign => IrExpr::Binary {
                    left: Box::new(IrExpr::Var(target.clone())),
                    op: IrBinaryOp::Mul,
                    right: Box::new(val_expr),
                },
                AssignOp::DivAssign => IrExpr::Binary {
                    left: Box::new(IrExpr::Var(target.clone())),
                    op: IrBinaryOp::Div,
                    right: Box::new(val_expr),
                },
            };
            Ok(IrInstruction::Assign {
                target: target.clone(),
                expr: final_expr,
            })
        }
        Statement::Call { target, name, args, .. } => {
            let full_name = match target {
                Some(t) => format!("{}.{}", t, name),
                None => name.clone(),
            };

            // Check against block registry; if unknown, warn or suggest
            if !registry.contains(&full_name) {
                if let Some(similar) = registry.find_similar(&full_name) {
                    let _help_msg = format!("Did you mean \"{}\"?", similar);
                }
            }

            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(lower_expr(arg, registry)?);
            }

            Ok(IrInstruction::CallBlock {
                name: full_name,
                args: lowered_args,
            })
        }
        Statement::If { condition, then_body, else_body, .. } => {
            let cond_expr = lower_expr(condition, registry)?;
            let mut then_branch = Vec::new();
            for s in then_body {
                then_branch.push(lower_statement(s, registry)?);
            }
            let mut else_branch = Vec::new();
            if let Some(eb) = else_body {
                for s in eb {
                    else_branch.push(lower_statement(s, registry)?);
                }
            }
            Ok(IrInstruction::If {
                condition: cond_expr,
                then_branch,
                else_branch,
            })
        }
        Statement::Repeat { count, body, .. } => {
            let count_expr = lower_expr(count, registry)?;
            let mut body_instructions = Vec::new();
            for s in body {
                body_instructions.push(lower_statement(s, registry)?);
            }
            Ok(IrInstruction::Repeat {
                count: count_expr,
                body: body_instructions,
            })
        }
        Statement::Return { value, .. } => {
            let expr = match value {
                Some(v) => Some(lower_expr(v, registry)?),
                None => None,
            };
            Ok(IrInstruction::Return(expr))
        }
        Statement::Assert { expr, .. } => {
            let lowered = lower_expr(expr, registry)?;
            Ok(IrInstruction::Assert(lowered))
        }
    }
}

fn lower_expr(expr: &Expr, registry: &BlockRegistry) -> Result<IrExpr, LowerError> {
    match expr {
        Expr::Number(n, _) => Ok(IrExpr::Constant(IrValue::Number(*n))),
        Expr::String(s, _) => Ok(IrExpr::Constant(IrValue::String(s.clone()))),
        Expr::Bool(b, _) => Ok(IrExpr::Constant(IrValue::Bool(*b))),
        Expr::Ident(name, _) => Ok(IrExpr::Var(name.clone())),
        Expr::Binary { left, op, right, .. } => {
            let lowered_left = lower_expr(left, registry)?;
            let lowered_right = lower_expr(right, registry)?;
            let ir_op = match op {
                BinaryOp::Add => IrBinaryOp::Add,
                BinaryOp::Sub => IrBinaryOp::Sub,
                BinaryOp::Mul => IrBinaryOp::Mul,
                BinaryOp::Div => IrBinaryOp::Div,
                BinaryOp::Mod => IrBinaryOp::Mod,
                BinaryOp::Equal => IrBinaryOp::Equal,
                BinaryOp::NotEqual => IrBinaryOp::NotEqual,
                BinaryOp::Less => IrBinaryOp::Less,
                BinaryOp::LessEqual => IrBinaryOp::LessEqual,
                BinaryOp::Greater => IrBinaryOp::Greater,
                BinaryOp::GreaterEqual => IrBinaryOp::GreaterEqual,
                BinaryOp::And => IrBinaryOp::And,
                BinaryOp::Or => IrBinaryOp::Or,
            };
            Ok(IrExpr::Binary {
                left: Box::new(lowered_left),
                op: ir_op,
                right: Box::new(lowered_right),
            })
        }
        Expr::Unary { op, expr, .. } => {
            let lowered = lower_expr(expr, registry)?;
            let ir_op = match op {
                UnaryOp::Not => IrUnaryOp::Not,
                UnaryOp::Neg => IrUnaryOp::Neg,
            };
            Ok(IrExpr::Unary {
                op: ir_op,
                expr: Box::new(lowered),
            })
        }
        Expr::Call { target, name, args, .. } => {
            let full_name = match target {
                Some(t) => format!("{}.{}", t, name),
                None => name.clone(),
            };
            let mut lowered_args = Vec::new();
            for arg in args {
                lowered_args.push(lower_expr(arg, registry)?);
            }
            Ok(IrExpr::CallBlock {
                name: full_name,
                args: lowered_args,
            })
        }
    }
}
