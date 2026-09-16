use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrProgram {
    pub events: Vec<IrEventHandler>,
    pub functions: Vec<IrFunction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrEventHandler {
    pub trigger: IrTrigger,
    pub instructions: Vec<IrInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrTrigger {
    OnStart,
    OnUpdate,
    OnActionDown(String),
    OnActionPress(String),
    OnActionUp(String),
    OnTouches { object_a: String, object_b: String },
    OnTimer { seconds: u64, repeating: bool },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<String>,
    pub instructions: Vec<IrInstruction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrInstruction {
    Assign {
        target: String,
        expr: IrExpr,
    },
    CallBlock {
        name: String,
        args: Vec<IrExpr>,
    },
    If {
        condition: IrExpr,
        then_branch: Vec<IrInstruction>,
        else_branch: Vec<IrInstruction>,
    },
    Repeat {
        count: IrExpr,
        body: Vec<IrInstruction>,
    },
    Return(Option<IrExpr>),
    Assert(IrExpr),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrExpr {
    Constant(IrValue),
    Var(String),
    Binary {
        left: Box<IrExpr>,
        op: IrBinaryOp,
        right: Box<IrExpr>,
    },
    Unary {
        op: IrUnaryOp,
        expr: Box<IrExpr>,
    },
    CallBlock {
        name: String,
        args: Vec<IrExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrValue {
    Number(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrUnaryOp {
    Not,
    Neg,
}
