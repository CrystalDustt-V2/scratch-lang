pub mod block;
pub mod registry;
pub mod types;

pub use block::{AutocompleteMetadata, BlockDefinition, LintMetadata, ParameterDefinition};
pub use registry::BlockRegistry;
pub use types::{BlockCategory, BlockType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_registry_contains_primitives() {
        let registry = BlockRegistry::core();
        assert!(registry.contains("move"));
        assert!(registry.contains("jump"));
        assert!(registry.contains("touching"));
        assert!(registry.contains("damage"));
        assert!(registry.contains("sound.play"));

        let move_def = registry.get("move").expect("move block exists");
        assert_eq!(move_def.category, BlockCategory::Movement);
        assert_eq!(move_def.parameters.len(), 3);
        assert_eq!(move_def.parameters[0].name, "target");
        assert_eq!(move_def.parameters[0].param_type, BlockType::Object);
    }

    #[test]
    fn test_did_you_mean_suggestion() {
        let registry = BlockRegistry::core();
        assert_eq!(registry.find_similar("moov"), Some("move"));
        assert_eq!(registry.find_similar("jmp"), Some("jump"));
    }
}
