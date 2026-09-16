use serde::{Deserialize, Serialize};

/// Supported scratch-lang value and parameter types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockType {
    Void,
    Number,
    String,
    Boolean,
    Object,
    List(Box<BlockType>),
    Any,
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockType::Void => write!(f, "Void"),
            BlockType::Number => write!(f, "Number"),
            BlockType::String => write!(f, "String"),
            BlockType::Boolean => write!(f, "Boolean"),
            BlockType::Object => write!(f, "Object"),
            BlockType::List(inner) => write!(f, "List<{}>", inner),
            BlockType::Any => write!(f, "Any"),
        }
    }
}

/// Category grouping for block organisation and visual representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockCategory {
    Events,
    Movement,
    Gameplay,
    Condition,
    Audio,
    Animation,
    Camera,
    Scene,
    Variables,
    Control,
    Custom,
}

impl std::fmt::Display for BlockCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockCategory::Events => write!(f, "Events"),
            BlockCategory::Movement => write!(f, "Movement"),
            BlockCategory::Gameplay => write!(f, "Gameplay"),
            BlockCategory::Condition => write!(f, "Condition"),
            BlockCategory::Audio => write!(f, "Audio"),
            BlockCategory::Animation => write!(f, "Animation"),
            BlockCategory::Camera => write!(f, "Camera"),
            BlockCategory::Scene => write!(f, "Scene"),
            BlockCategory::Variables => write!(f, "Variables"),
            BlockCategory::Control => write!(f, "Control"),
            BlockCategory::Custom => write!(f, "Custom"),
        }
    }
}
