use crate::ast::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
    pub span: Span,
    pub suggestion: Option<String>,
}

impl Diagnostic {
    pub fn new(code: impl Into<String>, message: impl Into<String>, span: Span) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            span,
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Renders a beginner-friendly educational error message.
    pub fn render(&self, source: &str) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let mut out = format!("Error [{}]: {}\n", self.code, self.message);

        if self.span.line > 0 && self.span.line <= lines.len() {
            let line_content = lines[self.span.line - 1];
            out.push_str(&format!("  --> Line {}:{}\n", self.span.line, self.span.col));
            out.push_str(&format!("   |\n{:4} | {}\n   | ", self.span.line, line_content));

            let col_offset = if self.span.col > 0 { self.span.col - 1 } else { 0 };
            for _ in 0..col_offset {
                out.push(' ');
            }

            let width = if self.span.end > self.span.start {
                self.span.end - self.span.start
            } else {
                1
            };

            for _ in 0..width {
                out.push('^');
            }
            out.push('\n');
        }

        if let Some(suggestion) = &self.suggestion {
            out.push_str(&format!("\nHelp: Did you mean \"{}\"?\n", suggestion));
        }

        out
    }
}
