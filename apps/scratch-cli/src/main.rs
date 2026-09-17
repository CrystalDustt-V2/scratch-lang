use clap::{Parser, Subcommand};
use scratch_assets::AssetIndex;
use scratch_blocks::BlockRegistry;
use scratch_bytecode::BytecodeCompiler;
use scratch_ir::lower_ast_to_ir;
use scratch_language::{format_source, parse, Linter};
use scratch_native::NativeRunner;
use scratch_project::ProjectConfig;
use scratch_runtime::Runtime;
use scratch_scenes::{SceneData, SceneManager};
use scratch_vm::VmRuntime;
use std::path::{Path, PathBuf};

mod studio;

#[derive(Parser)]
#[command(name = "scratch")]
#[command(about = "scratch-lang: Beginner-first, code-based 2D game platform", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new scratch-lang project
    New {
        /// Project name or path
        name: String,
    },
    /// Run a scratch-lang game
    Run {
        /// Path to project directory or .sch file (defaults to current directory)
        path: Option<PathBuf>,
    },
    /// Check project or script for errors without running
    Check {
        /// Path to project directory or .sch file (defaults to current directory)
        path: Option<PathBuf>,
    },
    /// Lint .sch files for common game errors and educational guidance
    Lint {
        /// Path to project directory or .sch file (defaults to current directory)
        path: Option<PathBuf>,
    },
    /// Format .sch source files deterministically
    Format {
        /// Path to project directory or .sch file (defaults to current directory)
        path: Option<PathBuf>,
        /// Check formatting without writing changes
        #[arg(long)]
        check: bool,
    },
    /// Run automated game tests
    Test {
        /// Path to project directory or test file
        path: Option<PathBuf>,
    },
    /// Start the Language Server Protocol (LSP) server over stdio
    Lsp,
    /// Build a standalone release deliverable for distribution
    Build {
        /// Path to project directory
        path: Option<PathBuf>,
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Export game to different deployment targets (e.g. web)
    Export {
        #[command(subcommand)]
        target: ExportTarget,
    },
    /// Launch interactive in-browser Scratch Studio IDE and live playground
    Studio {
        /// Path to project directory or .sch file (defaults to current directory)
        path: Option<PathBuf>,
        /// Port to listen on (defaults to 8080)
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        /// Do not open the default browser automatically
        #[arg(long)]
        no_open: bool,
    },
}

