use crate::protocol::{
    CompletionItem, CompletionItemKind, Diagnostic as LspDiagnostic, DiagnosticSeverity,
    DocumentSymbol, Hover, InsertTextFormat, MarkupContent, Position, Range, SymbolKind, TextEdit,
};
use scratch_blocks::{BlockRegistry, BlockType};
use scratch_language::ast::EventKind;
use scratch_language::linter::Linter;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Document {
    pub uri: String,
    pub version: i64,
    pub text: String,
}

impl Document {
    pub fn new(uri: String, version: i64, text: String) -> Self {
        Self { uri, version, text }
    }

    pub fn get_line(&self, line_index: usize) -> Option<&str> {
        self.text.lines().nth(line_index)
    }

    pub fn line_count(&self) -> usize {
        self.text.lines().count().max(1)
    }

    pub fn last_line_len(&self) -> usize {
        self.text.lines().last().map(|l| l.len()).unwrap_or(0)
    }
}

pub struct Analyzer {
    registry: BlockRegistry,
    known_objects: HashSet<String>,
    known_actions: HashSet<String>,
    known_assets: HashSet<String>,
    known_scenes: HashSet<String>,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer {
    pub fn new() -> Self {
        let registry = BlockRegistry::core();
        let mut known_objects = HashSet::new();
        known_objects.insert("Player".to_string());
        known_objects.insert("Enemy".to_string());
        known_objects.insert("Coin".to_string());
        known_objects.insert("Hazard".to_string());
        known_objects.insert("Ground".to_string());
        known_objects.insert("Door".to_string());
        known_objects.insert("Goal".to_string());

        let mut known_actions = HashSet::new();
        known_actions.insert("left".to_string());
        known_actions.insert("right".to_string());
        known_actions.insert("up".to_string());
        known_actions.insert("down".to_string());
        known_actions.insert("jump".to_string());
        known_actions.insert("action".to_string());

        let mut known_assets = HashSet::new();
        known_assets.insert("forest".to_string());
        known_assets.insert("dungeon".to_string());
        known_assets.insert("sky".to_string());
        known_assets.insert("coin".to_string());
        known_assets.insert("jump".to_string());
        known_assets.insert("hit".to_string());

        let mut known_scenes = HashSet::new();
        known_scenes.insert("main".to_string());
        known_scenes.insert("level1".to_string());
        known_scenes.insert("level2".to_string());
        known_scenes.insert("gameover".to_string());

        Self {
            registry,
            known_objects,
            known_actions,
            known_assets,
            known_scenes,
        }
    }

    pub fn set_known_actions(&mut self, actions: HashSet<String>) {
        self.known_actions = actions;
    }

    pub fn set_known_assets(&mut self, assets: HashSet<String>) {
        self.known_assets = assets;
    }

    pub fn set_known_scenes(&mut self, scenes: HashSet<String>) {
        self.known_scenes = scenes;
    }

    pub fn set_known_objects(&mut self, objects: HashSet<String>) {
        self.known_objects = objects;
    }

    /// Computes LSP diagnostics from scratch-language parser and linter.
    pub fn compute_diagnostics(&self, doc: &Document) -> Vec<LspDiagnostic> {
        let mut diagnostics = Vec::new();

        match scratch_language::parse(&doc.text) {
            Err(parse_err_msg) => {
                // Parse error message may contain line:col or fallback to line 0
                let (line, col) = parse_line_col_from_err(&parse_err_msg);
                diagnostics.push(LspDiagnostic {
                    range: Range {
                        start: Position { line, character: col },
                        end: Position { line, character: col + 4 },
                    },
                    severity: Some(DiagnosticSeverity::Error),
                    code: Some("SYNTAX".to_string()),
                    source: Some("scratch-lang".to_string()),
                    message: parse_err_msg,
                });
            }
            Ok(program) => {
                let mut linter = Linter::new(&self.registry)
                    .with_objects(self.known_objects.iter().cloned());
                let lint_items = linter.lint_program(&program);

                for item in lint_items {
                    let lsp_line = item.span.line.saturating_sub(1) as u32;
                    let lsp_col = item.span.col.saturating_sub(1) as u32;
                    let width = if item.span.end > item.span.start {
                        (item.span.end - item.span.start) as u32
                    } else {
                        4
                    };

                    let mut msg = item.message.clone();
                    if let Some(sug) = &item.suggestion {
                        msg.push_str(&format!("\n\n💡 Suggestion: Did you mean '{}'?", sug));
                    }

                    diagnostics.push(LspDiagnostic {
                        range: Range {
                            start: Position { line: lsp_line, character: lsp_col },
                            end: Position { line: lsp_line, character: lsp_col + width },
                        },
                        severity: Some(DiagnosticSeverity::Warning),
                        code: Some(item.code),
                        source: Some("scratch-linter".to_string()),
                        message: msg,
                    });
                }
            }
        }

        diagnostics
    }

    /// Computes autocompletion items based on current document and cursor position.
    pub fn compute_completions(&self, doc: &Document, pos: Position) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let line_text = doc.get_line(pos.line as usize).unwrap_or("");
        let prefix = if (pos.character as usize) <= line_text.len() {
            &line_text[..pos.character as usize]
        } else {
            line_text
        };
        let trimmed_prefix = prefix.trim_start();

        // 1. Inside string quotes context
        if trimmed_prefix.contains("(\"") || trimmed_prefix.ends_with('"') {
            if trimmed_prefix.contains("action.") {
                for action in &["left", "right", "up", "down", "jump", "action"] {
                    items.push(CompletionItem {
                        label: (*action).to_string(),
                        kind: Some(CompletionItemKind::Value),
                        detail: Some("Input Action".to_string()),
                        documentation: Some(MarkupContent::plaintext(format!("Physical key mapping for '{}'", action))),
                        insert_text: Some((*action).to_string()),
                        insert_text_format: Some(InsertTextFormat::PlainText),
                    });
                }
                return items;
            }

            if trimmed_prefix.contains("scene.switch") {
                for sc in &self.known_scenes {
                    items.push(CompletionItem {
                        label: sc.clone(),
                        kind: Some(CompletionItemKind::File),
                        detail: Some("Game Scene".to_string()),
                        documentation: Some(MarkupContent::plaintext(format!("Switch to scene '{}'", sc))),
                        insert_text: Some(sc.clone()),
                        insert_text_format: Some(InsertTextFormat::PlainText),
                    });
                }
                return items;
            }

            if trimmed_prefix.contains("sound.play") || trimmed_prefix.contains("background.set") {
                for asset in &self.known_assets {
                    items.push(CompletionItem {
                        label: asset.clone(),
                        kind: Some(CompletionItemKind::File),
                        detail: Some("Indexed Asset".to_string()),
                        documentation: Some(MarkupContent::plaintext(format!("Asset '{}'", asset))),
                        insert_text: Some(asset.clone()),
                        insert_text_format: Some(InsertTextFormat::PlainText),
                    });
                }
                return items;
            }
        }

        // 2. Event headers: at beginning of line or following 'when '
        if trimmed_prefix.is_empty() || trimmed_prefix == "when" || trimmed_prefix == "when " {
            let event_snippets = [
                ("when start:", "when start:\n    ${0}", "Runs once when the game or scene starts"),
                ("when update:", "when update:\n    ${0}", "Runs every single frame (60 FPS)"),
                ("when action.down(\"right\"):", "when action.down(\"${1:right}\"):\n    ${0}", "Triggers while the specified input action is held down"),
                ("when action.press(\"jump\"):", "when action.press(\"${1:jump}\"):\n    ${0}", "Triggers once when the specified input action is pressed"),
                ("when action.up(\"jump\"):", "when action.up(\"${1:jump}\"):\n    ${0}", "Triggers once when the specified input action is released"),
                ("when Player touches Enemy:", "when ${1:Player} touches ${2:Enemy}:\n    ${0}", "Triggers when two objects collide"),
                ("every 1 seconds:", "every ${1:1} seconds:\n    ${0}", "Repeats periodically at fixed time intervals"),
                ("after 2 seconds:", "after ${1:2} seconds:\n    ${0}", "Triggers once after a specified duration"),
            ];

            for (label, snippet, doc_str) in event_snippets {
                items.push(CompletionItem {
                    label: label.to_string(),
                    kind: Some(CompletionItemKind::Event),
                    detail: Some("Event Trigger".to_string()),
                    documentation: Some(MarkupContent::markdown(doc_str)),
                    insert_text: Some(snippet.to_string()),
                    insert_text_format: Some(InsertTextFormat::Snippet),
                });
            }

            if trimmed_prefix.is_empty() {
                items.push(CompletionItem {
                    label: "when".to_string(),
                    kind: Some(CompletionItemKind::Keyword),
                    detail: Some("Keyword".to_string()),
                    documentation: Some(MarkupContent::plaintext("Defines a game event handler")),
                    insert_text: Some("when ".to_string()),
                    insert_text_format: Some(InsertTextFormat::PlainText),
                });
            }
        }

        // 3. Built-in blocks from BlockRegistry
        for block in self.registry.iter() {
            let mut params_snippet = Vec::new();
            let mut param_types = Vec::new();
            for (idx, p) in block.parameters.iter().enumerate() {
                param_types.push(format!("{}: {:?}", p.name, p.param_type));
                let default_val = match p.param_type {
                    BlockType::Object => "Player",
                    BlockType::Number => "5",
                    BlockType::String => "\"sound\"",
                    BlockType::Boolean => "true",
                    _ => "0",
                };
                params_snippet.push(format!("${{{}:{}}}", idx + 1, default_val));
            }

            let snippet = format!("{}({})", block.name, params_snippet.join(", "));
            let signature = format!("{}({})", block.name, param_types.join(", "));

            let doc_md = format!(
                "### `{}`\n**Category**: {:?}\n\n{}\n\n#### Examples\n```sch\n{}\n```",
                signature,
                block.category,
                block.description,
                block.examples.join("\n")
            );

            items.push(CompletionItem {
                label: block.name.clone(),
                kind: Some(CompletionItemKind::Function),
                detail: Some(signature),
                documentation: Some(MarkupContent::markdown(doc_md)),
                insert_text: Some(snippet),
                insert_text_format: Some(InsertTextFormat::Snippet),
            });
        }

        // 4. Control flow keywords
        items.push(CompletionItem {
            label: "if".to_string(),
            kind: Some(CompletionItemKind::Keyword),
            detail: Some("Conditional block".to_string()),
            documentation: Some(MarkupContent::plaintext("Executes statements when condition is true")),
            insert_text: Some("if ${1:condition}:\n    ${0}".to_string()),
            insert_text_format: Some(InsertTextFormat::Snippet),
        });

        items.push(CompletionItem {
            label: "repeat".to_string(),
            kind: Some(CompletionItemKind::Keyword),
            detail: Some("Loop block".to_string()),
            documentation: Some(MarkupContent::plaintext("Repeats statements N times")),
            insert_text: Some("repeat ${1:5}:\n    ${0}".to_string()),
            insert_text_format: Some(InsertTextFormat::Snippet),
        });

        items.push(CompletionItem {
            label: "return".to_string(),
            kind: Some(CompletionItemKind::Keyword),
            detail: Some("Return keyword".to_string()),
            documentation: Some(MarkupContent::plaintext("Exits early from event handler")),
            insert_text: Some("return".to_string()),
            insert_text_format: Some(InsertTextFormat::PlainText),
        });

        // 5. Known game objects
        for obj in &self.known_objects {
            items.push(CompletionItem {
                label: obj.clone(),
                kind: Some(CompletionItemKind::Class),
                detail: Some("Game Object".to_string()),
                documentation: Some(MarkupContent::plaintext(format!("Active scene entity '{}'", obj))),
                insert_text: Some(obj.clone()),
                insert_text_format: Some(InsertTextFormat::PlainText),
            });
        }

        items
    }

