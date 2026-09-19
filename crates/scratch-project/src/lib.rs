pub mod ratio;
pub use ratio::*;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub entry: String,
    pub resolution: ResolutionConfig,
    #[serde(default = "default_background")]
    pub background: String,
    #[serde(default = "default_ratio")]
    pub ratio: String,
}

fn default_background() -> String {
    "white".to_string()
}

fn default_ratio() -> String {
    "16:9".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionConfig {
    pub width: u32,
    pub height: u32,
}

impl Default for ResolutionConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "My Game".to_string(),
            entry: "src/main.sch".to_string(),
            resolution: ResolutionConfig::default(),
            background: "white".to_string(),
            ratio: "16:9".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Project file not found at {0}")]
    NotFound(PathBuf),
}

impl ProjectConfig {
    pub fn apply_ratio_preset(&mut self, preset: AspectRatioPreset) {
        let (w, h) = preset.to_resolution();
        self.resolution.width = w;
        self.resolution.height = h;
        self.ratio = preset.short_label();
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, ProjectError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(ProjectError::NotFound(path.to_path_buf()));
        }
        let content = std::fs::read_to_string(path)?;
        let config: ProjectConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), ProjectError> {
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_project_config() {
        let config = ProjectConfig::default();
        assert_eq!(config.name, "My Game");
        assert_eq!(config.resolution.width, 1280);
        assert_eq!(config.resolution.height, 720);
        assert_eq!(config.entry, "src/main.sch");
    }

    #[test]
    fn test_yaml_roundtrip() {
        let config = ProjectConfig::default();
        let yaml = serde_yaml::to_string(&config).expect("serialize ok");
        let parsed: ProjectConfig = serde_yaml::from_str(&yaml).expect("deserialize ok");
        assert_eq!(parsed.name, config.name);
        assert_eq!(parsed.resolution.width, 1280);
    }
}
