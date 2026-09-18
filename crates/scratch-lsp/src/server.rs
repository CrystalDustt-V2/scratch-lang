use crate::analysis::{Analyzer, Document};
use crate::protocol::{
    read_message, write_message, CompletionItem, Diagnostic, DocumentSymbol, Hover, Id,
    NotificationMessage, Position, PublishDiagnosticsParams, RequestMessage, ResponseError,
    ResponseMessage, TextEdit,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

pub struct LspServer {
    documents: HashMap<String, Document>,
    analyzer: Analyzer,
    is_shutdown: bool,
}

impl LspServer {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            analyzer: Analyzer::new(),
            is_shutdown: false,
        }
    }

    pub fn analyzer_mut(&mut self) -> &mut Analyzer {
        &mut self.analyzer
    }

    /// Runs the LSP JSON-RPC message processing loop on the provided reader and writer.
    pub fn run<R: BufRead, W: Write>(&mut self, mut reader: R, mut writer: W) -> io::Result<()> {
        while !self.is_shutdown {
            let message_opt = match read_message(&mut reader) {
                Ok(opt) => opt,
                Err(e) => {
                    if e.kind() == io::ErrorKind::UnexpectedEof {
                        break;
                    }
                    return Err(e);
                }
            };

            let raw_json = match message_opt {
                Some(s) => s,
                None => break, // EOF reached
            };

            let parsed: Value = match serde_json::from_str(&raw_json) {
                Ok(v) => v,
                Err(_) => {
                    let err_resp = ResponseMessage {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(ResponseError {
                            code: -32700,
                            message: "Parse error".to_string(),
                            data: None,
                        }),
                    };
                    write_message(&mut writer, &serde_json::to_string(&err_resp)?)?;
                    continue;
                }
            };

            if let Some(id_val) = parsed.get("id") {
                // Request
                let parsed_id = serde_json::from_value::<Id>(id_val.clone()).ok();
                match serde_json::from_value::<RequestMessage>(parsed) {
                    Ok(req) => {
                        let resp = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            self.handle_request(req)
                        })) {
                            Ok(r) => r,
                            Err(_) => ResponseMessage {
                                jsonrpc: "2.0".to_string(),
                                id: parsed_id,
                                result: None,
                                error: Some(ResponseError {
                                    code: -32603,
                                    message: "Internal server error: panic recovered".to_string(),
                                    data: None,
                                }),
                            },
                        };
                        write_message(&mut writer, &serde_json::to_string(&resp)?)?;
                    }
                    Err(e) => {
                        let err_resp = ResponseMessage {
                            jsonrpc: "2.0".to_string(),
                            id: parsed_id,
                            result: None,
                            error: Some(ResponseError {
                                code: -32600,
                                message: format!("Invalid Request: {}", e),
                                data: None,
                            }),
                        };
                        write_message(&mut writer, &serde_json::to_string(&err_resp)?)?;
                    }
                }
            } else {
                // Notification
                if let Ok(notif) = serde_json::from_value::<NotificationMessage>(parsed) {
                    let notif_res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        self.handle_notification(notif)
                    }));
                    if let Ok(Some(outgoing_notifs)) = notif_res {
                        for out in outgoing_notifs {
                            write_message(&mut writer, &serde_json::to_string(&out)?)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn handle_request(&mut self, req: RequestMessage) -> ResponseMessage {
        match req.method.as_str() {
            "initialize" => {
                let result = json!({
                    "capabilities": {
                        "textDocumentSync": 1, // Full document sync
                        "completionProvider": {
                            "resolveProvider": false,
                            "triggerCharacters": [".", "(", "\"", " "]
                        },
                        "hoverProvider": true,
                        "documentFormattingProvider": true,
                        "documentSymbolProvider": true
                    },
                    "serverInfo": {
                        "name": "scratch-lsp",
                        "version": "0.1.0"
                    }
                });

                ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: Some(req.id),
                    result: Some(result),
                    error: None,
                }
            }
            "shutdown" => {
                self.is_shutdown = true;
                ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: Some(req.id),
                    result: Some(Value::Null),
                    error: None,
                }
            }
            "textDocument/completion" => {
                let params = req.params.unwrap_or(Value::Null);
                let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
                let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;

                let completions: Vec<CompletionItem> = if let Some(doc) = self.get_document(uri) {
                    self.analyzer.compute_completions(doc, Position { line, character })
                } else {
                    Vec::new()
                };

                ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: Some(req.id),
                    result: Some(serde_json::to_value(completions).unwrap_or(Value::Null)),
                    error: None,
                }
            }
            "textDocument/hover" => {
                let params = req.params.unwrap_or(Value::Null);
                let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
                let line = params["position"]["line"].as_u64().unwrap_or(0) as u32;
                let character = params["position"]["character"].as_u64().unwrap_or(0) as u32;

                let hover_opt: Option<Hover> = if let Some(doc) = self.get_document(uri) {
                    self.analyzer.compute_hover(doc, Position { line, character })
                } else {
                    None
                };

                ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: Some(req.id),
                    result: Some(serde_json::to_value(hover_opt).unwrap_or(Value::Null)),
                    error: None,
                }
            }
            "textDocument/formatting" => {
                let params = req.params.unwrap_or(Value::Null);
                let uri = params["textDocument"]["uri"].as_str().unwrap_or("");

                let edits: Vec<TextEdit> = if let Some(doc) = self.get_document(uri) {
                    self.analyzer.format_document(doc).unwrap_or_default()
                } else {
                    Vec::new()
                };

                ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: Some(req.id),
                    result: Some(serde_json::to_value(edits).unwrap_or(Value::Null)),
                    error: None,
                }
            }
            "textDocument/documentSymbol" => {
                let params = req.params.unwrap_or(Value::Null);
                let uri = params["textDocument"]["uri"].as_str().unwrap_or("");

                let symbols: Vec<DocumentSymbol> = if let Some(doc) = self.get_document(uri) {
                    self.analyzer.compute_document_symbols(doc)
                } else {
                    Vec::new()
                };

                ResponseMessage {
                    jsonrpc: "2.0".to_string(),
                    id: Some(req.id),
                    result: Some(serde_json::to_value(symbols).unwrap_or(Value::Null)),
                    error: None,
                }
            }
            unknown => ResponseMessage {
                jsonrpc: "2.0".to_string(),
                id: Some(req.id),
                result: None,
                error: Some(ResponseError {
                    code: -32601,
                    message: format!("Method not found: {}", unknown),
                    data: None,
                }),
            },
        }
    }

    pub fn get_document(&self, uri: &str) -> Option<&Document> {
        if let Some(doc) = self.documents.get(uri) {
            return Some(doc);
        }
        let norm = normalize_uri(uri);
        self.documents
            .iter()
            .find(|(k, _)| normalize_uri(k) == norm)
            .map(|(_, doc)| doc)
    }

    pub fn handle_notification(&mut self, notif: NotificationMessage) -> Option<Vec<NotificationMessage>> {
        match notif.method.as_str() {
            "initialized" => None,
            "exit" => {
                self.is_shutdown = true;
                None
            }
            "textDocument/didOpen" => {
                let params = notif.params?;
                let uri = params["textDocument"]["uri"].as_str()?.to_string();
                let version = params["textDocument"]["version"].as_i64().unwrap_or(0);
                let text = params["textDocument"]["text"].as_str()?.to_string();

                let doc = Document::new(uri.clone(), version, text);
                let diagnostics = self.analyzer.compute_diagnostics(&doc);
                self.documents.insert(uri.clone(), doc);

                Some(vec![self.make_diagnostics_notification(uri, diagnostics)])
            }
            "textDocument/didChange" => {
                let params = notif.params?;
                let uri = params["textDocument"]["uri"].as_str()?.to_string();
                let version = params["textDocument"]["version"].as_i64().unwrap_or(0);
                let changes = params["contentChanges"].as_array()?;
                let text = changes.last()?.get("text")?.as_str()?.to_string();

                let doc = Document::new(uri.clone(), version, text);
                let diagnostics = self.analyzer.compute_diagnostics(&doc);
                self.documents.insert(uri.clone(), doc);

                Some(vec![self.make_diagnostics_notification(uri, diagnostics)])
            }
            "textDocument/didClose" => {
                let params = notif.params?;
                let uri = params["textDocument"]["uri"].as_str()?;
                let norm = normalize_uri(uri);
                self.documents.retain(|k, _| normalize_uri(k) != norm);
                None
            }
            _ => None,
        }
    }

    fn make_diagnostics_notification(
        &self,
        uri: String,
        diagnostics: Vec<Diagnostic>,
    ) -> NotificationMessage {
        let params = PublishDiagnosticsParams { uri, diagnostics };
        NotificationMessage {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/publishDiagnostics".to_string(),
            params: Some(serde_json::to_value(params).unwrap_or(Value::Null)),
        }
    }
}

fn normalize_uri(uri: &str) -> String {
    uri.replace("%3A", ":")
        .replace("%3a", ":")
        .replace("%20", " ")
        .to_lowercase()
}
