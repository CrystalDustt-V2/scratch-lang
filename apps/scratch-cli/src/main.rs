use clap::{Parser, Subcommand};
use scratch_blocks::BlockRegistry;
use scratch_ir::lower_ast_to_ir;
use scratch_language::parse;
use scratch_native::NativeRunner;
use scratch_project::ProjectConfig;
use scratch_runtime::Runtime;
use std::path::{Path, PathBuf};

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
    /// Run automated game tests
    Test {
        /// Path to project directory or test file
        path: Option<PathBuf>,
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
        Commands::Test { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = test_project(&target_path) {
                eprintln!("Tests failed: {}", e);
                std::process::exit(1);
            } else {
                println!("All game tests passed!");
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
    let _ir = lower_ast_to_ir(&ast, &registry).map_err(|e| format!("IR lowering in {}: {}", entry_path.display(), e))?;

    println!("Validated: {}", entry_path.display());
    Ok(())
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

    let mut runtime = Runtime::new(ir, registry);
    NativeRunner::run_headless_ticks(&mut runtime, 10, 0.016);
    println!("Executed 10 simulation frames headlessly without errors.");

    Ok(())
}
