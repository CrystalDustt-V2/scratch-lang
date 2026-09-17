use scratch_assets::AssetIndex;
use scratch_blocks::BlockRegistry;
use scratch_bytecode::BytecodeCompiler;
use scratch_ir::lower_ast_to_ir;
use scratch_language::{format_source, parse, Linter};
use scratch_project::ProjectConfig;
use scratch_scenes::SceneData;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct ProjectInfoResponse {
    name: String,
    code: String,
    entry: String,
    scenes: Vec<String>,
    blocks: Vec<BlockSummary>,
}

#[derive(Serialize, Deserialize)]
struct BlockSummary {
    name: String,
    category: String,
    description: String,
    snippet: String,
    doc: String,
}

#[derive(Deserialize)]
struct CodePayload {
    code: String,
}

#[derive(Serialize)]
struct CheckResponse {
    success: bool,
    errors: Vec<DiagnosticDto>,
    warnings: Vec<DiagnosticDto>,
}

#[derive(Serialize)]
struct DiagnosticDto {
    code: String,
    message: String,
    line: usize,
    column: usize,
    suggestion: Option<String>,
}

#[derive(Serialize)]
struct FormatResponse {
    success: bool,
    formatted: String,
    error: Option<String>,
}

#[derive(Serialize)]
struct CompileResponse {
    success: bool,
    events_count: usize,
    total_instructions: usize,
    disassembly: String,
    error: Option<String>,
}

#[derive(Serialize)]
struct SaveResponse {
    success: bool,
    message: String,
}

