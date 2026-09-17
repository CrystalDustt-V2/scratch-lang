use scratch_assets::AssetIndex;
use scratch_blocks::BlockRegistry;
use scratch_bytecode::BytecodeCompiler;
use scratch_ir::lower_ast_to_ir;
use scratch_language::{format_source, parse, Linter};
use scratch_project::ProjectConfig;
use scratch_scenes::{SceneData, SceneManager, SceneObject};
use scratch_vm::VmRuntime;
use std::path::{Path, PathBuf};

pub fn resolve_entry_file(path: &Path) -> Result<(PathBuf, ProjectConfig), Box<dyn std::error::Error>> {
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

pub fn check_project(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
    println!("  Status: All checks passed! Ready to run.");
    Ok(())
}

pub fn lint_project(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, _config) = resolve_entry_file(path)?;
    let source = std::fs::read_to_string(&entry_path)?;

    let proj_dir = if entry_path.is_file() {
        entry_path
            .parent()
            .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
            .unwrap_or(Path::new("."))
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

pub fn format_project(path: &Path, check_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, _config) = resolve_entry_file(path)?;
    let source = std::fs::read_to_string(&entry_path)?;

    let formatted = format_source(&source).map_err(|e| format!("Format error: {}", e))?;

    if check_only {
        if source == formatted {
            println!("File {} is correctly formatted.", entry_path.display());
            Ok(())
        } else {
            Err(format!("File {} is not formatted. Run 'scratch project format' to format.", entry_path.display()).into())
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

pub fn test_project(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, _config) = resolve_entry_file(path)?;
    println!("Testing {} via Bytecode VM simulation...", entry_path.display());

    let source = std::fs::read_to_string(&entry_path)?;
    let ast = parse(&source).map_err(|e| format!("Parse failure in test: {}", e))?;
    let registry = BlockRegistry::core();
    let ir = lower_ast_to_ir(&ast, &registry).map_err(|e| format!("IR lowering failure: {}", e))?;

    let mut compiler = BytecodeCompiler::new();
    let program = compiler.compile_program(&ir);

    let proj_dir = if entry_path.is_file() {
        entry_path
            .parent()
            .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
            .unwrap_or(Path::new("."))
    } else {
        path
    };

    let mut scene_mgr = SceneManager::new();
    let scenes_dir = proj_dir.join("scenes");
    if scenes_dir.exists() {
        scene_mgr = scene_mgr.with_directory(&scenes_dir);
    }

    let mut vm_runtime = VmRuntime::new(program, registry).with_scene_manager(scene_mgr);

    vm_runtime.start().map_err(|e| format!("VM start error: {}", e))?;
    println!("  [Pass] OnStart event initialization");

    for frame in 1..=60 {
        vm_runtime.tick(0.016).map_err(|e| format!("VM tick error at frame {}: {}", frame, e))?;
    }
    println!("  [Pass] Simulation loop (60 frames deterministic step)");

    vm_runtime.world.set_action_down("right", true);
    vm_runtime.tick(0.016)?;
    vm_runtime.world.set_action_down("right", false);
    println!("  [Pass] Action input injection test");

    println!("All game tests passed successfully!");
    Ok(())
}

pub fn project_info(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, config) = resolve_entry_file(path)?;
    let proj_dir = if entry_path.is_file() {
        entry_path
            .parent()
            .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
            .unwrap_or(Path::new("."))
    } else {
        path
    };

    let source = std::fs::read_to_string(&entry_path).unwrap_or_default();
    let lines_of_code = source.lines().count();

    let mut scene_files = Vec::new();
    let scenes_dir = proj_dir.join("scenes");
    if scenes_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&scenes_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("scene") {
                    scene_files.push(entry.file_name().to_string_lossy().to_string());
                }
            }
        }
    }

    let mut asset_index = AssetIndex::new();
    let assets_dir = proj_dir.join("assets");
    if assets_dir.exists() {
        asset_index.scan_directory(&assets_dir);
    }

    let sprites_count = asset_index.iter().filter(|a| a.category == scratch_assets::AssetCategory::Sprite).count();
    let sounds_count = asset_index.iter().filter(|a| a.category == scratch_assets::AssetCategory::Sound).count();
    let music_count = asset_index.iter().filter(|a| a.category == scratch_assets::AssetCategory::Music).count();
    let fonts_count = asset_index.iter().filter(|a| a.category == scratch_assets::AssetCategory::Font).count();

    println!("============================================================");
    println!(" Project Information: {}", config.name);
    println!("============================================================");
    println!(" Directory:   {}", proj_dir.display());
    println!(" Entry File:  {}", entry_path.display());
    println!(" Resolution:  {}x{}", config.resolution.width, config.resolution.height);
    println!(" Code Lines:  {} lines in entry", lines_of_code);
    println!(" Scenes ({}):   {}", scene_files.len(), if scene_files.is_empty() { "(none)".to_string() } else { scene_files.join(", ") });
    println!(" Assets:      {} sprites, {} sounds, {} music tracks, {} fonts",
        sprites_count,
        sounds_count,
        music_count,
        fonts_count
    );
    println!("============================================================");

    Ok(())
}