    /// Computes rich Hover card for identifier under cursor.
    pub fn compute_hover(&self, doc: &Document, pos: Position) -> Option<Hover> {
        let line = doc.get_line(pos.line as usize)?;
        let word = extract_word_at_pos(line, pos.character as usize)?;
        let full_name = extract_full_identifier_at_pos(line, pos.character as usize).unwrap_or_else(|| word.to_string());

        if let Some(block) = self.registry.get(&full_name) {
            let param_list: Vec<String> = block
                .parameters
                .iter()
                .map(|p| format!("{}: {:?}", p.name, p.param_type))
                .collect();

            let explanation = if !block.documentation.is_empty() {
                format!("{}\n\n{}", block.description, block.documentation)
            } else {
                block.description.clone()
            };

            let doc_md = format!(
                "### `{}({}) -> {:?}`\n**Category**: {:?}\n\n{}\n\n#### Examples\n```sch\n{}\n```",
                block.name,
                param_list.join(", "),
                block.return_type,
                block.category,
                explanation,
                block.examples.join("\n")
            );

            return Some(Hover {
                contents: MarkupContent::markdown(doc_md),
                range: None,
            });
        }

        // Keywords & events
        match word {
            "when" => Some(Hover {
                contents: MarkupContent::markdown(
                    "### `when <event>:`\nDefines a top-level event handler that executes whenever the specified trigger fires.\n\n**Supported Triggers**:\n- `when start:`\n- `when update:`\n- `when action.down(\"...\"):`\n- `when action.press(\"...\"):`\n- `when action.up(\"...\"):`\n- `when <ObjectA> touches <ObjectB>:`\n- `every <N> seconds:`\n- `after <N> seconds:`",
                ),
                range: None,
            }),
            "if" => Some(Hover {
                contents: MarkupContent::markdown(
                    "### `if <condition>:`\nEvaluates condition expression. If true, runs the indented block of code.\n\n#### Example\n```sch\nif health <= 0:\n    respawn(Player)\n```",
                ),
                range: None,
            }),
            "repeat" => Some(Hover {
                contents: MarkupContent::markdown(
                    "### `repeat <count>:`\nExecutes the indented block a positive integer number of times.",
                ),
                range: None,
            }),
            "touches" => Some(Hover {
                contents: MarkupContent::markdown(
                    "### `<ObjectA> touches <ObjectB>`\nCollision condition checking if two game entities overlap in the current frame.",
                ),
                range: None,
            }),
            "start" => Some(Hover {
                contents: MarkupContent::markdown(
                    "### `when start:`\nLifecycle event that fires once when the game boots or a scene loads.",
                ),
                range: None,
            }),
            "update" => Some(Hover {
                contents: MarkupContent::markdown(
                    "### `when update:`\nLifecycle event that runs once per frame (60 FPS) for animations or continuous calculations.",
                ),
                range: None,
            }),
            _ => {
                if self.known_objects.contains(word) {
                    Some(Hover {
                        contents: MarkupContent::markdown(format!(
                            "### Game Object: `{}`\nActive entity instantiated in the active scene.",
                            word
                        )),
                        range: None,
                    })
                } else {
                    None
                }
            }
        }
    }

