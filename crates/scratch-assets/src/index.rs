use crate::types::{AssetCategory, AssetInfo};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct AssetIndex {
    assets: HashMap<String, AssetInfo>,
}

impl AssetIndex {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn register(&mut self, info: AssetInfo) {
        self.assets.insert(info.name.clone(), info);
    }

    pub fn get(&self, name: &str) -> Option<&AssetInfo> {
        self.assets.get(name)
    }

    pub fn has_category(&self, name: &str, category: AssetCategory) -> bool {
        self.assets.get(name).map_or(false, |a| a.category == category)
    }

    pub fn has_sound(&self, name: &str) -> bool {
        self.has_category(name, AssetCategory::Sound) || self.has_category(name, AssetCategory::Music)
    }

    pub fn has_sprite(&self, name: &str) -> bool {
        self.has_category(name, AssetCategory::Sprite)
    }

    pub fn has_background(&self, name: &str) -> bool {
        self.has_category(name, AssetCategory::Background)
    }

    pub fn scan_directory(&mut self, root_dir: impl AsRef<Path>) {
        let root = root_dir.as_ref();
        if !root.exists() {
            return;
        }

        self.scan_category_dir(&root.join("sprites"), AssetCategory::Sprite);
        self.scan_category_dir(&root.join("animations"), AssetCategory::Animation);
        self.scan_category_dir(&root.join("backgrounds"), AssetCategory::Background);
        self.scan_category_dir(&root.join("sounds"), AssetCategory::Sound);
        self.scan_category_dir(&root.join("music"), AssetCategory::Music);
        self.scan_category_dir(&root.join("fonts"), AssetCategory::Font);
    }

    fn scan_category_dir(&mut self, dir: &Path, category: AssetCategory) {
        if !dir.is_dir() {
            return;
        }

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_string();
                        self.register(AssetInfo {
                            name: stem.to_string(),
                            category,
                            relative_path: path.clone(),
                            extension: ext,
                        });
                    }
                }
            }
        }
    }

    pub fn find_similar(&self, query: &str) -> Option<&str> {
        let mut closest = None;
        let mut min_dist = usize::MAX;

        for name in self.assets.keys() {
            let dist = levenshtein(query, name);
            if dist < min_dist && dist <= 3 {
                min_dist = dist;
                closest = Some(name.as_str());
            }
        }

        closest
    }

    pub fn iter(&self) -> impl Iterator<Item = &AssetInfo> {
        self.assets.values()
    }
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
