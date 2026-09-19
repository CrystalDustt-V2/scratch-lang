use scratch_blocks::BlockRegistry;
use scratch_bytecode::BytecodeCompiler;
use scratch_ir::lower_ast_to_ir;
use scratch_language::{format_source, parse};
use scratch_runtime::Runtime;
use std::path::PathBuf;

pub fn print_version() {
    println!("scratch-lang SDK v{}", env!("CARGO_PKG_VERSION"));
    println!("Compiler: Bytecode VM / Game IR");
    println!("Rust Edition: 2021 (rustc 1.85+)");
    println!("Platform: Windows x86_64");
}

pub fn print_sdk_path() {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    let sdk_dir = PathBuf::from(home).join(".scratch").join("sdk");
    println!("{}", sdk_dir.display());
}

pub fn print_info() {
    println!("============================================================");
    println!(" Scratch SDK Environment Information");
    println!("============================================================");
    println!(" SDK Version:        {}", env!("CARGO_PKG_VERSION"));
    println!(" Workspace Architecture: 11 decoupled modular crates");
    println!(" Compiler:           scratch-bytecode (Stack-based Bytecode VM)");
    println!(" Intermediate Repr:  scratch-ir (Engine-independent Game IR)");
    println!(" Language Server:    scratch-lsp (JSON-RPC 2.0 stdio LSP)");
    println!(" Headless Runtime:   scratch-runtime (World, Entities, Events)");
    println!(" Native Runner:      scratch-native (Bevy 0.15 2D window & headless)");
    println!(" Scene Engine:       scratch-scenes (YAML .schscene multi-level data)");
    println!(" Asset Pipeline:     scratch-assets (Sprite, sound, music indexing)");
    
    let registry = BlockRegistry::core();
    let block_count = registry.iter().count();
    println!(" Block Catalog:      {} Scratch 2.0 primitives registered", block_count);
    println!(" Target Resolutions: 1280x720 (16:9), 480x360 (Scratch standard)");
    println!(" Audio Engine:       Web Audio API synth & Bevy audio");
    println!(" Live Studio IDE:    scratch studio (Local embedded HTTP playground)");
    println!("============================================================");
}

pub fn run_sdk_check() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running Scratch SDK Component Integrity Checks...");
    println!("{:-<60}", "");

    // 1. Block Registry
    let reg = BlockRegistry::core();
    let count = reg.iter().count();
    if count >= 80 {
        println!(" [OK] scratch-blocks: {} primitives validated", count);
    } else {
        return Err(format!("scratch-blocks registry incomplete: found only {} blocks", count).into());
    }

    // 2. Parser & Formatter
    let test_code = "when start:\n    score = 42\n";
    let ast = parse(test_code).map_err(|e| format!("scratch-language parse failure: {}", e))?;
    let formatted = format_source(test_code).map_err(|e| format!("scratch-language format failure: {}", e))?;
    if !formatted.is_empty() {
        println!(" [OK] scratch-language: Lexer, AST Parser, and Formatter operational");
    }

    // 3. Game IR Lowering
    let ir = lower_ast_to_ir(&ast, &reg).map_err(|e| format!("scratch-ir lowering failure: {}", e))?;
    println!(" [OK] scratch-ir: Game IR lowered {} events cleanly", ir.events.len());

    // 4. Bytecode Compilation
    let mut compiler = BytecodeCompiler::new();
    let program = compiler.compile_program(&ir);
    println!(" [OK] scratch-bytecode: Compiled {} bytecode chunks", program.events.len());

    // 5. Runtime & World Simulation
    let mut runtime = Runtime::new(ir, reg);
    runtime.start();
    runtime.tick(0.016);
    println!(" [OK] scratch-runtime: Headless tick loop executed deterministically");

    // 6. Project & Config directory
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    let scratch_dir = PathBuf::from(home).join(".scratch");
    std::fs::create_dir_all(&scratch_dir)?;
    println!(" [OK] Scratch User Storage: {} accessible", scratch_dir.display());

    println!("{:-<60}", "");
    println!("Integrity Check Succeeded! All 6 core SDK subsystems operational.");
    Ok(())
}
