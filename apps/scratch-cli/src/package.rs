use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct ScratchPackage {
    pub format: String,
    pub name: String,
    pub version: String,
    pub created_at: String,
    pub entry: String,
    pub files: BTreeMap<String, FilePayload>,
}

#[derive(Serialize, Deserialize)]
pub struct FilePayload {
    pub is_binary: bool,
    pub content: String,
}

pub fn pack_project(proj_path: &Path, out_file: Option<PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let entry_file = if proj_path.is_file() {
        proj_path.to_path_buf()
    } else {
        proj_path.join("src/main.sch")
    };

    let proj_dir = if entry_file.is_file() {
        entry_file
            .parent()
            .and_then(|p| if p.ends_with("src") { p.parent() } else { Some(p) })
            .unwrap_or(Path::new("."))
    } else {
        proj_path
    };

    let proj_name = proj_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let target_out = out_file.unwrap_or_else(|| proj_dir.join(format!("{}.scratch", proj_name)));

    println!("Packaging project in '{}' into '{}'...", proj_dir.display(), target_out.display());

    let mut files = BTreeMap::new();
    collect_files_recursive(proj_dir, proj_dir, &mut files)?;

    let package = ScratchPackage {
        format: "scratch-package-v1".to_string(),
        name: proj_name,
        version: "0.1.0".to_string(),
        created_at: format!("{:?}", std::time::SystemTime::now()),
        entry: "src/main.sch".to_string(),
        files,
    };

    let json = serde_json::to_string_pretty(&package)?;
    std::fs::write(&target_out, json)?;

    println!("============================================================");
    println!(" Package created successfully!");
    println!(" File: {} ({} files bundled)", target_out.display(), package.files.len());
    println!("============================================================");

    Ok(target_out)
}

fn collect_files_recursive(
    root: &Path,
    current: &Path,
    files: &mut BTreeMap<String, FilePayload>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !current.exists() {
        return Ok(());
    }

    for entry in std::fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let rel_path = path.strip_prefix(root)?.to_string_lossy().replace('\\', "/");

        // Ignore build outputs and git dirs
        if rel_path.starts_with("target")
            || rel_path.starts_with("dist")
            || rel_path.starts_with(".git")
            || rel_path.ends_with(".scratch")
        {
            continue;
        }

        if path.is_dir() {
            collect_files_recursive(root, &path, files)?;
        } else if path.is_file() {
            let bytes = std::fs::read(&path)?;
            let is_binary = match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
                "png" | "jpg" | "jpeg" | "wav" | "mp3" | "ogg" | "schbc" => true,
                _ => bytes.iter().any(|&b| b == 0),
            };

            let content = if is_binary {
                // simple hex string for binary files
                bytes.iter().map(|b| format!("{:02x}", b)).collect()
            } else {
                String::from_utf8_lossy(&bytes).to_string()
            };

            files.insert(rel_path, FilePayload { is_binary, content });
        }
    }

    Ok(())
}

pub fn unpack_package(package_file: &Path, out_dir: Option<PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if !package_file.exists() {
        return Err(format!("Package file '{}' does not exist", package_file.display()).into());
    }

    let content = std::fs::read_to_string(package_file)?;
    let package: ScratchPackage = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse .scratch package: {}", e))?;

    let target_dir = out_dir.unwrap_or_else(|| {
        let stem = package_file.file_stem().unwrap_or_default();
        PathBuf::from(stem)
    });

    println!("Unpacking '{}' into '{}'...", package.name, target_dir.display());
    std::fs::create_dir_all(&target_dir)?;

    for (rel_path, payload) in package.files {
        let file_path = target_dir.join(&rel_path);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if payload.is_binary {
            let mut bytes = Vec::new();
            let hex = payload.content.as_bytes();
            for i in (0..hex.len()).step_by(2) {
                if i + 1 < hex.len() {
                    let byte_str = std::str::from_utf8(&hex[i..i+2]).unwrap_or("00");
                    let b = u8::from_str_radix(byte_str, 16).unwrap_or(0);
                    bytes.push(b);
                }
            }
            std::fs::write(&file_path, bytes)?;
        } else {
            std::fs::write(&file_path, payload.content)?;
        }
    }

    println!("============================================================");
    println!(" Package unpacked successfully!");
    println!(" Destination: {}", target_dir.display());
    println!(" Run with:");
    println!("   scratch run {}", target_dir.display());
    println!("============================================================");

    Ok(target_dir)
}

pub fn inspect_package(package_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !package_file.exists() {
        return Err(format!("Package file '{}' not found", package_file.display()).into());
    }

    let file_size = std::fs::metadata(package_file)?.len();
    let content = std::fs::read_to_string(package_file)?;
    let package: ScratchPackage = serde_json::from_str(&content)
        .map_err(|e| format!("Invalid package format: {}", e))?;

    println!("============================================================");
    println!(" Scratch Package Details: {}", package_file.display());
    println!("============================================================");
    println!(" Name:        {}", package.name);
    println!(" Version:     {}", package.version);
    println!(" Format:      {}", package.format);
    println!(" Entry Point: {}", package.entry);
    println!(" Package Size:{:.2} KB", file_size as f64 / 1024.0);
    println!(" Files Count: {}", package.files.len());
    println!();
    println!(" Bundled Files:");
    for (name, payload) in &package.files {
        let kind = if payload.is_binary { "binary" } else { "text" };
        let size = payload.content.len();
        println!("   - {:<36} ({}, ~{} bytes)", name, kind, size);
    }
    println!("============================================================");

    Ok(())
}
