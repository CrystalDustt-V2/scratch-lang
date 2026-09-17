use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneCamera {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default = "default_zoom")]
    pub zoom: f32,
    pub follow: Option<String>,
}

fn default_zoom() -> f32 {
    1.0
}

impl Default for SceneCamera {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            follow: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneObject {
    pub name: String,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default = "default_size")]
    pub size: [f32; 2],
    #[serde(default = "default_color")]
    pub color: [f32; 4],
    #[serde(default)]
    pub tags: Vec<String>,
    pub sprite: Option<String>,
}

fn default_size() -> [f32; 2] {
    [40.0, 40.0]
}

fn default_color() -> [f32; 4] {
    [0.2, 0.6, 1.0, 1.0]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneData {
    pub name: String,
    #[serde(default = "default_background")]
    pub background: String,
    #[serde(default)]
    pub camera: SceneCamera,
    #[serde(default)]
    pub objects: Vec<SceneObject>,
}

fn default_background() -> String {
    "white".to_string()
}

impl Default for SceneData {
    fn default() -> Self {
        Self {
            name: "Main".to_string(),
            background: "white".to_string(),
            camera: SceneCamera::default(),
            objects: vec![SceneObject {
                name: "Player".to_string(),
                x: 0.0,
                y: 0.0,
                size: [40.0, 40.0],
                color: [0.2, 0.6, 1.0, 1.0],
                tags: vec!["player".to_string()],
                sprite: None,
            }],
        }
    }
}

#[derive(Debug, Error)]
pub enum SceneError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Scene file not found: {0}")]
    NotFound(String),
}

impl SceneData {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, SceneError> {
        let content = std::fs::read_to_string(path)?;
        let scene: SceneData = serde_yaml::from_str(&content)?;
        Ok(scene)
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), SceneError> {
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }
}
