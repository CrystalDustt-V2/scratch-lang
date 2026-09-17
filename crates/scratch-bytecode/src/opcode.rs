use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BytecodeValue {
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
    Object(String),
    List(Vec<BytecodeValue>),
}

impl std::fmt::Display for BytecodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeValue::Nil => write!(f, "nil"),
            BytecodeValue::Number(n) => write!(f, "{}", n),
            BytecodeValue::String(s) => write!(f, "\"{}\"", s),
            BytecodeValue::Bool(b) => write!(f, "{}", b),
            BytecodeValue::Object(o) => write!(f, "@{}", o),
            BytecodeValue::List(l) => {
                let items: Vec<String> = l.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", items.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OpCode {
    /// Push a constant value onto the stack by index in chunk's constant pool
    PushConst(usize),
    /// Push nil onto the stack
    PushNil,
    /// Push true onto the stack
    PushTrue,
    /// Push false onto the stack
    PushFalse,

    /// Load variable by name onto the stack
    LoadVar(String),
    /// Store top of stack into variable by name
    StoreVar(String),

    /// Pop top of stack
    Pop,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Comparison & Logic
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    Neg,

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),

    // Invocations
    CallBlock { name: String, arg_count: usize },
    CallFunc { name: String, arg_count: usize },
    Return,
    Assert,
    Halt,
}
