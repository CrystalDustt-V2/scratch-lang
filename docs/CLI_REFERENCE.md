# Scratch CLI Reference Manual

The `scratch` CLI provides a unified developer toolchain for initializing, testing, linting, running, packaging, and building Scratch language projects. Its interface and ergonomics are modeled after game engine tooling (like the `geode` CLI).

```text
Command-line interface for Scratch

Usage: scratch.exe <COMMAND>

Commands:
  new      Initialize a new Scratch project
  config   Options for configuring Scratch CLI
  sdk      Options for installing & managing the Scratch SDK
  project  Tools for working with the current project
  package  Options for working with .scratch packages
  run      Run preview of the project in a popup window
  build    Builds the project at the current directory
  help     Print this message or the help of the given subcommand(s)
```

---

## 1. `scratch new`

Initializes a new Scratch project with starter assets, scenes, and entrypoint code.

```bash
# Basic project
scratch new my_game

# Choose a starter template
scratch new my_platformer --template platformer

# Specify author and custom directory
scratch new my_rpg --template dialogue --author "PlayerOne" --path ./projects/my_rpg
```

### Available Templates:
- `starter`: Standard clean template with movement, bounce, and say bubbles.
- `platformer`: Player sprite with gravity, ground collisions, and jump mechanics.
- `coins`: Collectible item tracking with score variables and broadcast events.
- `dialogue`: Narrative interaction showing dialogues, questions, and responses.
- `motion`: Glide interpolation, point towards target, and rotation style demonstrations.
- `lists`: Dynamic list manipulation, CRUD queries, and sorting.

---

## 2. `scratch config`

Manage global CLI preferences stored at `~/.scratch/config.json`.

```bash
# List all configured settings
scratch config list

# Get a specific setting
scratch config get author
scratch config get preview_mode

# Set a configuration value
scratch config set author "YourName"
scratch config set studio_port 4500
scratch config set preview_mode popup

# Reset configuration to default values
scratch config reset
```

### Configurable Keys:
- `author`: Default author name when scaffolding new projects.
- `default_template`: Default starter template (`starter`, `platformer`, etc.).
- `preview_mode`: Preview window mode (`popup`, `browser`, `headless`).
- `editor`: Preferred code editor executable command (`code`, `cursor`, etc.).
- `studio_port`: Default web studio listening port (default: `3456`).
- `log_level`: Default log verbosity (`info`, `debug`, `trace`).

---

## 3. `scratch sdk`

Tools for inspecting, verifying, and diagnosing the Scratch SDK installation.

```bash
# View SDK installation information and paths
scratch sdk info

# Print SDK version
scratch sdk version

# Print SDK root path
scratch sdk path

# Run diagnostic health check across all SDK subsystems
scratch sdk check
```

The diagnostic health check validates:
1. Lexer & Grammar Parser (`scratch-language`)
2. Intermediate Representation Lowering (`scratch-ir`)
3. Bytecode Compiler & Opcode Assembler (`scratch-bytecode`)
4. Virtual Machine Execution Engine (`scratch-vm`)
5. Asset & Sound Indexer (`scratch-assets`)
6. Scene & Stage Manager (`scratch-scenes`)

---

## 4. `scratch project`

Subcommands for inspecting, developing, and validating the current project.

```bash
# Check syntax, assets, and scenes
scratch project check

# Lint project for anti-patterns and missing references
scratch project lint

# Auto-format Scratch code according to standard grammar
scratch project format --write

# Run project unit tests and validation suites
scratch project test

# Display project metadata, scene hierarchy, and assets
scratch project info

# Add a new scene to the project
scratch project add-scene Level2

# Register a sprite or sound asset into the project manifest
scratch project add-asset --name cat --kind sprite --path assets/cat.png

# Launch the in-browser visual Scratch Studio IDE
scratch project studio --port 3456
```

---

## 5. `scratch package`

Inspect, create, extract, and export portable `.scratch` packages.

```bash
# Pack a project directory into a single .scratch bundle
scratch package pack ./my_game -o ./dist/my_game.scratch

# Inspect an existing package's contents and manifest
scratch package info ./dist/my_game.scratch

# Unpack a .scratch bundle into a target directory
scratch package unpack ./dist/my_game.scratch -o ./extracted_game

# Export project as a self-contained Web HTML5 player
scratch package export-web -o ./dist/web

# Export project bundle for Scratch Studio distribution
scratch package export-studio -o ./dist/studio
```

---

## 6. `scratch run`

Run an instant preview of the project.

```bash
# Open preview in a standalone app popup window
scratch run

# Preview another project directory
scratch run ./games/platformer

# Run headless in the terminal without opening a window
scratch run --headless --ticks 100

# Specify custom port
scratch run --port 8080
```

On Windows, `scratch run` automatically detects installed browsers (Edge, Chrome) and opens a dedicated borderless application window (`--app=http://127.0.0.1:<port> --window-size=1020,720`) simulating a standalone native game window.

---

## 7. `scratch build`

Compile the Scratch project into native executables, Web packages, or bytecode binaries.

```bash
# Default build (native target)
scratch build

# Build specific target
scratch build --target native    # Standalone executable
scratch build --target web       # Web HTML5 / WASM distribution
scratch build --target studio    # Studio bundle
scratch build --target bytecode  # Compiled .scb bytecode bundle
scratch build --target all       # Build all distribution targets

# Optimized release build to custom output directory
scratch build --release --out-dir ./dist
```