pub fn add_scene(proj_path: &Path, scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (entry_path, _config) = resolve_entry_file(proj_path)?;
    let proj_dir = if entry_path.is_file() {
        entry_path
            .parent()
            .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
            .unwrap_or(Path::new("."))
    } else {
        proj_path
    };

    let scenes_dir = proj_dir.join("scenes");
    std::fs::create_dir_all(&scenes_dir)?;

    let clean_name = if scene_name.ends_with(".scene") {
        scene_name.to_string()
    } else {
        format!("{}.scene", scene_name)
    };

    let scene_path = scenes_dir.join(&clean_name);
    if scene_path.exists() {
        return Err(format!("Scene file '{}' already exists", scene_path.display()).into());
    }

    let scene_stem = clean_name.trim_end_matches(".scene");
    let mut scene_data = SceneData::default();
    scene_data.name = scene_stem.to_string();
    scene_data.background = "night".to_string();
    scene_data.objects = vec![
        SceneObject {
            name: "Player".to_string(),
            sprite: Some("player".to_string()),
            x: 100.0,
            y: 200.0,
            size: [40.0, 40.0],
            color: [0.2, 0.6, 1.0, 1.0],
            tags: vec!["player".to_string()],
        },
        SceneObject {
            name: "Goal".to_string(),
            sprite: Some("flag".to_string()),
            x: 700.0,
            y: 200.0,
            size: [40.0, 40.0],
            color: [0.2, 1.0, 0.4, 1.0],
            tags: vec!["goal".to_string()],
        },
    ];

    scene_data.save_to_file(&scene_path)?;
    println!("Created scene '{}' at {}", scene_stem, scene_path.display());
    Ok(())
}

pub fn add_asset(proj_path: &Path, asset_source: &Path, asset_type: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    if !asset_source.exists() {
        return Err(format!("Asset file '{}' not found", asset_source.display()).into());
    }

    let (entry_path, _config) = resolve_entry_file(proj_path)?;
    let proj_dir = if entry_path.is_file() {
        entry_path
            .parent()
            .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
            .unwrap_or(Path::new("."))
    } else {
        proj_path
    };

    let subfolder = match asset_type {
        Some("sound") | Some("sounds") => "sounds",
        Some("music") => "music",
        Some("font") | Some("fonts") => "fonts",
        _ => {
            // guess by extension
            match asset_source.extension().and_then(|s| s.to_str()).unwrap_or("") {
                "wav" | "mp3" | "ogg" => "sounds",
                "ttf" | "otf" => "fonts",
                _ => "sprites",
            }
        }
    };

    let target_dir = proj_dir.join("assets").join(subfolder);
    std::fs::create_dir_all(&target_dir)?;

    let file_name = asset_source.file_name().ok_or("Invalid asset filename")?;
    let target_file = target_dir.join(file_name);

    std::fs::copy(asset_source, &target_file)?;
    println!("Copied asset to {}", target_file.display());
    Ok(())
}
