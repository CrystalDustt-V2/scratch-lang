pub mod index;
pub mod types;

pub use index::AssetIndex;
pub use types::{AssetCategory, AssetInfo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_indexing_and_lookup() {
        let mut index = AssetIndex::new();
        index.register(AssetInfo {
            name: "coin".into(),
            category: AssetCategory::Sound,
            relative_path: "assets/sounds/coin.wav".into(),
            extension: "wav".into(),
        });

        assert!(index.has_sound("coin"));
        assert!(!index.has_sprite("coin"));
        assert_eq!(index.find_similar("coyn"), Some("coin"));
    }
}