    /// Formats the document using scratch-language deterministic formatter.
    pub fn format_document(&self, doc: &Document) -> Option<Vec<TextEdit>> {
        let formatted = match scratch_language::format_source(&doc.text) {
            Ok(s) => s,
            Err(_) => return None,
        };

        if formatted == doc.text {
            return Some(Vec::new());
        }

        let lines = doc.text.lines().collect::<Vec<&str>>();
        let end_line = lines.len().saturating_sub(1) as u32;
        let end_char = lines.last().map(|l| l.len()).unwrap_or(0) as u32;

        Some(vec![TextEdit {
            range: Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: end_line, character: end_char },
            },
            new_text: formatted,
        }])
    }

    /// Extracts DocumentSymbols for an outline view of the game code.
    pub fn compute_document_symbols(&self, doc: &Document) -> Vec<DocumentSymbol> {
        let mut symbols = Vec::new();

        if let Ok(program) = scratch_language::parse(&doc.text) {
            for event in program.events {
                let name = match &event.kind {
                    EventKind::Start => "when start".to_string(),
                    EventKind::Update => "when update".to_string(),
                    EventKind::ActionDown(act) => format!("when action.down(\"{}\")", act),
                    EventKind::ActionPress(act) => format!("when action.press(\"{}\")", act),
                    EventKind::ActionUp(act) => format!("when action.up(\"{}\")", act),
                    EventKind::Touches { object_a, object_b } => {
                        format!("when {} touches {}", object_a, object_b)
                    }
                    EventKind::EverySeconds(s) => format!("every {} seconds", s),
                    EventKind::AfterSeconds(s) => format!("after {} seconds", s),
                    EventKind::Custom(c) => format!("when custom(\"{}\")", c),
                };

                let lsp_start_line = event.span.line.saturating_sub(1) as u32;
                let lsp_start_col = event.span.col.saturating_sub(1) as u32;

                let range = Range {
                    start: Position { line: lsp_start_line, character: lsp_start_col },
                    end: Position { line: lsp_start_line + 5, character: 0 },
                };

                symbols.push(DocumentSymbol {
                    name,
                    detail: Some(format!("{} statements", event.body.len())),
                    kind: SymbolKind::Event,
                    range,
                    selection_range: Range {
                        start: Position { line: lsp_start_line, character: lsp_start_col },
                        end: Position { line: lsp_start_line, character: lsp_start_col + 10 },
                    },
                    children: None,
                });
            }
        }

        symbols
    }
}