pub fn start_studio(project_path: &Path, port: u16, no_open: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_file, config) = resolve_or_init_project(project_path)?;
    let project_dir = entry_file
        .parent()
        .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
        .unwrap_or(Path::new("."))
        .to_path_buf();

    let addr = format!("127.0.0.1:{}", port);
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(_) => {
            // Try next port if busy
            let fallback_port = port + 1;
            let fallback_addr = format!("127.0.0.1:{}", fallback_port);
            println!("Port {} busy, binding to {}...", port, fallback_port);
            TcpListener::bind(&fallback_addr)?
        }
    };

    let local_addr = listener.local_addr()?;
    let url = format!("http://{}", local_addr);

    println!("============================================================");
    println!(" Scratch Studio Web Playground is live!");
    println!(" URL: {}", url);
    println!(" Project: {} ({})", config.name, entry_file.display());
    println!(" Press Ctrl+C in terminal to stop server.");
    println!("============================================================");

    if !no_open {
        open_browser(&url);
    }

    let shared_entry = Arc::new(entry_file);
    let shared_dir = Arc::new(project_dir);
    let shared_config = Arc::new(config);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let entry = Arc::clone(&shared_entry);
                let dir = Arc::clone(&shared_dir);
                let conf = Arc::clone(&shared_config);

                std::thread::spawn(move || {
                    if let Err(e) = handle_client(&mut stream, &entry, &dir, &conf) {
                        eprintln!("HTTP handler error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}

fn resolve_or_init_project(path: &Path) -> Result<(PathBuf, ProjectConfig), Box<dyn std::error::Error>> {
    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) == Some("sch") {
            let config = ProjectConfig {
                name: path.file_stem().unwrap().to_string_lossy().to_string(),
                entry: path.to_string_lossy().to_string(),
                ..Default::default()
            };
            return Ok((path.to_path_buf(), config));
        } else if path.file_name().and_then(|s| s.to_str()) == Some("project.schproj") {
            let config = ProjectConfig::load_from_file(path)?;
            let parent = path.parent().unwrap_or(Path::new("."));
            let entry = parent.join(&config.entry);
            return Ok((entry, config));
        }
    }

    let proj_file = path.join("project.schproj");
    if proj_file.exists() {
        let config = ProjectConfig::load_from_file(&proj_file)?;
        let entry = path.join(&config.entry);
        return Ok((entry, config));
    }

    let main_sch = path.join("src/main.sch");
    if main_sch.exists() {
        let config = ProjectConfig::default();
        return Ok((main_sch, config));
    }

    let direct_main = path.join("main.sch");
    if direct_main.exists() {
        let config = ProjectConfig::default();
        return Ok((direct_main, config));
    }

    // If path is a non-existent dir or empty, create starter project
    let entry = path.join("src/main.sch");
    std::fs::create_dir_all(path.join("src"))?;
    std::fs::create_dir_all(path.join("scenes"))?;
    std::fs::create_dir_all(path.join("assets/sprites"))?;
    std::fs::create_dir_all(path.join("assets/sounds"))?;

    let default_code = r#"when start:
    score = 0
    background.set("night")
    variable.show("score")

when action.down("right"):
    move(Player, 5)

when action.down("left"):
    move(Player, -5)

when action.press("jump"):
    jump(Player, 12)
"#;
    std::fs::write(&entry, default_code)?;

    let config = ProjectConfig {
        name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        entry: "src/main.sch".to_string(),
        ..Default::default()
    };
    let _ = config.save_to_file(path.join("project.schproj"));

    Ok((entry, config))
}

fn handle_client(
    stream: &mut TcpStream,
    entry_file: &Path,
    project_dir: &Path,
    config: &ProjectConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 8192];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        return Ok(());
    }

    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut lines = request_str.lines();
    let req_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = req_line.split_whitespace().collect();

    if parts.len() < 2 {
        send_response(stream, 400, "Bad Request", "text/plain", b"Malformed request")?;
        return Ok(());
    }

    let method = parts[0];
    let uri = parts[1];

    // Find Content-Length and body if POST
    let mut content_length: usize = 0;
    for line in request_str.lines() {
        if line.to_lowercase().starts_with("content-length:") {
            if let Some(val) = line.split(':').nth(1) {
                content_length = val.trim().parse().unwrap_or(0);
            }
        }
    }

    // Extract body from buffer or read remaining bytes
    let body = if method == "POST" && content_length > 0 {
        let header_end = request_str.find("\r\n\r\n").map(|idx| idx + 4)
            .or_else(|| request_str.find("\n\n").map(|idx| idx + 2));

        if let Some(start) = header_end {
            let initial_body = &buffer[start..bytes_read];
            let mut full_body = initial_body.to_vec();
            while full_body.len() < content_length {
                let mut chunk = [0u8; 4096];
                let r = stream.read(&mut chunk)?;
                if r == 0 { break; }
                full_body.extend_from_slice(&chunk[..r]);
            }
            String::from_utf8_lossy(&full_body).to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    match (method, uri) {
        ("GET", "/") | ("GET", "/index.html") => {
            let html = render_studio_html(&config.name);
            send_response(stream, 200, "OK", "text/html; charset=utf-8", html.as_bytes())?;
        }
        ("GET", "/api/project") => {
            let code = std::fs::read_to_string(entry_file).unwrap_or_default();
            let registry = BlockRegistry::core();
            let mut blocks = Vec::new();

            for block in registry.iter() {
                blocks.push(BlockSummary {
                    name: block.name.clone(),
                    category: format!("{:?}", block.category),
                    description: block.description.clone(),
                    snippet: block.autocomplete_metadata.snippet.clone(),
                    doc: block.documentation.clone(),
                });
            }
            blocks.sort_by(|a, b| a.category.cmp(&b.category).then_with(|| a.name.cmp(&b.name)));

            let mut scenes = Vec::new();
            let scenes_dir = project_dir.join("scenes");
            if scenes_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&scenes_dir) {
                    for entry in entries.flatten() {
                        if entry.path().extension().and_then(|s| s.to_str()) == Some("scene") {
                            scenes.push(entry.file_name().to_string_lossy().to_string());
                        }
                    }
                }
            }

            let resp = ProjectInfoResponse {
                name: config.name.clone(),
                code,
                entry: entry_file.to_string_lossy().to_string(),
                scenes,
                blocks,
            };

            let json = serde_json::to_string(&resp)?;
            send_response(stream, 200, "OK", "application/json", json.as_bytes())?;
        }
        ("POST", "/api/check") => {
            let payload: CodePayload = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(e) => {
                    let err = format!("Invalid JSON: {}", e);
                    send_response(stream, 400, "Bad Request", "application/json", err.as_bytes())?;
                    return Ok(());
                }
            };

            let registry = BlockRegistry::core();
            match parse(&payload.code) {
                Ok(ast) => {
                    let mut linter = Linter::new(&registry);
                    let assets_dir = project_dir.join("assets");
                    let mut asset_index = AssetIndex::new();
                    if assets_dir.exists() {
                        asset_index.scan_directory(&assets_dir);
                        linter = linter.with_assets(&asset_index);
                    }

                    let scenes_dir = project_dir.join("scenes");
                    let mut scene_objects = Vec::new();
                    if scenes_dir.exists() {
                        if let Ok(entries) = std::fs::read_dir(&scenes_dir) {
                            for entry in entries.flatten() {
                                if entry.path().extension().and_then(|s| s.to_str()) == Some("scene") {
                                    if let Ok(scene_data) = SceneData::load_from_file(entry.path()) {
                                        for object in scene_data.objects {
                                            scene_objects.push(object.name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if !scene_objects.is_empty() {
                        linter = linter.with_objects(scene_objects);
                    }

                    let diagnostics = linter.lint_program(&ast);
                    let mut warnings = Vec::new();
                    for d in diagnostics {
                        warnings.push(DiagnosticDto {
                            code: d.code.to_string(),
                            message: d.message,
                            line: d.span.line,
                            column: d.span.col,
                            suggestion: d.suggestion,
                        });
                    }

                    let resp = CheckResponse {
                        success: true,
                        errors: Vec::new(),
                        warnings,
                    };
                    let json = serde_json::to_string(&resp)?;
                    send_response(stream, 200, "OK", "application/json", json.as_bytes())?;
                }
                Err(err) => {
                    let resp = CheckResponse {
                        success: false,
                        errors: vec![DiagnosticDto {
                            code: "SYNTAX_ERROR".to_string(),
                            message: err.to_string(),
                            line: 1,
                            column: 1,
                            suggestion: None,
                        }],
                        warnings: Vec::new(),
                    };
                    let json = serde_json::to_string(&resp)?;
                    send_response(stream, 200, "OK", "application/json", json.as_bytes())?;
                }
            }
        }
        ("POST", "/api/format") => {
            let payload: CodePayload = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(e) => {
                    let err = format!("Invalid JSON: {}", e);
                    send_response(stream, 400, "Bad Request", "application/json", err.as_bytes())?;
                    return Ok(());
                }
            };

            match format_source(&payload.code) {
                Ok(formatted) => {
                    let resp = FormatResponse {
                        success: true,
                        formatted,
                        error: None,
                    };
                    let json = serde_json::to_string(&resp)?;
                    send_response(stream, 200, "OK", "application/json", json.as_bytes())?;
                }
                Err(e) => {
                    let resp = FormatResponse {
                        success: false,
                        formatted: payload.code,
                        error: Some(e.to_string()),
                    };
                    let json = serde_json::to_string(&resp)?;
                    send_response(stream, 200, "OK", "application/json", json.as_bytes())?;
                }
            }
        }
        ("POST", "/api/save") => {
            let payload: CodePayload = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(e) => {
                    let err = format!("Invalid JSON: {}", e);
                    send_response(stream, 400, "Bad Request", "application/json", err.as_bytes())?;
                    return Ok(());
                }
            };

            match std::fs::write(entry_file, &payload.code) {
                Ok(_) => {
                    let resp = SaveResponse {
                        success: true,
                        message: format!("Saved to {}", entry_file.display()),
                    };
                    let json = serde_json::to_string(&resp)?;
                    send_response(stream, 200, "OK", "application/json", json.as_bytes())?;
                }
                Err(e) => {
                    let resp = SaveResponse {
                        success: false,
                        message: format!("Error saving: {}", e),
                    };
                    let json = serde_json::to_string(&resp)?;
                    send_response(stream, 500, "Internal Server Error", "application/json", json.as_bytes())?;
                }
            }
        }
        ("POST", "/api/compile") => {
            let payload: CodePayload = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(e) => {
                    let err = format!("Invalid JSON: {}", e);
                    send_response(stream, 400, "Bad Request", "application/json", err.as_bytes())?;
                    return Ok(());
                }
            };

            let registry = BlockRegistry::core();
            match parse(&payload.code) {
                Ok(ast) => match lower_ast_to_ir(&ast, &registry) {
                    Ok(ir) => {
                        let mut compiler = BytecodeCompiler::new();
                        let program = compiler.compile_program(&ir);

                        let mut disasm = String::new();
                        let mut total_inst = 0;
                        for event in &program.events {
                            total_inst += event.chunk.instructions.len();
                            disasm.push_str(&event.chunk.disassemble(&format!("{:?}", event.trigger)));
                            disasm.push('\n');
                        }

                        let resp = CompileResponse {
                            success: true,
                            events_count: program.events.len(),
                            total_instructions: total_inst,
                            disassembly: disasm,
                            error: None,
                        };
                        let json = serde_json::to_string(&resp)?;
                        send_response(stream, 200, "OK", "application/json", json.as_bytes())?;
                    }
                    Err(e) => {
                        let resp = CompileResponse {
                            success: false,
                            events_count: 0,
                            total_instructions: 0,
                            disassembly: String::new(),
                            error: Some(format!("IR Lowering error: {}", e)),
                        };
                        let json = serde_json::to_string(&resp)?;
                        send_response(stream, 200, "OK", "application/json", json.as_bytes())?;
                    }
                },
                Err(e) => {
                    let resp = CompileResponse {
                        success: false,
                        events_count: 0,
                        total_instructions: 0,
                        disassembly: String::new(),
                        error: Some(format!("Parse error: {}", e)),
                    };
                    let json = serde_json::to_string(&resp)?;
                    send_response(stream, 200, "OK", "application/json", json.as_bytes())?;
                }
            }
        }
        _ => {
            // Check for assets / static files
            if uri.starts_with("/assets/") {
                let rel_path = uri.trim_start_matches('/');
                let file_path = project_dir.join(rel_path);
                if file_path.is_file() {
                    let content = std::fs::read(&file_path)?;
                    let mime = guess_mime_type(&file_path);
                    send_response(stream, 200, "OK", mime, &content)?;
                    return Ok(());
                }
            }
            send_response(stream, 404, "Not Found", "text/plain", b"404 Not Found")?;
        }
    }

    Ok(())
}

fn guess_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "wav" => "audio/wav",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        "json" => "application/json",
        "js" => "application/javascript",
        "css" => "text/css",
        _ => "application/octet-stream",
    }
}

fn send_response(
    stream: &mut TcpStream,
    status_code: u16,
    status_text: &str,
    content_type: &str,
    body: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        status_code,
        status_text,
        content_type,
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

pub fn render_studio_html(project_name: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Scratch Studio — {name}</title>
    <style>
        :root {{
            --bg-base: #13141b;
            --bg-panel: #1e1f29;
            --bg-surface: #272836;
            --border: #333547;
            --accent: #4c97ff;
            --accent-hover: #6ea8ff;
            --text: #f0f3fa;
            --text-muted: #8b92a5;
            --motion: #4c97ff;
            --looks: #9966ff;
            --sound: #cf63cf;
            --events: #ffbf00;
            --control: #ffab19;
            --sensing: #5cb1d6;
            --operators: #59c059;
            --variables: #ff8c1a;
            --custom: #ff6680;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background: var(--bg-base);
            color: var(--text);
            height: 100vh;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            user-select: none;
        }}
        /* Top Navigation */
        header {{
            height: 52px;
            background: var(--bg-panel);
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 16px;
            gap: 12px;
        }}
        .brand {{
            display: flex;
            align-items: center;
            gap: 10px;
            font-weight: 700;
            font-size: 16px;
        }}
        .brand-badge {{
            background: linear-gradient(135deg, #ff8c1a, #ff6680);
            color: white;
            padding: 3px 8px;
            border-radius: 6px;
            font-size: 11px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}
        .proj-name {{
            font-size: 14px;
            color: var(--text-muted);
            font-weight: 500;
        }}
        .toolbar {{
            display: flex;
            align-items: center;
            gap: 8px;
        }}
        .btn {{
            background: var(--bg-surface);
            border: 1px solid var(--border);
            color: var(--text);
            padding: 6px 14px;
            border-radius: 6px;
            font-size: 13px;
            font-weight: 600;
            cursor: pointer;
            display: inline-flex;
            align-items: center;
            gap: 6px;
            transition: all 0.15s ease;
        }}
        .btn:hover {{ background: #35374a; border-color: #4a4d66; }}
        .btn.primary {{ background: var(--accent); border-color: var(--accent); color: white; }}
        .btn.primary:hover {{ background: var(--accent-hover); }}
        .btn.success {{ background: #27ae60; border-color: #27ae60; color: white; }}
        .btn.danger {{ background: #e74c3c; border-color: #e74c3c; color: white; }}
        .status-pill {{
            font-size: 11px;
            padding: 4px 10px;
            border-radius: 12px;
            background: #272836;
            color: #2ecc71;
            font-weight: 600;
        }}
        .fps-badge {{
            font-family: monospace;
            font-size: 12px;
            color: var(--text-muted);
        }}
        /* Main Workspace 3 Columns */
        .workspace {{
            flex: 1;
            display: grid;
            grid-template-columns: 240px 1fr 480px;
            overflow: hidden;
        }}
        /* Panel 1: Blocks Palette */
        .palette {{
            background: var(--bg-panel);
            border-right: 1px solid var(--border);
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }}
        .palette-header {{
            padding: 12px 14px;
            font-size: 12px;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            color: var(--text-muted);
            border-bottom: 1px solid var(--border);
            display: flex;
            justify-content: space-between;
        }}
        .categories {{
            display: flex;
            flex-wrap: wrap;
            padding: 8px;
            gap: 4px;
            border-bottom: 1px solid var(--border);
            background: #181922;
        }}
        .cat-chip {{
            padding: 3px 8px;
            font-size: 11px;
            border-radius: 4px;
            cursor: pointer;
            opacity: 0.8;
            font-weight: 600;
        }}
        .cat-chip:hover, .cat-chip.active {{ opacity: 1.0; }}
        .cat-chip.motion {{ background: var(--motion); color: white; }}
        .cat-chip.looks {{ background: var(--looks); color: white; }}
        .cat-chip.sound {{ background: var(--sound); color: white; }}
        .cat-chip.events {{ background: var(--events); color: #222; }}
        .cat-chip.control {{ background: var(--control); color: #222; }}
        .cat-chip.sensing {{ background: var(--sensing); color: #222; }}
        .cat-chip.operators {{ background: var(--operators); color: white; }}
        .cat-chip.variables {{ background: var(--variables); color: white; }}
        .block-list {{
            flex: 1;
            overflow-y: auto;
            padding: 10px;
            display: flex;
            flex-direction: column;
            gap: 6px;
        }}
        .block-item {{
            padding: 6px 10px;
            border-radius: 6px;
            font-family: monospace;
            font-size: 12px;
            cursor: pointer;
            color: white;
            box-shadow: 0 1px 3px rgba(0,0,0,0.3);
            transition: transform 0.1s ease;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .block-item:hover {{ transform: scale(1.02); }}
        .block-item.motion {{ background: var(--motion); }}
        .block-item.looks {{ background: var(--looks); }}
        .block-item.sound {{ background: var(--sound); }}
        .block-item.events {{ background: var(--events); color: #1a1a1a; }}
        .block-item.control {{ background: var(--control); color: #1a1a1a; }}
        .block-item.sensing {{ background: var(--sensing); color: #1a1a1a; }}
        .block-item.operators {{ background: var(--operators); }}
        .block-item.variables {{ background: var(--variables); }}
        /* Panel 2: Code Editor */
        .editor-container {{
            display: flex;
            flex-direction: column;
            background: #181922;
            overflow: hidden;
        }}
        .editor-tabs {{
            height: 38px;
            background: var(--bg-panel);
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            padding: 0 12px;
            gap: 8px;
        }}
        .tab {{
            font-size: 13px;
            font-weight: 600;
            padding: 6px 12px;
            background: var(--bg-surface);
            border-radius: 4px 4px 0 0;
            color: var(--accent);
            border-bottom: 2px solid var(--accent);
        }}
        .editor-wrap {{
            flex: 1;
            display: flex;
            overflow: hidden;
            position: relative;
        }}
        .line-numbers {{
            width: 44px;
            background: #15161e;
            color: #555a6d;
            font-family: "Fira Code", monospace;
            font-size: 13px;
            line-height: 20px;
            padding: 12px 6px;
            text-align: right;
            user-select: none;
            overflow: hidden;
            border-right: 1px solid var(--border);
        }}
        .code-textarea {{
            flex: 1;
            background: transparent;
            color: #e2e8f0;
            font-family: "Fira Code", monospace;
            font-size: 13px;
            line-height: 20px;
            padding: 12px;
            border: none;
            outline: none;
            resize: none;
            white-space: pre;
            tab-size: 4;
            overflow: auto;
        }}
        .editor-error-bar {{
            height: 28px;
            background: #2a1b1b;
            color: #ff7675;
            font-size: 12px;
            display: flex;
            align-items: center;
            padding: 0 12px;
            border-top: 1px solid #4a2222;
            font-family: monospace;
            display: none;
        }}
        /* Panel 3: Stage Canvas */
        .stage-pane {{
            background: var(--bg-panel);
            border-left: 1px solid var(--border);
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }}
        .stage-header {{
            padding: 10px 14px;
            border-bottom: 1px solid var(--border);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        .canvas-container {{
            position: relative;
            background: #000;
            display: flex;
            justify-content: center;
            align-items: center;
            width: 480px;
            height: 360px;
            margin: 0 auto;
            box-shadow: 0 4px 12px rgba(0,0,0,0.5);
        }}
        canvas {{
            width: 480px;
            height: 360px;
            background: #1e222d;
            display: block;
        }}
        /* Dialogue Overlay */
        .dialogue-overlay {{
            position: absolute;
            bottom: 12px;
            left: 12px;
            right: 12px;
            background: rgba(30, 34, 45, 0.95);
            border: 2px solid var(--accent);
            border-radius: 8px;
            padding: 8px 12px;
            display: none;
            flex-direction: column;
            gap: 6px;
            box-shadow: 0 4px 16px rgba(0,0,0,0.6);
        }}
        .dialogue-prompt {{
            font-size: 13px;
            color: #ffffff;
            font-weight: 600;
        }}
        .dialogue-row {{
            display: flex;
            gap: 8px;
        }}
        .dialogue-input {{
            flex: 1;
            background: #13141b;
            border: 1px solid var(--border);
            color: white;
            padding: 6px 10px;
            border-radius: 4px;
            font-size: 13px;
            outline: none;
        }}
        .dialogue-submit {{
            background: var(--accent);
            color: white;
            border: none;
            border-radius: 4px;
            padding: 0 12px;
            font-weight: 600;
            cursor: pointer;
        }}
        /* Variable monitor overlay */
        .var-overlay {{
            position: absolute;
            top: 10px;
            left: 10px;
            display: flex;
            flex-direction: column;
            gap: 4px;
            pointer-events: none;
        }}
        .var-tag {{
            background: rgba(255, 140, 26, 0.9);
            color: white;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 700;
            display: inline-flex;
            gap: 6px;
            box-shadow: 0 1px 4px rgba(0,0,0,0.4);
        }}
        .var-val {{
            background: #ffffff;
            color: #111;
            padding: 0 6px;
            border-radius: 3px;
        }}
        /* Bottom Inspector Panel */
        .inspector {{
            height: 180px;
            background: var(--bg-panel);
            border-top: 1px solid var(--border);
            display: flex;
            flex-direction: column;
        }}
        .inspector-tabs {{
            display: flex;
            background: #181922;
            border-bottom: 1px solid var(--border);
            padding: 0 8px;
        }}
        .insp-tab {{
            padding: 6px 14px;
            font-size: 12px;
            color: var(--text-muted);
            cursor: pointer;
            font-weight: 600;
        }}
        .insp-tab.active {{
            color: var(--text);
            border-bottom: 2px solid var(--accent);
        }}
        .insp-body {{
            flex: 1;
            overflow: auto;
            padding: 8px 12px;
            font-family: monospace;
            font-size: 12px;
            color: #a0aec0;
        }}
        .insp-table {{
            width: 100%;
            border-collapse: collapse;
        }}
        .insp-table th, .insp-table td {{
            text-align: left;
            padding: 4px 8px;
            border-bottom: 1px solid #2a2c3a;
        }}
        .insp-table th {{ color: var(--text-muted); }}
    </style>
</head>
<body>
    <header>
        <div class="brand">
            <span class="brand-badge">Studio</span>
            <span>Scratch Studio</span>
            <span class="proj-name">/ {name}</span>
        </div>
        <div class="toolbar">
            <button class="btn success" id="btn-run" title="Ctrl+Enter">▶ Run</button>
            <button class="btn" id="btn-pause">⏸ Pause</button>
            <button class="btn danger" id="btn-restart">🔄 Restart</button>
            <button class="btn" id="btn-format" title="Alt+Shift+F">✨ Format</button>
            <button class="btn primary" id="btn-save" title="Ctrl+S">💾 Save</button>
            <select class="btn" id="template-select">
                <option value="">Templates...</option>
                <option value="platformer">2D Platformer</option>
                <option value="coins">Catch Coins</option>
                <option value="dialogue">Dialogue & Quiz</option>
                <option value="motion">Smooth Motion</option>
                <option value="lists">Dynamic Inventory</option>
            </select>
            <span class="status-pill" id="status-pill">Ready</span>
            <span class="fps-badge" id="fps-badge">60 FPS</span>
        </div>
    </header>

    <div class="workspace">
        <!-- Panel 1: Block Library Palette -->
        <div class="palette">
            <div class="palette-header">
                <span>Blocks Palette</span>
            </div>
            <div class="categories">
                <div class="cat-chip motion active" data-cat="Motion">Motion</div>
                <div class="cat-chip looks" data-cat="Looks">Looks</div>
                <div class="cat-chip sound" data-cat="Audio">Sound</div>
                <div class="cat-chip events" data-cat="Movement">Events</div>
                <div class="cat-chip sensing" data-cat="Sensing">Sensing</div>
                <div class="cat-chip variables" data-cat="Variables">Data</div>
            </div>
            <div class="block-list" id="block-list"></div>
        </div>

        <!-- Panel 2: Code Studio -->
        <div class="editor-container">
            <div class="editor-tabs">
                <div class="tab">src/main.sch</div>
            </div>
            <div class="editor-wrap">
                <div class="line-numbers" id="line-numbers">1</div>
                <textarea class="code-textarea" id="code-editor" spellcheck="false"></textarea>
            </div>
            <div class="editor-error-bar" id="error-bar"></div>
        </div>

        <!-- Panel 3: Stage Canvas -->
        <div class="stage-pane">
            <div class="stage-header">
                <span style="font-size: 13px; font-weight: 700;">Stage View (480 x 360)</span>
                <span style="font-size: 11px; color: var(--text-muted);">Canvas 2D</span>
            </div>
            <div class="canvas-container">
                <canvas id="stage-canvas" width="480" height="360"></canvas>
                <div class="var-overlay" id="var-overlay"></div>
                <div class="dialogue-overlay" id="dialogue-overlay">
                    <div class="dialogue-prompt" id="dialogue-prompt">What's your name?</div>
                    <div class="dialogue-row">
                        <input type="text" class="dialogue-input" id="dialogue-input" placeholder="Type an answer..." />
                        <button class="dialogue-submit" id="dialogue-submit">✔</button>
                    </div>
                </div>
            </div>
            <!-- Bottom Inspector Inside Right Pane -->
            <div class="inspector">
                <div class="inspector-tabs">
                    <div class="insp-tab active" data-tab="console">Console</div>
                    <div class="insp-tab" data-tab="vars">Variables</div>
                    <div class="insp-tab" data-tab="entities">Entities</div>
                    <div class="insp-tab" data-tab="bytecode">Bytecode IR</div>
                </div>
                <div class="insp-body" id="insp-body"></div>
            </div>
        </div>
    </div>

    <script>
        // State
        let project = {{ name: "{name}", code: "" }};
        let blocks = [];
        let running = true;
        let paused = false;
        let lastTime = performance.now();
        let frameCount = 0;
        let activeTab = "console";
        let consoleLogs = [];

        // Game Simulation State
        let world = {{
            vars: {{ score: 0 }},
            visibleVars: new Set(["score"]),
            entities: [
                {{ name: "Player", x: 240, y: 180, vx: 0, vy: 0, width: 36, height: 36, color: "#4c97ff", layer: 1, visible: true, rotation: 0 }},
                {{ name: "Coin", x: 140, y: 120, vx: 0, vy: 0, width: 24, height: 24, color: "#f1c40f", layer: 0, visible: true, rotation: 0 }},
                {{ name: "Enemy", x: 340, y: 240, vx: 1, vy: 0, width: 32, height: 32, color: "#e74c3c", layer: 0, visible: true, rotation: 0 }}
            ],
            background: "#1e222d",
            prompt: null,
            answer: ""
        }};

        // Audio Synth
        let audioCtx = null;
        function playBeep(freq = 440, type = 'sine', duration = 0.15) {{
            try {{
                if (!audioCtx) audioCtx = new (window.AudioContext || window.webkitAudioContext)();
                let osc = audioCtx.createOscillator();
                let gain = audioCtx.createGain();
                osc.type = type;
                osc.frequency.value = freq;
                gain.gain.setValueAtTime(0.15, audioCtx.currentTime);
                gain.gain.exponentialRampToValueAtTime(0.001, audioCtx.currentTime + duration);
                osc.connect(gain);
                gain.connect(audioCtx.destination);
                osc.start();
                osc.stop(audioCtx.currentTime + duration);
            }} catch(e) {{}}
        }}

        // DOM elements
        const editor = document.getElementById("code-editor");
        const lineNums = document.getElementById("line-numbers");
        const canvas = document.getElementById("stage-canvas");
        const ctx = canvas.getContext("2d");
        const blockList = document.getElementById("block-list");
        const errorBar = document.getElementById("error-bar");
        const statusPill = document.getElementById("status-pill");
        const fpsBadge = document.getElementById("fps-badge");
        const varOverlay = document.getElementById("var-overlay");
        const dialogueOverlay = document.getElementById("dialogue-overlay");
        const dialoguePrompt = document.getElementById("dialogue-prompt");
        const dialogueInput = document.getElementById("dialogue-input");
        const dialogueSubmit = document.getElementById("dialogue-submit");
        const inspBody = document.getElementById("insp-body");

        // Key tracking
        const keys = {{}};
        window.addEventListener("keydown", (e) => {{
            keys[e.key] = true;
            if (e.ctrlKey && e.key === "s") {{
                e.preventDefault();
                saveCode();
            }}
            if (e.ctrlKey && e.key === "Enter") {{
                e.preventDefault();
                restartGame();
            }}
            if (e.altKey && e.shiftKey && (e.key === "F" || e.key === "f")) {{
                e.preventDefault();
                formatCode();
            }}
        }});
        window.addEventListener("keyup", (e) => {{ keys[e.key] = false; }});

        // Initialize project
        fetch("/api/project")
            .then(r => r.json())
            .then(data => {{
                project = data;
                editor.value = data.code;
                blocks = data.blocks || [];
                updateLineNumbers();
                renderBlocks("Motion");
                checkCode();
                compileCode();
                logConsole("Project loaded: " + data.name);
            }});

        function updateLineNumbers() {{
            const count = editor.value.split("\n").length;
            let lines = [];
            for (let i = 1; i <= count; i++) lines.push(i);
            lineNums.textContent = lines.join("\n");
        }}

        editor.addEventListener("input", () => {{
            updateLineNumbers();
            debounce(checkCode, 400)();
        }});

        editor.addEventListener("keydown", (e) => {{
            if (e.key === "Tab") {{
                e.preventDefault();
                const start = editor.selectionStart;
                const end = editor.selectionEnd;
                editor.value = editor.value.substring(0, start) + "    " + editor.value.substring(end);
                editor.selectionStart = editor.selectionEnd = start + 4;
            }}
        }});

        function renderBlocks(category) {{
            blockList.innerHTML = "";
            const filtered = blocks.filter(b => b.category.toLowerCase() === category.toLowerCase() || category === "all");
            filtered.forEach(b => {{
                const div = document.createElement("div");
                div.className = `block-item ${{category.toLowerCase()}}`;
                div.textContent = b.name;
                div.title = `${{b.description}}\nSignature: ${{b.snippet}}`;
                div.onclick = () => {{
                    insertAtCursor(b.snippet.replace(/\$\d+/g, ""));
                }};
                blockList.appendChild(div);
            }});
        }}

        document.querySelectorAll(".cat-chip").forEach(chip => {{
            chip.onclick = () => {{
                document.querySelectorAll(".cat-chip").forEach(c => c.classList.remove("active"));
                chip.classList.add("active");
                renderBlocks(chip.dataset.cat);
            }};
        }});

        function insertAtCursor(text) {{
            const start = editor.selectionStart;
            const end = editor.selectionEnd;
            editor.value = editor.value.substring(0, start) + text + editor.value.substring(end);
            editor.focus();
            editor.selectionStart = editor.selectionEnd = start + text.length;
            updateLineNumbers();
            checkCode();
        }}

        function checkCode() {{
            statusPill.textContent = "Checking...";
            statusPill.style.color = "#f39c12";
            fetch("/api/check", {{
                method: "POST",
                headers: {{ "Content-Type": "application/json" }},
                body: JSON.stringify({{ code: editor.value }})
            }})
            .then(r => r.json())
            .then(res => {{
                if (!res.success && res.errors.length > 0) {{
                    const err = res.errors[0];
                    errorBar.style.display = "flex";
                    errorBar.textContent = `Line ${{err.line}}: ${{err.message}}`;
                    statusPill.textContent = "Syntax Error";
                    statusPill.style.color = "#e74c3c";
                }} else if (res.warnings && res.warnings.length > 0) {{
                    const w = res.warnings[0];
                    errorBar.style.display = "flex";
                    errorBar.textContent = `[${{w.code}}] Line ${{w.line}}: ${{w.message}}`;
                    statusPill.textContent = "Warning";
                    statusPill.style.color = "#f1c40f";
                }} else {{
                    errorBar.style.display = "none";
                    statusPill.textContent = "Clean";
                    statusPill.style.color = "#2ecc71";
                }}
            }});
        }}

        function formatCode() {{
            fetch("/api/format", {{
                method: "POST",
                headers: {{ "Content-Type": "application/json" }},
                body: JSON.stringify({{ code: editor.value }})
            }})
            .then(r => r.json())
            .then(res => {{
                if (res.success) {{
                    editor.value = res.formatted;
                    updateLineNumbers();
                    checkCode();
                    logConsole("Source code formatted cleanly.");
                }}
            }});
        }}

        function saveCode() {{
            statusPill.textContent = "Saving...";
            fetch("/api/save", {{
                method: "POST",
                headers: {{ "Content-Type": "application/json" }},
                body: JSON.stringify({{ code: editor.value }})
            }})
            .then(r => r.json())
            .then(res => {{
                if (res.success) {{
                    statusPill.textContent = "Saved";
                    statusPill.style.color = "#2ecc71";
                    logConsole("Project saved successfully.");
                    compileCode();
                }}
            }});
        }}

        let lastDisassembly = "";
        function compileCode() {{
            fetch("/api/compile", {{
                method: "POST",
                headers: {{ "Content-Type": "application/json" }},
                body: JSON.stringify({{ code: editor.value }})
            }})
            .then(r => r.json())
            .then(res => {{
                if (res.success) {{
                    lastDisassembly = res.disassembly;
                    if (activeTab === "bytecode") updateInspector();
                }}
            }});
        }}

        // Templates
        const templates = {{
            platformer: `when start:
    score = 0
    variable.show("score")
    background.set("night")

when action.down("right"):
    move(Player, 5)

when action.down("left"):
    move(Player, -5)

when action.press("jump"):
    jump(Player, 12)
    sound.play("jump")

when touches(Player, Coin):
    score += 10
    teleport(Coin, random(50, 430), random(50, 310))
    sound.play("coin")
`,
            dialogue: `when start:
    ask("What is your adventure name?")
    variable.show("score")

when message("answered"):
    say_for(Player, text.join("Welcome, ", get_answer()), 2)
    score = 100
`,
            motion: `when start:
    set_rotation_style(Player, "left-right")
    glide(Player, 2, 380, 280)

every 3 seconds:
    go_to(Player, "random")
`,
            lists: `when start:
    list.add("inventory", "iron_sword")
    list.add("inventory", "wooden_shield")
    list.add("inventory", "health_potion")
    list.show("inventory")
`
        }};

        document.getElementById("template-select").onchange = (e) => {{
            const val = e.target.value;
            if (templates[val]) {{
                editor.value = templates[val];
                updateLineNumbers();
                checkCode();
                restartGame();
            }}
            e.target.value = "";
        }};

        // Buttons
        document.getElementById("btn-run").onclick = () => {{ running = true; paused = false; }};
        document.getElementById("btn-pause").onclick = () => {{ paused = !paused; }};
        document.getElementById("btn-restart").onclick = restartGame;
        document.getElementById("btn-format").onclick = formatCode;
        document.getElementById("btn-save").onclick = saveCode;

        function restartGame() {{
            world.entities[0].x = 240;
            world.entities[0].y = 180;
            world.entities[0].vx = 0;
            world.entities[0].vy = 0;
            world.vars.score = 0;
            logConsole("Simulation restarted.");
            playBeep(520, 'triangle', 0.2);
        }}

        // Dialogue submission
        dialogueSubmit.onclick = submitDialogue;
        dialogueInput.onkeydown = (e) => {{ if (e.key === "Enter") submitDialogue(); }};

        function submitDialogue() {{
            const val = dialogueInput.value.trim();
            world.answer = val;
            world.prompt = null;
            dialogueOverlay.style.display = "none";
            logConsole(`Dialogue answer submitted: "${{val}}"`);
            playBeep(660, 'sine', 0.1);
        }}

        function showDialogue(prompt) {{
            world.prompt = prompt;
            dialoguePrompt.textContent = prompt;
            dialogueInput.value = "";
            dialogueOverlay.style.display = "flex";
            dialogueInput.focus();
        }}

        // Inspector tabs
        document.querySelectorAll(".insp-tab").forEach(tab => {{
            tab.onclick = () => {{
                document.querySelectorAll(".insp-tab").forEach(t => t.classList.remove("active"));
                tab.classList.add("active");
                activeTab = tab.dataset.tab;
                updateInspector();
            }};
        }});

        function logConsole(msg) {{
            const time = new Date().toLocaleTimeString();
            consoleLogs.unshift(`[${{time}}] ${{msg}}`);
            if (consoleLogs.length > 50) consoleLogs.pop();
            if (activeTab === "console") updateInspector();
        }}

        function updateInspector() {{
            if (activeTab === "console") {{
                inspBody.innerHTML = consoleLogs.map(l => `<div>${{l}}</div>`).join("");
            }} else if (activeTab === "vars") {{
                let rows = Object.entries(world.vars).map(([k, v]) =>
                    `<tr><td><strong>${{k}}</strong></td><td>${{v}}</td></tr>`
                ).join("");
                inspBody.innerHTML = `<table class="insp-table"><thead><tr><th>Variable</th><th>Value</th></tr></thead><tbody>${{rows}}</tbody></table>`;
            }} else if (activeTab === "entities") {{
                let rows = world.entities.map(e =>
                    `<tr><td><strong>${{e.name}}</strong></td><td>(${{Math.round(e.x)}}, ${{Math.round(e.y)}})</td><td>${{e.layer}}</td><td>${{e.visible}}</td></tr>`
                ).join("");
                inspBody.innerHTML = `<table class="insp-table"><thead><tr><th>Entity</th><th>Position</th><th>Layer</th><th>Visible</th></tr></thead><tbody>${{rows}}</tbody></table>`;
            }} else if (activeTab === "bytecode") {{
                inspBody.innerHTML = `<pre style="white-space: pre-wrap;">${{lastDisassembly || "(No bytecode compiled yet)"}}</pre>`;
            }}
        }}

        // Canvas Game Loop
        function tick(now) {{
            requestAnimationFrame(tick);
            frameCount++;
            if (now - lastTime >= 1000) {{
                fpsBadge.textContent = `${{frameCount}} FPS`;
                frameCount = 0;
                lastTime = now;
            }}

            if (!running || paused) return;

            // Physics / movement
            const player = world.entities[0];
            if (keys["ArrowRight"] || keys["d"]) player.x += 4;
            if (keys["ArrowLeft"] || keys["a"]) player.x -= 4;
            if (keys["ArrowUp"] || keys["w"]) player.y -= 4;
            if (keys["ArrowDown"] || keys["s"]) player.y += 4;

            // Enemy patrol
            const enemy = world.entities[2];
            enemy.x += enemy.vx * 2;
            if (enemy.x > 420 || enemy.x < 60) enemy.vx *= -1;

            // Coin collision
            const coin = world.entities[1];
            if (Math.hypot(player.x - coin.x, player.y - coin.y) < 30) {{
                world.vars.score = (world.vars.score || 0) + 10;
                coin.x = 40 + Math.random() * 400;
                coin.y = 40 + Math.random() * 280;
                playBeep(880, 'sine', 0.1);
                logConsole(`Collected Coin! Score: ${{world.vars.score}}`);
            }}

            // Render
            ctx.fillStyle = world.background || "#1e222d";
            ctx.fillRect(0, 0, canvas.width, canvas.height);

            // Subtle grid
            ctx.strokeStyle = "rgba(255, 255, 255, 0.04)";
            for (let x = 0; x < canvas.width; x += 32) {{
                ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, canvas.height); ctx.stroke();
            }}
            for (let y = 0; y < canvas.height; y += 32) {{
                ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(canvas.width, y); ctx.stroke();
            }}

            // Draw entities sorted by layer
            const sorted = [...world.entities].sort((a, b) => a.layer - b.layer);
            for (let ent of sorted) {{
                if (!ent.visible) continue;
                ctx.fillStyle = ent.color || "#fff";
                ctx.fillRect(ent.x - ent.width/2, ent.y - ent.height/2, ent.width, ent.height);

                // Sprite label
                ctx.fillStyle = "rgba(255, 255, 255, 0.7)";
                ctx.font = "10px sans-serif";
                ctx.textAlign = "center";
                ctx.fillText(ent.name, ent.x, ent.y - ent.height/2 - 4);
            }}

            // Render variable overlays
            let overlayHtml = "";
            for (let vName of world.visibleVars) {{
                overlayHtml += `<div class="var-tag"><span>${{vName}}</span><span class="var-val">${{world.vars[vName] ?? 0}}</span></div>`;
            }}
            varOverlay.innerHTML = overlayHtml;
        }}

        function debounce(func, wait) {{
            let timeout;
            return function(...args) {{
                clearTimeout(timeout);
                timeout = setTimeout(() => func.apply(this, args), wait);
            }};
        }}

        tick(performance.now());
    </script>
</body>
</html>
"##,
        name = project_name
    )
}