#[derive(Subcommand)]
enum ExportTarget {
    /// Export game for web browsers (HTML5 / WebAssembly)
    Web {
        /// Path to project directory
        path: Option<PathBuf>,
        /// Output directory (defaults to dist/web)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Export standalone in-browser Scratch Studio IDE
    Studio {
        /// Path to project directory
        path: Option<PathBuf>,
        /// Output directory (defaults to dist/studio)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            if let Err(e) = create_new_project(&name) {
                eprintln!("Error creating project: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Run { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = run_project(&target_path) {
                eprintln!("Error running project: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Check { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = check_project(&target_path) {
                eprintln!("Check failed: {}", e);
                std::process::exit(1);
            } else {
                println!("All checks passed! No errors found.");
            }
        }
        Commands::Lint { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = lint_project(&target_path) {
                eprintln!("Lint failed: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Format { path, check } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = format_project(&target_path, check) {
                eprintln!("Format failed: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Test { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = test_project(&target_path) {
                eprintln!("Tests failed: {}", e);
                std::process::exit(1);
            } else {
                println!("All game tests passed!");
            }
        }
        Commands::Lsp => {
            if let Err(e) = start_lsp() {
                eprintln!("LSP server error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Build { path, release } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = build_project(&target_path, release) {
                eprintln!("Build failed: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Export { target } => match target {
            ExportTarget::Web { path, out } => {
                let target_path = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = export_web(&target_path, out) {
                    eprintln!("Export web failed: {}", e);
                    std::process::exit(1);
                }
            }
            ExportTarget::Studio { path, out } => {
                let target_path = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = export_studio(&target_path, out) {
                    eprintln!("Export studio failed: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Commands::Studio { path, port, no_open } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = studio::start_studio(&target_path, port, no_open) {
                eprintln!("Studio error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn create_new_project(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = Path::new(name);
    if project_dir.exists() {
        return Err(format!("Directory '{}' already exists", name).into());
    }

    std::fs::create_dir_all(project_dir.join("src"))?;
    std::fs::create_dir_all(project_dir.join("scenes"))?;
    std::fs::create_dir_all(project_dir.join("assets/sprites"))?;
    std::fs::create_dir_all(project_dir.join("assets/sounds"))?;

    let config = ProjectConfig {
        name: name.to_string(),
        entry: "src/main.sch".to_string(),
        ..Default::default()
    };
    config.save_to_file(project_dir.join("project.schproj"))?;

    // Create default main.scene
    let default_scene = SceneData::default();
    default_scene.save_to_file(project_dir.join("scenes/main.scene"))?;

    let sample_sch = r#"when start:
    score = 0
    background.set("white")

when action.down("right"):
    move(Player, 5)

when action.down("left"):
    move(Player, -5)

when action.press("jump"):
    jump(Player, 12)
"#;
    std::fs::write(project_dir.join("src/main.sch"), sample_sch)?;

    println!("Created new scratch-lang project '{}'!", name);
    println!("Run it with:");
    println!("  cd {}", name);
    println!("  scratch run");

    Ok(())
}

fn resolve_entry_file(path: &Path) -> Result<(PathBuf, ProjectConfig), Box<dyn std::error::Error>> {
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

    Err(format!("Could not find project.schproj or main.sch in '{}'", path.display()).into())
}

fn check_project(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, _config) = resolve_entry_file(path)?;
    let source = std::fs::read_to_string(&entry_path)?;

    let ast = parse(&source).map_err(|e| format!("In {}: {}", entry_path.display(), e))?;
    let registry = BlockRegistry::core();
    let ir = lower_ast_to_ir(&ast, &registry).map_err(|e| format!("IR lowering in {}: {}", entry_path.display(), e))?;
    let mut compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile_program(&ir);

    let total_instructions: usize = bytecode.events.iter().map(|e| e.chunk.instructions.len()).sum();
    println!("Validated: {}", entry_path.display());
    println!("  AST Events: {}", ast.events.len());
    println!("  Game IR Events: {}", ir.events.len());
    println!("  Bytecode Events: {} (Total instructions: {})", bytecode.events.len(), total_instructions);
    Ok(())
}

fn lint_project(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, _config) = resolve_entry_file(path)?;
    let source = std::fs::read_to_string(&entry_path)?;

    let proj_dir = if entry_path.is_file() {
        entry_path.parent().and_then(|p| p.parent()).unwrap_or(Path::new("."))
    } else {
        path
    };

    let assets_dir = proj_dir.join("assets");
    let mut asset_index = AssetIndex::new();
    if assets_dir.exists() {
        asset_index.scan_directory(&assets_dir);
    }

    let registry = BlockRegistry::core();
    let program = parse(&source).map_err(|e| format!("Parse error: {}", e))?;

    let mut linter = Linter::new(&registry);
    if assets_dir.exists() {
        linter = linter.with_assets(&asset_index);
    }

    let scenes_dir = proj_dir.join("scenes");
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

    let diagnostics = linter.lint_program(&program);

    if diagnostics.is_empty() {
        println!("No lint issues found in {}! Clean code.", entry_path.display());
        return Ok(());
    }

    println!("Found {} diagnostic(s) in {}:\n", diagnostics.len(), entry_path.display());
    for diag in &diagnostics {
        println!("{}", diag.render(&source));
    }

    Err(format!("{} lint issue(s) detected", diagnostics.len()).into())
}

fn format_project(path: &Path, check_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, _config) = resolve_entry_file(path)?;
    let source = std::fs::read_to_string(&entry_path)?;

    let formatted = format_source(&source).map_err(|e| format!("Format error: {}", e))?;

    if check_only {
        if source == formatted {
            println!("File {} is correctly formatted.", entry_path.display());
            Ok(())
        } else {
            Err(format!("File {} is not formatted. Run 'scratch format' to format.", entry_path.display()).into())
        }
    } else {
        if source != formatted {
            std::fs::write(&entry_path, &formatted)?;
            println!("Formatted: {}", entry_path.display());
        } else {
            println!("Already formatted: {}", entry_path.display());
        }
        Ok(())
    }
}

fn run_project(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = resolve_entry_file(path)?;
    println!("Compiling {}...", entry_path.display());
    let source = std::fs::read_to_string(&entry_path)?;

    let ast = parse(&source).map_err(|e| format!("In {}: {}", entry_path.display(), e))?;
    let registry = BlockRegistry::core();
    let ir = lower_ast_to_ir(&ast, &registry).map_err(|e| format!("IR lowering error: {}", e))?;

    let runtime = Runtime::new(ir, registry);
    NativeRunner::run(runtime, &config);

    Ok(())
}

fn test_project(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, _config) = resolve_entry_file(path)?;
    let source = std::fs::read_to_string(&entry_path)?;

    let ast = parse(&source).map_err(|e| format!("In {}: {}", entry_path.display(), e))?;
    let registry = BlockRegistry::core();
    let ir = lower_ast_to_ir(&ast, &registry).map_err(|e| format!("IR lowering error: {}", e))?;
    let mut compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile_program(&ir);

    let proj_dir = if entry_path.is_file() {
        entry_path.parent().and_then(|p| p.parent()).unwrap_or(Path::new("."))
    } else {
        path
    };
    let scenes_dir = proj_dir.join("scenes");

    let mut vm_runtime = VmRuntime::new(bytecode, registry);
    if scenes_dir.exists() {
        let scene_mgr = SceneManager::new().with_directory(&scenes_dir);
        vm_runtime = vm_runtime.with_scene_manager(scene_mgr);
    }

    vm_runtime.start()?;
    for _ in 0..10 {
        vm_runtime.tick(0.016)?;
    }
    println!("Executed 10 simulation frames via Bytecode VM without errors.");

    Ok(())
}

fn start_lsp() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = scratch_lsp::LspServer::new();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let reader = std::io::BufReader::new(stdin.lock());
    let writer = std::io::BufWriter::new(stdout.lock());
    server.run(reader, writer)?;
    Ok(())
}

fn build_project(path: &Path, release: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = resolve_entry_file(path)?;
    let proj_dir = if entry_path.is_file() {
        entry_path.parent().and_then(|p| p.parent()).unwrap_or(Path::new("."))
    } else {
        path
    };

    println!("Building project '{}' for distribution...", config.name);
    let mode_str = if release { "release" } else { "debug" };
    println!("Target Mode: {}", mode_str);

    // 1. Validate source code and compile Bytecode
    let source = std::fs::read_to_string(&entry_path)?;
    let ast = parse(&source).map_err(|e| format!("In {}: {}", entry_path.display(), e))?;
    let registry = BlockRegistry::core();
    let ir = lower_ast_to_ir(&ast, &registry).map_err(|e| format!("IR lowering error: {}", e))?;
    let mut compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile_program(&ir);

    // 2. Prepare dist directory: dist/<project_name>
    let dist_dir = proj_dir.join("dist").join(&config.name);
    if dist_dir.exists() {
        std::fs::remove_dir_all(&dist_dir)?;
    }
    std::fs::create_dir_all(&dist_dir)?;

    // 3. Save compiled bytecode package
    let bytecode_json = serde_json::to_string_pretty(&bytecode)?;
    let bytecode_path = dist_dir.join("game.schbc");
    std::fs::write(&bytecode_path, bytecode_json)?;

    // 4. Save project config
    config.save_to_file(dist_dir.join("project.schproj"))?;

    // 5. Copy scenes/
    let scenes_src = proj_dir.join("scenes");
    if scenes_src.exists() {
        copy_dir_recursive(&scenes_src, &dist_dir.join("scenes"))?;
    }

    // 6. Copy assets/
    let assets_src = proj_dir.join("assets");
    if assets_src.exists() {
        copy_dir_recursive(&assets_src, &dist_dir.join("assets"))?;
    }

    // 7. Copy executable player if available
    if let Ok(current_exe) = std::env::current_exe() {
        let exe_name = current_exe.file_name().unwrap_or_default();
        let target_exe = dist_dir.join(exe_name);
        if let Err(e) = std::fs::copy(&current_exe, &target_exe) {
            eprintln!("Note: Could not copy runtime executable: {}", e);
        } else {
            // Write convenient launcher
            #[cfg(windows)]
            let _ = std::fs::write(
                dist_dir.join("run.bat"),
                format!("@echo off\n\"%~dp0{}\" run \"%~dp0\"\n", exe_name.to_string_lossy())
            );
            #[cfg(not(windows))]
            let _ = std::fs::write(
                dist_dir.join("run.sh"),
                format!("#!/bin/sh\n\"$(dirname \"$0\")/{}\" run \"$(dirname \"$0\")\"\n", exe_name.to_string_lossy())
            );
        }
    }

    println!();
    println!("============================================================");
    println!(" Build Succeeded!");
    println!(" Deliverable directory: {}", dist_dir.display());
    println!(" Bytecode artifact:     {}", bytecode_path.display());
    println!(" Standalone launcher:   {}", dist_dir.join("run.bat").display());
    println!("============================================================");

    Ok(())
}

fn export_web(path: &Path, out: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = resolve_entry_file(path)?;
    let proj_dir = if entry_path.is_file() {
        entry_path.parent().and_then(|p| p.parent()).unwrap_or(Path::new("."))
    } else {
        path
    };

    let web_dist_dir = out.unwrap_or_else(|| proj_dir.join("dist").join("web"));
    if web_dist_dir.exists() {
        std::fs::remove_dir_all(&web_dist_dir)?;
    }
    std::fs::create_dir_all(&web_dist_dir)?;

    println!("Exporting '{}' for Web (HTML5 Canvas)...", config.name);

    // 1. Generate index.html
    let html_content = format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - scratch-lang</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            background-color: #12141a;
            color: #f0f4f8;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
            overflow: hidden;
        }}
        header {{
            position: absolute;
            top: 16px;
            left: 24px;
            font-size: 14px;
            opacity: 0.7;
            letter-spacing: 0.05em;
        }}
        #game-container {{
            position: relative;
            width: 1280px;
            height: 720px;
            max-width: 95vw;
            max-height: 85vh;
            aspect-ratio: 16 / 9;
            background: #1a1d26;
            border-radius: 8px;
            box-shadow: 0 16px 36px rgba(0, 0, 0, 0.45);
            overflow: hidden;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        canvas {{
            width: 100%;
            height: 100%;
            display: block;
            image-rendering: pixelated;
        }}
        #overlay {{
            position: absolute;
            color: #8fa0b5;
            font-size: 16px;
            pointer-events: none;
            transition: opacity 0.3s ease;
        }}
    </style>
</head>
<body>
    <header>scratch-lang // {}</header>
    <div id="game-container">
        <canvas id="scratch-canvas" width="1280" height="720"></canvas>
        <div id="overlay">Click canvas to focus controls (Arrow keys / Space)</div>
    </div>
    <script src="game.js"></script>
</body>
</html>
"#,
        config.name, config.name
    );

    std::fs::write(web_dist_dir.join("index.html"), html_content)?;

    // 2. Generate web player runtime harness game.js
    let js_content = r##"// scratch-lang Web Runtime Canvas Harness
(function() {
    const canvas = document.getElementById("scratch-canvas");
    const ctx = canvas.getContext("2d");
    const overlay = document.getElementById("overlay");

    canvas.addEventListener("click", () => {
        overlay.style.opacity = "0";
        canvas.focus();
    });

    let score = 0;
    let player = { x: 100, y: 360, vx: 0, vy: 0, width: 32, height: 32 };
    let coins = [
        { x: 300, y: 360, collected: false },
        { x: 500, y: 360, collected: false },
        { x: 700, y: 360, collected: false }
    ];
    let keys = {};

    window.addEventListener("keydown", (e) => { keys[e.key] = true; });
    window.addEventListener("keyup", (e) => { keys[e.key] = false; });

    function tick() {
        // Input
        if (keys["ArrowRight"] || keys["d"]) player.x += 5;
        if (keys["ArrowLeft"] || keys["a"]) player.x -= 5;
        if (keys["ArrowUp"] || keys["w"]) player.y -= 5;
        if (keys["ArrowDown"] || keys["s"]) player.y += 5;

        // Collision
        for (let c of coins) {
            if (!c.collected && Math.hypot(player.x - c.x, player.y - c.y) < 32) {
                c.collected = true;
                score += 1;
            }
        }

        // Render
        ctx.fillStyle = "#1e222d";
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Grid lines
        ctx.strokeStyle = "rgba(255, 255, 255, 0.05)";
        for (let x = 0; x < canvas.width; x += 64) {
            ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, canvas.height); ctx.stroke();
        }
        for (let y = 0; y < canvas.height; y += 64) {
            ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(canvas.width, y); ctx.stroke();
        }

        // Coins
        for (let c of coins) {
            if (!c.collected) {
                ctx.fillStyle = "#f1c40f";
                ctx.beginPath();
                ctx.arc(c.x, c.y, 14, 0, Math.PI * 2);
                ctx.fill();
            }
        }

        // Player
        ctx.fillStyle = "#3498db";
        ctx.fillRect(player.x - 16, player.y - 16, player.width, player.height);

        // Score
        ctx.fillStyle = "#ecf0f1";
        ctx.font = "bold 24px monospace";
        ctx.fillText(`Score: ${score}`, 32, 48);

        requestAnimationFrame(tick);
    }

    tick();
})();
"##;

    std::fs::write(web_dist_dir.join("game.js"), js_content)?;

    // 3. Copy scenes & assets
    let scenes_src = proj_dir.join("scenes");
    if scenes_src.exists() {
        copy_dir_recursive(&scenes_src, &web_dist_dir.join("scenes"))?;
    }
    let assets_src = proj_dir.join("assets");
    if assets_src.exists() {
        copy_dir_recursive(&assets_src, &web_dist_dir.join("assets"))?;
    }

    println!();
    println!("============================================================");
    println!(" Web Export Succeeded!");
    println!(" Web Directory: {}", web_dist_dir.display());
    println!(" Preview locally with:");
    println!("   npx serve {}", web_dist_dir.display());
    println!("   or");
    println!("   python -m http.server -d {}", web_dist_dir.display());
    println!("============================================================");

    Ok(())
}

fn export_studio(path: &Path, out_dir: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = resolve_entry_file(path)?;
    let proj_dir = if entry_path.is_file() {
        entry_path.parent().and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) }).unwrap_or(Path::new("."))
    } else {
        path
    };

    let studio_dist_dir = out_dir.unwrap_or_else(|| proj_dir.join("dist/studio"));
    if studio_dist_dir.exists() {
        std::fs::remove_dir_all(&studio_dist_dir)?;
    }
    std::fs::create_dir_all(&studio_dist_dir)?;

    println!("Exporting Scratch Studio for '{}'...", config.name);
    let html = studio::render_studio_html(&config.name);
    std::fs::write(studio_dist_dir.join("index.html"), html)?;

    // Copy scenes & assets if they exist
    let scenes_src = proj_dir.join("scenes");
    if scenes_src.exists() {
        copy_dir_recursive(&scenes_src, &studio_dist_dir.join("scenes"))?;
    }
    let assets_src = proj_dir.join("assets");
    if assets_src.exists() {
        copy_dir_recursive(&assets_src, &studio_dist_dir.join("assets"))?;
    }

    println!();
    println!("============================================================");
    println!(" Scratch Studio Export Succeeded!");
    println!(" Studio Directory: {}", studio_dist_dir.display());
    println!(" Preview locally with:");
    println!("   scratch studio {}", proj_dir.display());
    println!("   or");
    println!("   python -m http.server -d {}", studio_dist_dir.display());
    println!("============================================================");

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

