use crate::types::{BlockCategory, BlockType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub name: String,
    pub param_type: BlockType,
    pub optional: bool,
    pub default_value: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AutocompleteMetadata {
    pub snippet: String,
    pub trigger_characters: Vec<String>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LintMetadata {
    pub deprecated: bool,
    pub deprecation_reason: Option<String>,
    pub per_frame_warning: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockDefinition {
    pub name: String,
    pub category: BlockCategory,
    pub description: String,
    pub parameters: Vec<ParameterDefinition>,
    pub return_type: BlockType,
    pub documentation: String,
    pub examples: Vec<String>,
    pub autocomplete_metadata: AutocompleteMetadata,
    pub lint_metadata: LintMetadata,
    pub runtime_handler: String,
}

impl BlockDefinition {
    pub fn new(
        name: impl Into<String>,
        category: BlockCategory,
        description: impl Into<String>,
        return_type: BlockType,
        runtime_handler: impl Into<String>,
    ) -> Self {
        let name_str = name.into();
        Self {
            name: name_str.clone(),
            category,
            description: description.into(),
            parameters: Vec::new(),
            return_type,
            documentation: String::new(),
            examples: Vec::new(),
            autocomplete_metadata: AutocompleteMetadata {
                snippet: format!("{}($0)", name_str),
                trigger_characters: vec![],
                detail: String::new(),
            },
            lint_metadata: LintMetadata::default(),
            runtime_handler: runtime_handler.into(),
        }
    }

    pub fn with_param(
        mut self,
        name: impl Into<String>,
        param_type: BlockType,
        optional: bool,
        default_value: Option<&str>,
        description: impl Into<String>,
    ) -> Self {
        self.parameters.push(ParameterDefinition {
            name: name.into(),
            param_type,
            optional,
            default_value: default_value.map(|s| s.to_string()),
            description: description.into(),
        });
        self
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.documentation = doc.into();
        self
    }

    pub fn with_example(mut self, example: impl Into<String>) -> Self {
        self.examples.push(example.into());
        self
    }
}
