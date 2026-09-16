use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeValue {
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
    Object(String),
}

impl RuntimeValue {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            RuntimeValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            RuntimeValue::String(s) => Some(s),
            RuntimeValue::Object(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            RuntimeValue::Bool(b) => *b,
            RuntimeValue::Nil => false,
            RuntimeValue::Number(n) => *n != 0.0,
            RuntimeValue::String(s) => !s.is_empty(),
            RuntimeValue::Object(_) => true,
        }
    }
}

impl std::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Nil => write!(f, "nil"),
            RuntimeValue::Number(n) => write!(f, "{}", n),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Bool(b) => write!(f, "{}", b),
            RuntimeValue::Object(o) => write!(f, "{}", o),
        }
    }
}
