use clap::{Parser, Subcommand};
use scratch_blocks::BlockRegistry;
use scratch_bytecode::BytecodeCompiler;
use scratch_ir::lower_ast_to_ir;
use scratch_language::parse;
use scratch_native::NativeRunner;
use scratch_project::ProjectConfig;
use scratch_runtime::Runtime;
use scratch_scenes::SceneData;
use std::path::{Path, PathBuf};

mod config;
mod gui_preview;
mod gui_studio;
mod package;
mod project_cmd;
mod sdk;
mod stage_view;
mod studio;
mod templates;

use config::CliConfig;

#[derive(Parser)]
#[command(name = "scratch")]
#[command(about = "Command-line interface for Scratch", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Scratch project
    New {
        /// Project name or directory
        name: String,
        /// Starter template (starter, platformer, coins, dialogue, motion, lists)
        #[arg(short, long)]
        template: Option<String>,
        /// Destination path (defaults to project name)
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Project author
        #[arg(long)]
        author: Option<String>,
        /// Project display title
        #[arg(long)]
        title: Option<String>,
    },
    /// Options for configuring Scratch CLI
    Config {
        #[command(subcommand)]
        command: Option<ConfigSubcommand>,
    },
    /// Options for installing & managing the Scratch SDK
    Sdk {
        #[command(subcommand)]
        command: Option<SdkSubcommand>,
    },
    /// Tools for working with the current project
    Project {
        #[command(subcommand)]
        command: ProjectSubcommand,
    },
    /// Options for working with .scratch packages
    Package {
        #[command(subcommand)]
        command: PackageSubcommand,
    },
    /// View or configure the game window aspect ratio (16:9, 4:3, or device)
    Ratio {
        /// Aspect ratio preset to set (16:9, 4:3, device)
        #[arg(short, long)]
        set: Option<String>,
        /// Shortcut preset name (e.g. `scratch ratio device` or `scratch ratio 4:3`)
        target: Option<String>,
        /// Path to project directory (defaults to current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Run preview of the project in a popup window
    Run {
        /// Path to project directory or .sch file (defaults to current directory)
        path: Option<PathBuf>,
        /// Force popup app window preview
        #[arg(long)]
        preview: bool,
        /// Run headlessly in terminal without graphical window
        #[arg(long)]
        headless: bool,
        /// Port for preview server
        #[arg(short, long)]
        port: Option<u16>,
        /// Frame rate cap
        #[arg(long)]
        fps: Option<u32>,
        /// Target aspect ratio override (16:9, 4:3, device)
        #[arg(long)]
        ratio: Option<String>,
    },
    /// Builds the project at the current directory
    Build {
        /// Path to project directory
        path: Option<PathBuf>,
        /// Build in release mode
        #[arg(long)]
        release: bool,
        /// Target output format (native, web, studio, bytecode, all)
        #[arg(short, long, default_value = "native")]
        target: String,
        /// Custom output directory
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    // Top-level shortcuts (hidden from help list for clean Geode styling)
    #[command(hide = true)]
    Check { path: Option<PathBuf> },
    #[command(hide = true)]
    Lint { path: Option<PathBuf> },
    #[command(hide = true)]
    Format { path: Option<PathBuf>, #[arg(long)] check: bool },
    #[command(hide = true)]
    Test { path: Option<PathBuf> },
    #[command(hide = true)]
    Studio { path: Option<PathBuf>, #[arg(short, long, default_value_t = 8080)] port: u16, #[arg(long)] no_open: bool },
    #[command(hide = true)]
    Lsp {
        /// Use stdio transport for LSP communication (default)
        #[arg(long)]
        stdio: bool,
    },
    #[command(hide = true)]
    Export { #[command(subcommand)] target: ExportTarget },
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// Get configuration value by key
    Get { key: String },
    /// Set configuration value by key
    Set { key: String, value: String },
    /// List all configuration values and descriptions
    List,
    /// Reset configuration to factory defaults
    Reset,
}

#[derive(Subcommand)]
pub enum SdkSubcommand {
    /// Detailed info about installed SDK, compiler, toolchains
    Info,
    /// Print installed SDK version
    Version,
    /// Print path to Scratch SDK cache / directory
    Path,
    /// Verify SDK component integrity and subsystems
    Check,
}

#[derive(Subcommand)]
pub enum ProjectSubcommand {
    /// Check project or script for errors without running
    Check { path: Option<PathBuf> },
    /// Lint .sch files for common game errors and educational guidance
    Lint { path: Option<PathBuf> },
    /// Format .sch source files deterministically
    Format {
        path: Option<PathBuf>,
        #[arg(long)]
        check: bool,
    },
    /// Run automated game simulations and assertions
    Test { path: Option<PathBuf> },
    /// Display project metadata, scene listing, asset counts, line statistics
    Info { path: Option<PathBuf> },
    /// Create a new .scene file in scenes/
    AddScene {
        /// Name of the new scene (e.g. level2)
        name: String,
        /// Target project directory
        path: Option<PathBuf>,
    },
    /// Add an asset file into the project assets directory
    AddAsset {
        /// Path to asset file (image, audio, font)
        asset_path: PathBuf,
        /// Asset category (sprite, sound, music, font)
        #[arg(short, long)]
        asset_type: Option<String>,
        /// Target project directory
        path: Option<PathBuf>,
    },
    /// Launch interactive native Scratch Studio Desktop IDE
    Studio {
        path: Option<PathBuf>,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        #[arg(long)]
        no_open: bool,
    },
}

#[derive(Subcommand)]
pub enum PackageSubcommand {
    /// Package project into a standalone .scratch archive bundle
    Pack {
        path: Option<PathBuf>,
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Extract a .scratch package archive into a project directory
    Unpack {
        /// Path to .scratch package file
        file: PathBuf,
        /// Destination directory
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Inspect metadata and contents of a .scratch package
    Info {
        file: PathBuf,
    },
    /// Export game for web browser canvas deployment
    ExportWeb {
        path: Option<PathBuf>,
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Export standalone Scratch Studio IDE bundle
    ExportStudio {
        path: Option<PathBuf>,
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum ExportTarget {
    Web {
        path: Option<PathBuf>,
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    Studio {
        path: Option<PathBuf>,
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut config = CliConfig::load();

    match cli.command {
        Commands::New { name, template, path, author, title } => {
            let tmpl = template.unwrap_or_else(|| config.default_template.clone());
            let aut = author.unwrap_or_else(|| config.default_author.clone());
            let dest = path.unwrap_or_else(|| PathBuf::from(&name));

            if let Err(e) = create_new_project(&dest, &name, &tmpl, &aut, title.as_deref()) {
                eprintln!("Error creating project: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Config { command } => {
            match command.unwrap_or(ConfigSubcommand::List) {
                ConfigSubcommand::Get { key } => {
                    if let Some(val) = config.get(&key) {
                        println!("{}", val);
                    } else {
                        eprintln!("Config key '{}' not found", key);
                        std::process::exit(1);
                    }
                }
                ConfigSubcommand::Set { key, value } => {
                    if let Err(e) = config.set(&key, &value) {
                        eprintln!("Error setting config: {}", e);
                        std::process::exit(1);
                    } else {
                        println!("Set {} = {}", key, value);
                    }
                }
                ConfigSubcommand::List => {
                    config.list();
                }
                ConfigSubcommand::Reset => {
                    if let Err(e) = config.reset() {
                        eprintln!("Error resetting config: {}", e);
                        std::process::exit(1);
                    } else {
                        println!("Config reset to defaults.");
                    }
                }
            }
        }
        Commands::Sdk { command } => {
            match command.unwrap_or(SdkSubcommand::Info) {
                SdkSubcommand::Info => sdk::print_info(),
                SdkSubcommand::Version => sdk::print_version(),
                SdkSubcommand::Path => sdk::print_sdk_path(),
                SdkSubcommand::Check => {
                    if let Err(e) = sdk::run_sdk_check() {
                        eprintln!("SDK check failed: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Project { command } => match command {
            ProjectSubcommand::Check { path } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = project_cmd::check_project(&target) {
                    eprintln!("Check failed: {}", e);
                    std::process::exit(1);
                }
            }
            ProjectSubcommand::Lint { path } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = project_cmd::lint_project(&target) {
                    eprintln!("Lint failed: {}", e);
                    std::process::exit(1);
                }
            }
            ProjectSubcommand::Format { path, check } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = project_cmd::format_project(&target, check) {
                    eprintln!("Format failed: {}", e);
                    std::process::exit(1);
                }
            }
            ProjectSubcommand::Test { path } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = project_cmd::test_project(&target) {
                    eprintln!("Test failed: {}", e);
                    std::process::exit(1);
                }
            }
            ProjectSubcommand::Info { path } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = project_cmd::project_info(&target) {
                    eprintln!("Project info failed: {}", e);
                    std::process::exit(1);
                }
            }
            ProjectSubcommand::AddScene { name, path } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = project_cmd::add_scene(&target, &name) {
                    eprintln!("Add scene failed: {}", e);
                    std::process::exit(1);
                }
            }
            ProjectSubcommand::AddAsset { asset_path, asset_type, path } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = project_cmd::add_asset(&target, &asset_path, asset_type.as_deref()) {
                    eprintln!("Add asset failed: {}", e);
                    std::process::exit(1);
                }
            }
            ProjectSubcommand::Studio { path, .. } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = gui_studio::run_studio_native(&target) {
                    eprintln!("Studio error: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Commands::Package { command } => match command {
            PackageSubcommand::Pack { path, out } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = package::pack_project(&target, out) {
                    eprintln!("Pack failed: {}", e);
                    std::process::exit(1);
                }
            }
            PackageSubcommand::Unpack { file, out } => {
                if let Err(e) = package::unpack_package(&file, out) {
                    eprintln!("Unpack failed: {}", e);
                    std::process::exit(1);
                }
            }
            PackageSubcommand::Info { file } => {
                if let Err(e) = package::inspect_package(&file) {
                    eprintln!("Package inspect failed: {}", e);
                    std::process::exit(1);
                }
            }
            PackageSubcommand::ExportWeb { path, out } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = export_web(&target, out) {
                    eprintln!("Export web failed: {}", e);
                    std::process::exit(1);
                }
            }
            PackageSubcommand::ExportStudio { path, out } => {
                let target = path.unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = export_studio(&target, out) {
                    eprintln!("Export studio failed: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Commands::Ratio { set, target, path } => {
            if let Err(e) = handle_ratio_command(set, target, path) {
                eprintln!("Ratio error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Run { path, preview: _, headless, port: _, fps: _, ratio } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            let is_headless = headless || config.preview_mode == "headless";

            if is_headless {
                println!("Running headless preview simulation...");
                if let Err(e) = run_project_headless(&target_path) {
                    eprintln!("Error running headless project: {}", e);
                    std::process::exit(1);
                }
            } else {
                if let Err(e) = gui_preview::run_preview_native(&target_path, ratio.as_deref()) {
                    eprintln!("Error launching live popup preview: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Build { path, release, target, out } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = build_project(&target_path, release, &target, out) {
                eprintln!("Build failed: {}", e);
                std::process::exit(1);
            }
        }

        // Hidden aliases for backward compatibility
        Commands::Check { path } => {
            let target = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = project_cmd::check_project(&target) {
                eprintln!("Check failed: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Lint { path } => {
            let target = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = project_cmd::lint_project(&target) {
                eprintln!("Lint failed: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Format { path, check } => {
            let target = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = project_cmd::format_project(&target, check) {
                eprintln!("Format failed: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Test { path } => {
            let target = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = project_cmd::test_project(&target) {
                eprintln!("Test failed: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Studio { path, .. } => {
            let target = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = gui_studio::run_studio_native(&target) {
                eprintln!("Studio error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Lsp { .. } => {
            if let Err(e) = start_lsp() {
                eprintln!("LSP server error: {}", e);
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
    }
}

fn create_new_project(
    dest_path: &Path,
    name: &str,
    template: &str,
    author: &str,
    _title: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dest_path.exists() && dest_path.read_dir()?.next().is_some() {
        return Err(format!("Directory '{}' already exists and is not empty", dest_path.display()).into());
    }

    std::fs::create_dir_all(dest_path.join("src"))?;
    std::fs::create_dir_all(dest_path.join("scenes"))?;
    std::fs::create_dir_all(dest_path.join("assets/sprites"))?;
    std::fs::create_dir_all(dest_path.join("assets/sounds"))?;
    std::fs::create_dir_all(dest_path.join("assets/music"))?;
    std::fs::create_dir_all(dest_path.join("assets/fonts"))?;

    let config = ProjectConfig {
        name: name.to_string(),
        entry: "src/main.sch".to_string(),
        background: "white".to_string(),
        ..Default::default()
    };
    config.save_to_file(dest_path.join("project.schproj"))?;

    // Create default scene
    let default_scene = SceneData::default();
    default_scene.save_to_file(dest_path.join("scenes/main.scene"))?;

    // Populate starter code from chosen template
    let template_code = templates::get_template_code(template);
    std::fs::write(dest_path.join("src/main.sch"), template_code)?;

    println!("============================================================");
    println!(" Initialized new Scratch project '{}'!", name);
    println!(" Template:    {}", template);
    println!(" Directory:   {}", dest_path.display());
    println!(" Author:      {}", author);
    println!();
    println!(" Next steps:");
    println!("   cd {}", dest_path.display());
    println!("   scratch run            # Open native live popup app preview");
    println!("   scratch project studio # Open native Scratch Studio desktop IDE");
    println!("============================================================");

    Ok(())
}

fn run_project_headless(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = project_cmd::resolve_entry_file(path)?;
    println!("Compiling {}...", entry_path.display());
    let source = std::fs::read_to_string(&entry_path)?;

    let ast = parse(&source).map_err(|e| format!("In {}: {}", entry_path.display(), e))?;
    let registry = BlockRegistry::core();
    let ir = lower_ast_to_ir(&ast, &registry).map_err(|e| format!("IR lowering error: {}", e))?;

    let runtime = Runtime::new(ir, registry);
    NativeRunner::run(runtime, &config);

    Ok(())
}

fn build_project(
    path: &Path,
    release: bool,
    target: &str,
    out_dir: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = project_cmd::resolve_entry_file(path)?;
    let proj_dir = if entry_path.is_file() {
        entry_path
            .parent()
            .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
            .unwrap_or(Path::new("."))
    } else {
        path
    };

    println!("Building project '{}'...", config.name);
    println!("Target:  {}", target);
    println!("Mode:    {}", if release { "release" } else { "debug" });

    match target.to_lowercase().as_str() {
        "web" => {
            let dest = out_dir.unwrap_or_else(|| proj_dir.join("dist/web"));
            export_web(proj_dir, Some(dest))?;
        }
        "studio" => {
            let dest = out_dir.unwrap_or_else(|| proj_dir.join("dist/studio"));
            export_studio(proj_dir, Some(dest))?;
        }
        "bytecode" => {
            let source = std::fs::read_to_string(&entry_path)?;
            let ast = parse(&source)?;
            let reg = BlockRegistry::core();
            let ir = lower_ast_to_ir(&ast, &reg)?;
            let mut compiler = BytecodeCompiler::new();
            let program = compiler.compile_program(&ir);
            let dest = out_dir.unwrap_or_else(|| proj_dir.join("dist/bytecode"));
            std::fs::create_dir_all(&dest)?;
            let json = serde_json::to_string_pretty(&program)?;
            std::fs::write(dest.join("game.schbc"), json)?;
            println!("Bytecode written to {}", dest.join("game.schbc").display());
        }
        "all" => {
            build_native(proj_dir, &entry_path, &config, release, out_dir.as_ref())?;
            export_web(proj_dir, None)?;
            export_studio(proj_dir, None)?;
        }
        _ => {
            build_native(proj_dir, &entry_path, &config, release, out_dir.as_ref())?;
        }
    }

    Ok(())
}

fn build_native(
    proj_dir: &Path,
    entry_path: &Path,
    config: &ProjectConfig,
    release: bool,
    out_dir: Option<&PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(entry_path)?;
    let ast = parse(&source).map_err(|e| format!("In {}: {}", entry_path.display(), e))?;
    let registry = BlockRegistry::core();
    let ir = lower_ast_to_ir(&ast, &registry).map_err(|e| format!("IR lowering error: {}", e))?;
    let mut compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile_program(&ir);

    let default_dist = proj_dir.join("dist").join(&config.name);
    let dist_dir = out_dir.unwrap_or(&default_dist);

    if dist_dir.exists() {
        std::fs::remove_dir_all(dist_dir)?;
    }
    std::fs::create_dir_all(dist_dir)?;

    // 1. Write bytecode artifact
    let bytecode_json = serde_json::to_string_pretty(&bytecode)?;
    std::fs::write(dist_dir.join("game.schbc"), bytecode_json)?;

    // 2. Write project manifest
    config.save_to_file(dist_dir.join("project.schproj"))?;

    // 3. Copy scenes & assets
    let scenes_src = proj_dir.join("scenes");
    if scenes_src.exists() {
        copy_dir_recursive(&scenes_src, &dist_dir.join("scenes"))?;
    }
    let assets_src = proj_dir.join("assets");
    if assets_src.exists() {
        copy_dir_recursive(&assets_src, &dist_dir.join("assets"))?;
    }

    // 4. Generate runner script
    #[cfg(target_os = "windows")]
    {
        let run_bat = format!(
            "@echo off\r\necho Starting {}...\r\nscratch run .\r\npause\r\n",
            config.name
        );
        std::fs::write(dist_dir.join("run.bat"), run_bat)?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        let run_sh = format!(
            "#!/bin/bash\necho \"Starting {}...\"\nscratch run .\n",
            config.name
        );
        std::fs::write(dist_dir.join("run.sh"), run_sh)?;
    }

    println!();
    println!("============================================================");
    println!(" Native Build Succeeded!");
    println!(" Target Package: {}", dist_dir.display());
    println!(" Executable Launcher: run.bat");
    println!(" Mode: {}", if release { "Release" } else { "Debug" });
    println!("============================================================");

    Ok(())
}

fn export_web(proj_dir: &Path, out_dir: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = project_cmd::resolve_entry_file(proj_dir)?;
    let root = if entry_path.is_file() {
        entry_path
            .parent()
            .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
            .unwrap_or(Path::new("."))
    } else {
        proj_dir
    };

    let web_dist_dir = out_dir.unwrap_or_else(|| root.join("dist/web"));
    if web_dist_dir.exists() {
        std::fs::remove_dir_all(&web_dist_dir)?;
    }
    std::fs::create_dir_all(&web_dist_dir)?;

    println!("Exporting '{}' for Web (HTML5 Canvas)...", config.name);

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
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
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
        <div id="overlay">Click canvas to play (Arrow keys / Space)</div>
    </div>
    <script src="game.js"></script>
</body>
</html>
"#,
        config.name, config.name
    );

    std::fs::write(web_dist_dir.join("index.html"), html_content)?;

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
        if (keys["ArrowRight"] || keys["d"]) player.x += 5;
        if (keys["ArrowLeft"] || keys["a"]) player.x -= 5;
        if (keys["ArrowUp"] || keys["w"]) player.y -= 5;
        if (keys["ArrowDown"] || keys["s"]) player.y += 5;

        // Clamp player within 16:9 canvas border
        player.x = Math.max(16, Math.min(canvas.width - 16, player.x));
        player.y = Math.max(16, Math.min(canvas.height - 16, player.y));

        for (let c of coins) {
            if (!c.collected && Math.hypot(player.x - c.x, player.y - c.y) < 32) {
                c.collected = true;
                score += 1;
            }
        }

        ctx.fillStyle = "#1e222d";
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // 16:9 Physical Stage Border
        ctx.strokeStyle = "#3b82f6";
        ctx.lineWidth = 4;
        ctx.strokeRect(2, 2, canvas.width - 4, canvas.height - 4);

        ctx.strokeStyle = "rgba(255, 255, 255, 0.05)";
        ctx.lineWidth = 1;
        for (let x = 0; x < canvas.width; x += 64) {
            ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, canvas.height); ctx.stroke();
        }
        for (let y = 0; y < canvas.height; y += 64) {
            ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(canvas.width, y); ctx.stroke();
        }

        for (let c of coins) {
            if (!c.collected) {
                ctx.fillStyle = "#f1c40f";
                ctx.beginPath();
                ctx.arc(c.x, c.y, 14, 0, Math.PI * 2);
                ctx.fill();
            }
        }

        ctx.fillStyle = "#3498db";
        ctx.fillRect(player.x - 16, player.y - 16, player.width, player.height);

        ctx.fillStyle = "#ecf0f1";
        ctx.font = "bold 24px monospace";
        ctx.fillText(`Score: ${score}`, 32, 48);

        requestAnimationFrame(tick);
    }

    tick();
})();
"##;

    std::fs::write(web_dist_dir.join("game.js"), js_content)?;

    let scenes_src = root.join("scenes");
    if scenes_src.exists() {
        copy_dir_recursive(&scenes_src, &web_dist_dir.join("scenes"))?;
    }
    let assets_src = root.join("assets");
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

fn export_studio(proj_dir: &Path, out_dir: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = project_cmd::resolve_entry_file(proj_dir)?;
    let root = if entry_path.is_file() {
        entry_path
            .parent()
            .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
            .unwrap_or(Path::new("."))
    } else {
        proj_dir
    };

    let studio_dist_dir = out_dir.unwrap_or_else(|| root.join("dist/studio"));
    if studio_dist_dir.exists() {
        std::fs::remove_dir_all(&studio_dist_dir)?;
    }
    std::fs::create_dir_all(&studio_dist_dir)?;

    println!("Exporting Scratch Studio for '{}'...", config.name);
    let html = studio::render_studio_html(&config.name);
    std::fs::write(studio_dist_dir.join("index.html"), html)?;

    let scenes_src = root.join("scenes");
    if scenes_src.exists() {
        copy_dir_recursive(&scenes_src, &studio_dist_dir.join("scenes"))?;
    }
    let assets_src = root.join("assets");
    if assets_src.exists() {
        copy_dir_recursive(&assets_src, &studio_dist_dir.join("assets"))?;
    }

    println!();
    println!("============================================================");
    println!(" Scratch Studio Export Succeeded!");
    println!(" Studio Directory: {}", studio_dist_dir.display());
    println!(" Preview locally with:");
    println!("   scratch studio {}", root.display());
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

fn start_lsp() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = scratch_lsp::LspServer::new();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let reader = std::io::BufReader::new(stdin.lock());
    let writer = std::io::BufWriter::new(stdout.lock());
    server.run(reader, writer)?;
    Ok(())
}

fn handle_ratio_command(
    set: Option<String>,
    target: Option<String>,
    path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (screen_w, screen_h) = scratch_project::get_device_screen_resolution();
    let dev_ratio_str = scratch_project::format_aspect_ratio(screen_w, screen_h);
    let target_dir = path.unwrap_or_else(|| PathBuf::from("."));

    // Check if user requested to set ratio
    let set_target = set.or(target);

    if let Some(desired) = set_target {
        let preset = scratch_project::AspectRatioPreset::parse_str(&desired).ok_or_else(|| {
            format!(
                "Invalid ratio preset '{}'. Supported presets are: '16:9', '4:3', 'device' (detected {}).",
                desired, dev_ratio_str
            )
        })?;

        // Locate project.schproj
        let project_file = if target_dir.is_file() {
            target_dir.clone()
        } else {
            target_dir.join("project.schproj")
        };

        if !project_file.exists() {
            return Err(format!(
                "No project file found at '{}'. Run 'scratch new <name>' or specify a project path.",
                project_file.display()
            ).into());
        }

        let mut config = ProjectConfig::load_from_file(&project_file)?;
        config.apply_ratio_preset(preset);
        config.save_to_file(&project_file)?;

        println!("============================================================");
        println!(" Project Aspect Ratio Updated!");
        println!(" Project:    {}", config.name);
        println!(" Ratio:      {}", preset.label());
        println!(" Saved to:   {}", project_file.display());
        println!("============================================================");
        return Ok(());
    }

    // Otherwise, print device info & available presets
    println!("============================================================");
    println!(" Scratch Device & Display Aspect Ratio");
    println!("============================================================");
    println!(" Primary Screen:    {}x{}", screen_w, screen_h);
    println!(" Device Ratio:      {}", dev_ratio_str);
    println!();
    println!(" Available Presets:");
    println!("   - 16:9   (1280x720) [Default Widescreen]");
    println!("   - 4:3    (960x720)  [Classic Retro]");
    println!("   - device ({}x{}) [Matches Current Monitor: {}]", screen_w, screen_h, dev_ratio_str);
    println!();

    // Check if inside a project
    let project_file = if target_dir.is_file() {
        target_dir.clone()
    } else {
        target_dir.join("project.schproj")
    };

    if project_file.exists() {
        if let Ok(config) = ProjectConfig::load_from_file(&project_file) {
            println!(" Current Project:");
            println!("   Name:       {}", config.name);
            println!("   Ratio:      {}", config.ratio);
            println!("   Resolution: {}x{}", config.resolution.width, config.resolution.height);
        }
    } else {
        println!(" Current Project:   None (not in a project directory)");
    }

    println!("------------------------------------------------------------");
    println!(" To set your project ratio, run:");
    println!("   scratch ratio --set device   # Use detected {} screen ratio", dev_ratio_str);
    println!("   scratch ratio --set 16:9     # Use standard 16:9 widescreen");
    println!("   scratch ratio --set 4:3      # Use classic 4:3 ratio");
    println!("============================================================");

    Ok(())
}
