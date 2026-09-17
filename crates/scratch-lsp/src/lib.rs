pub mod analysis;
pub mod protocol;
pub mod server;

pub use analysis::{Analyzer, Document};
pub use protocol::*;
pub use server::LspServer;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_lsp_initialize_handshake() {
        let mut server = LspServer::new();
        let req = RequestMessage {
            jsonrpc: "2.0".to_string(),
            id: Id::Number(1),
            method: "initialize".to_string(),
            params: Some(json!({})),
        };

        let resp = server.handle_request(req);
        assert_eq!(resp.id, Some(Id::Number(1)));
        assert!(resp.error.is_none());

        let result = resp.result.expect("Expected result");
        assert!(result["capabilities"]["hoverProvider"].as_bool().unwrap());
        assert!(result["capabilities"]["documentFormattingProvider"].as_bool().unwrap());
    }

    #[test]
    fn test_lsp_diagnostics_on_open() {
        let mut server = LspServer::new();
        let notif = NotificationMessage {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/didOpen".to_string(),
            params: Some(json!({
                "textDocument": {
                    "uri": "file:///game/main.sch",
                    "version": 1,
                    "text": "when start:\n    moov(Player, 5)\n"
                }
            })),
        };

        let outgoing = server.handle_notification(notif).expect("Expected diagnostics notification");
        assert_eq!(outgoing.len(), 1);
        let params = outgoing[0].params.as_ref().unwrap();
        assert_eq!(params["uri"], "file:///game/main.sch");

        let diags = params["diagnostics"].as_array().unwrap();
        assert!(!diags.is_empty());
        assert_eq!(diags[0]["code"], "SL003");
        assert!(diags[0]["message"].as_str().unwrap().contains("Did you mean 'move'?"));
    }

    #[test]
    fn test_lsp_completions() {
        let mut server = LspServer::new();
        // First open document
        server.handle_notification(NotificationMessage {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/didOpen".to_string(),
            params: Some(json!({
                "textDocument": {
                    "uri": "file:///game/main.sch",
                    "version": 1,
                    "text": "when start:\n    "
                }
            })),
        });

        let req = RequestMessage {
            jsonrpc: "2.0".to_string(),
            id: Id::Number(2),
            method: "textDocument/completion".to_string(),
            params: Some(json!({
                "textDocument": { "uri": "file:///game/main.sch" },
                "position": { "line": 1, "character": 4 }
            })),
        };

        let resp = server.handle_request(req);
        let items: Vec<CompletionItem> = serde_json::from_value(resp.result.unwrap()).unwrap();
        assert!(items.iter().any(|i| i.label == "move"));
        assert!(items.iter().any(|i| i.label == "jump"));
        assert!(items.iter().any(|i| i.label == "scene.switch"));
    }

    #[test]
    fn test_lsp_hover() {
        let mut server = LspServer::new();
        server.handle_notification(NotificationMessage {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/didOpen".to_string(),
            params: Some(json!({
                "textDocument": {
                    "uri": "file:///game/main.sch",
                    "version": 1,
                    "text": "when start:\n    move(Player, 5)\n"
                }
            })),
        });

        let req = RequestMessage {
            jsonrpc: "2.0".to_string(),
            id: Id::Number(3),
            method: "textDocument/hover".to_string(),
            params: Some(json!({
                "textDocument": { "uri": "file:///game/main.sch" },
                "position": { "line": 1, "character": 6 }
            })),
        };

        let resp = server.handle_request(req);
        let hover: Hover = serde_json::from_value(resp.result.unwrap()).unwrap();
        assert!(hover.contents.value.contains("move"));
        assert!(hover.contents.value.contains("Moves the specified target object horizontally") || hover.contents.value.contains("Move an object"));
    }

    #[test]
    fn test_lsp_formatting() {
        let mut server = LspServer::new();
        server.handle_notification(NotificationMessage {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/didOpen".to_string(),
            params: Some(json!({
                "textDocument": {
                    "uri": "file:///game/main.sch",
                    "version": 1,
                    "text": "when start:\n score=0\n"
                }
            })),
        });

        let req = RequestMessage {
            jsonrpc: "2.0".to_string(),
            id: Id::Number(4),
            method: "textDocument/formatting".to_string(),
            params: Some(json!({
                "textDocument": { "uri": "file:///game/main.sch" }
            })),
        };

        let resp = server.handle_request(req);
        let edits: Vec<TextEdit> = serde_json::from_value(resp.result.unwrap()).unwrap();
        assert_eq!(edits.len(), 1);
        assert!(edits[0].new_text.contains("    score = 0"));
    }

    #[test]
    fn test_lsp_document_symbols() {
        let mut server = LspServer::new();
        server.handle_notification(NotificationMessage {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/didOpen".to_string(),
            params: Some(json!({
                "textDocument": {
                    "uri": "file:///game/main.sch",
                    "version": 1,
                    "text": "when start:\n    score = 0\n\nwhen action.down(\"right\"):\n    move(Player, 5)\n"
                }
            })),
        });

        let req = RequestMessage {
            jsonrpc: "2.0".to_string(),
            id: Id::Number(5),
            method: "textDocument/documentSymbol".to_string(),
            params: Some(json!({
                "textDocument": { "uri": "file:///game/main.sch" }
            })),
        };

        let resp = server.handle_request(req);
        let symbols: Vec<DocumentSymbol> = serde_json::from_value(resp.result.unwrap()).unwrap();
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].name, "when start");
        assert_eq!(symbols[1].name, "when action.down(\"right\")");
    }

    #[test]
    fn test_lsp_io_framing_loop() {
        use std::io::Cursor;

        let mut server = LspServer::new();
        let client_req = json!({
            "jsonrpc": "2.0",
            "id": 10,
            "method": "initialize",
            "params": {}
        }).to_string();

        let mut input_bytes = Vec::new();
        write_message(&mut input_bytes, &client_req).unwrap();

        let client_shutdown = json!({
            "jsonrpc": "2.0",
            "id": 11,
            "method": "shutdown",
            "params": null
        }).to_string();
        write_message(&mut input_bytes, &client_shutdown).unwrap();

        let reader = Cursor::new(input_bytes);
        let mut output = Vec::new();

        server.run(reader, &mut output).unwrap();

        let mut out_reader = Cursor::new(output);
        let msg1_str = read_message(&mut out_reader).unwrap().expect("Response 1");
        let msg1: ResponseMessage = serde_json::from_str(&msg1_str).unwrap();
        assert_eq!(msg1.id, Some(Id::Number(10)));

        let msg2_str = read_message(&mut out_reader).unwrap().expect("Response 2");
        let msg2: ResponseMessage = serde_json::from_str(&msg2_str).unwrap();
        assert_eq!(msg2.id, Some(Id::Number(11)));
    }
}
