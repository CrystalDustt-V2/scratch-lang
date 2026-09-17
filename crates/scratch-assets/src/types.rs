use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetCategory {
    Sprite,
    Animation,
    Background,
    Sound,
    Music,
    Font,
}

impl std::fmt::Display for AssetCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetCategory::Sprite => write!(f, "sprites"),
            AssetCategory::Animation => write!(f, "animations"),
            AssetCategory::Background => write!(f, "backgrounds"),
            AssetCategory::Sound => write!(f, "sounds"),
            AssetCategory::Music => write!(f, "music"),
            AssetCategory::Font => write!(f, "fonts"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetInfo {
    pub name: String,
    pub category: AssetCategory,
    pub relative_path: PathBuf,
    pub extension: String,
}