fn parse_line_col_from_err(err_msg: &str) -> (u32, u32) {
    if let Some(pos) = err_msg.find("line ") {
        let rest = &err_msg[pos + 5..];
        if let Some(colon) = rest.find(':') {
            let line_part = &rest[..colon];
            let after_colon = &rest[colon + 1..];
            let end_col = after_colon.find(|c: char| !c.is_ascii_digit()).unwrap_or(after_colon.len());
            let col_part = &after_colon[..end_col];

            let line = line_part.parse::<u32>().unwrap_or(1).saturating_sub(1);
            let col = col_part.parse::<u32>().unwrap_or(1).saturating_sub(1);
            return (line, col);
        }
    }
    (0, 0)
}

fn extract_word_at_pos(line: &str, char_idx: usize) -> Option<&str> {
    if line.is_empty() {
        return None;
    }
    let idx = char_idx.min(line.len().saturating_sub(1));
    let bytes = line.as_bytes();

    if !is_word_byte(bytes[idx]) && idx > 0 && is_word_byte(bytes[idx - 1]) {
        return extract_word_around(line, idx - 1);
    }
    if is_word_byte(bytes[idx]) {
        return extract_word_around(line, idx);
    }
    None
}

fn extract_word_around(line: &str, idx: usize) -> Option<&str> {
    let bytes = line.as_bytes();
    let mut start = idx;
    while start > 0 && is_word_byte(bytes[start - 1]) {
        start -= 1;
    }
    let mut end = idx;
    while end < bytes.len() && is_word_byte(bytes[end]) {
        end += 1;
    }
    if start < end {
        Some(&line[start..end])
    } else {
        None
    }
}

fn extract_full_identifier_at_pos(line: &str, char_idx: usize) -> Option<String> {
    if line.is_empty() {
        return None;
    }
    let idx = char_idx.min(line.len().saturating_sub(1));
    let bytes = line.as_bytes();

    let is_id_char = |b: u8| is_word_byte(b) || b == b'.';

    let target = if is_id_char(bytes[idx]) {
        idx
    } else if idx > 0 && is_id_char(bytes[idx - 1]) {
        idx - 1
    } else {
        return None;
    };

    let mut start = target;
    while start > 0 && is_id_char(bytes[start - 1]) {
        start -= 1;
    }
    let mut end = target;
    while end < bytes.len() && is_id_char(bytes[end]) {
        end += 1;
    }

    if start < end {
        Some(line[start..end].to_string())
    } else {
        None
    }
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
