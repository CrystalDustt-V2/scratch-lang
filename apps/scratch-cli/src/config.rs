use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub default_author: String,
    pub default_template: String,
    pub preview_mode: String,
    pub editor: String,
    pub studio_port: u16,
    pub log_level: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            default_author: whoami(),
            default_template: "starter".to_string(),
            preview_mode: "popup".to_string(),
            editor: "code".to_string(),
            studio_port: 8080,
            log_level: "info".to_string(),
        }
    }
}

fn whoami() -> String {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "scratch_developer".to_string())
}

impl CliConfig {
    pub fn config_dir() -> PathBuf {
        let home = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".scratch")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str(&content) {
                    return cfg;
                }
            }
        }
        let cfg = Self::default();
        let _ = cfg.save();
        cfg
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dir = Self::config_dir();
        std::fs::create_dir_all(&dir)?;
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(Self::config_path(), json)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key.to_lowercase().as_str() {
            "default_author" | "author" => Some(self.default_author.clone()),
            "default_template" | "template" => Some(self.default_template.clone()),
            "preview_mode" | "preview" => Some(self.preview_mode.clone()),
            "editor" => Some(self.editor.clone()),
            "studio_port" | "port" => Some(self.studio_port.to_string()),
            "log_level" | "log" => Some(self.log_level.clone()),
            _ => None,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key.to_lowercase().as_str() {
            "default_author" | "author" => self.default_author = value.to_string(),
            "default_template" | "template" => {
                let valid = ["starter", "platformer", "coins", "dialogue", "motion", "lists"];
                if !valid.contains(&value.to_lowercase().as_str()) {
                    return Err(format!("Invalid template '{}'. Valid options: {}", value, valid.join(", ")));
                }
                self.default_template = value.to_lowercase();
            }
            "preview_mode" | "preview" => {
                let valid = ["popup", "window", "headless"];
                if !valid.contains(&value.to_lowercase().as_str()) {
                    return Err(format!("Invalid preview mode '{}'. Valid options: {}", value, valid.join(", ")));
                }
                self.preview_mode = value.to_lowercase();
            }
            "editor" => self.editor = value.to_string(),
            "studio_port" | "port" => {
                let p: u16 = value.parse().map_err(|_| "Port must be a valid number (1-65535)")?;
                self.studio_port = p;
            }
            "log_level" | "log" => self.log_level = value.to_string(),
            _ => return Err(format!("Unknown configuration key '{}'", key)),
        }
        self.save().map_err(|e| format!("Failed to save config: {}", e))
    }

    pub fn list(&self) {
        println!("Scratch CLI Configuration ({})", Self::config_path().display());
        println!("{:-<60}", "");
        println!("{:<20} {:<18} {}", "KEY", "VALUE", "DESCRIPTION");
        println!("{:-<60}", "");
        println!("{:<20} {:<18} Default author name for new projects", "default_author", self.default_author);
        println!("{:<20} {:<18} Default project starter template", "default_template", self.default_template);
        println!("{:<20} {:<18} Preview mode for 'scratch run' (popup/headless)", "preview_mode", self.preview_mode);
        println!("{:<20} {:<18} Preferred editor command (e.g. code, notepad)", "editor", self.editor);
        println!("{:<20} {:<18} Default port for Scratch Studio server", "studio_port", self.studio_port);
        println!("{:<20} {:<18} Terminal logging verbosity (info/debug/warn)", "log_level", self.log_level);
        println!("{:-<60}", "");
    }

    pub fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        *self = Self::default();
        self.save()?;
        Ok(())
    }
}
