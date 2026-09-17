# scratch-lang

**A beginner-first, text-based programming language for building 2D games and interactive stories — powered by the Scratch visual programming model, but written in plain code.**

> Write game logic in readable `.sch` files. No drag-and-drop required. Full language server, bytecode compiler, and native runtime included.

---

## What Is scratch-lang?

scratch-lang is a statically-structured, event-driven scripting language purpose-built for creating 2D games and interactive experiences. It takes the core block-based concepts from [MIT Scratch](https://scratch.mit.edu/) — sprites, stages, events, broadcasts, and sensing — and expresses them as clean, readable code in `.sch` source files.

It is designed for learners making the leap from visual drag-and-drop programming to real text-based code, and for educators who want to teach game logic without the overhead of a general-purpose language.

### Key Principles

- **Readable by anyone** — Event handlers, conditionals, and game logic look like plain English.
- **Scratch-compatible mental model** — Every block from the Scratch palette has a direct text equivalent.
- **Full toolchain** — Lexer → Parser → AST → IR → Bytecode → VM. No shortcuts.
- **LSP-powered editor support** — Real-time diagnostics, autocompletion, hover docs, and formatting in VS Code.
- **One binary, everything included** — The `scratch` CLI handles project creation, linting, formatting, testing, packaging, and building.

---

## Quick Start

```bash
# Install (from source)
cargo install --path apps/scratch-cli

# Create a new platformer project
scratch new my_game --template platformer

# Run an instant preview window
scratch run my_game

# Build for distribution
scratch build my_game --target all
```

---

## Language Overview

Scratch source files use the `.sch` extension. Programs are structured as a list of **event handlers** — blocks of code that run when specific things happen in the game world.

### Hello, Game!

```scheme
when start:
    score = 0
    background.set("night")
    variable.show("score")

when action.down("right"):
    move(Player, 5)

when action.down("left"):
    move(Player, -5)

when action.press("jump"):
    jump(Player, 12)
    sound.play("jump")

when Player touches Coin:
    score += 10
    teleport(Coin, random(50, 430), random(50, 310))
    sound.play("coin")
```

### A Full Platformer

```scheme
when start:
    score = 0
    health = 3
    background.set("forest")

when action.down("right"):
    move(Player, 6)

when action.down("left"):
    move(Player, -6)

when action.press("jump"):
    jump(Player, 14)

when Player touches Coin:
    collect(Coin)
    score += 1
    sound.play("coin")

when Player touches Hazard:
    damage(Player, 1)
    health -= 1
    if health <= 0:
        respawn(Player)
        health = 3

when Player touches Goal:
    sound.play("victory")
    scene.switch("level2")

every 5 seconds:
    score += 1
```

---

## Language Features

### Event Handlers

The core unit of a scratch-lang program. Every block of code is attached to an event trigger.

| Syntax | Description |
| :--- | :--- |
| `when start:` | Runs once when the game launches |
| `when action.down("right"):` | Fires every frame a key/action is held |
| `when action.press("jump"):` | Fires once on key press |
| `when Player touches Coin:` | Fires on AABB collision between two entities |
| `when message("scored"):` | Fires when a broadcast is received |
| `when scene.switched("level2"):` | Fires on scene transitions |
| `when click(Player):` | Fires when player clicks a sprite |
| `when update:` | Runs every frame (60 FPS game loop) |
| `every 3 seconds:` | Runs on a recurring timer |

### Variables & Operators

```scheme
score = 0           # Assignment
score += 10         # Compound add
health -= 1         # Compound subtract
score *= 2          # Compound multiply
alive = true        # Boolean
name = "Player1"    # String

# Arithmetic
result = (score * 2) + math.round(math.sqrt(health))

# Comparison & Logic
if health <= 0 and not alive:
    respawn(Player)

if score > 100 or time > 60:
    scene.switch("win")
```

### Control Flow

```scheme
if health <= 0:
    respawn(Player)
    health = 3

repeat 10:
    move(Player, 5)

# Indefinite loop via frame event
when update:
    bounce_on_edge(Player)
```

### Broadcasts

```scheme
# Send a message to all handlers
broadcast("game-over")

# Send and wait for all handlers to finish
broadcast_and_wait("level-complete")

# Receive a message
when message("game-over"):
    say_for(Player, "You lost!", 2)
    scene.switch("menu")
```

### Dynamic Lists

```scheme
when start:
    list.add("inventory", "iron_sword")
    list.add("inventory", "health_potion")
    list.show("inventory")

# Read and remove
item = list.item("inventory", 1)
list.delete("inventory", "last")

# Query
if list.contains("inventory", "key"):
    scene.switch("locked_room")
```

### Smooth Motion & Tweening

```scheme
when start:
    # Smoothly glide to position over 2 seconds
    glide(Player, 2, 380, 280)

    # Set rotation mode
    set_rotation_style(Player, "left-right")

    # Point towards mouse or another entity
    point_towards(Player, "mouse")
```

### Dialogue & User Input

```scheme
when start:
    ask("What is your name?")

when message("answered"):
    say_for(Player, text.join("Hello, ", get_answer()), 3)
```

### Scenes

```scheme
when start:
    scene.switch("intro")

when message("start-game"):
    scene.switch("level1")

when Player touches Goal:
    scene.switch("level2")
```

---

## Built-in Block Reference

### Motion

| Function | Description |
| :--- | :--- |
| `move(target, dx)` | Move sprite by delta pixels |
| `teleport(target, x, y)` | Set absolute position instantly |
| `glide(target, secs, x, y)` | Smooth tween to position over time |
| `go_to(target, destination)` | Move to `"mouse"` or `"random"` |
| `change_x(target, dx)` | Adjust x-coordinate by delta |
| `change_y(target, dy)` | Adjust y-coordinate by delta |
| `set_x(target, x)` | Set x-coordinate directly |
| `set_y(target, y)` | Set y-coordinate directly |
| `turn_right(target, deg)` | Rotate clockwise |
| `turn_left(target, deg)` | Rotate counter-clockwise |
| `point_in_direction(target, deg)` | Set absolute facing angle |
| `point_towards(target, other)` | Face towards another entity or mouse |
| `bounce_on_edge(target)` | Bounce off stage boundaries |
| `set_rotation_style(target, style)` | `"left-right"`, `"all-around"`, `"none"` |
| `x_position(target)` | Returns current X |
| `y_position(target)` | Returns current Y |
| `get_direction(target)` | Returns facing angle in degrees |

### Looks

| Function | Description |
| :--- | :--- |
| `say(target, text)` | Show speech bubble |
| `say_for(target, text, secs)` | Show speech bubble for duration |
| `think(target, text)` | Show thought bubble |
| `think_for(target, text, secs)` | Show thought bubble for duration |
| `show(target)` | Make sprite visible |
| `hide(target)` | Hide sprite |
| `switch_costume(target, name)` | Set active costume frame |
| `next_costume(target)` | Advance to next costume |
| `set_size(target, pct)` | Set scale (100 = normal) |
| `change_size(target, delta)` | Adjust scale by delta |
| `set_effect(target, effect, val)` | Apply shader (`color`, `ghost`, `brightness`) |
| `change_effect(target, effect, delta)` | Adjust shader effect |
| `clear_effects(target)` | Remove all visual effects |
| `go_to_front(target)` | Bring to top render layer |
| `go_back_layers(target, n)` | Push back N render layers |

### Audio

| Function | Description |
| :--- | :--- |
| `sound.play(name)` | Play sound asynchronously |
| `sound.play_until_done(name)` | Play sound and wait for completion |
| `sound.stop_all()` | Stop all sounds |
| `sound.set_volume(pct)` | Set volume 0–100 |
| `sound.change_volume(delta)` | Adjust volume |
| `music.play_note(note, beats)` | Play MIDI note |
| `music.play_drum(drum, beats)` | Play drum hit |
| `music.set_instrument(id)` | Set MIDI instrument |
| `music.set_tempo(bpm)` | Set tempo |
| `music.rest(beats)` | Rest for beat interval |

### Sensing

| Function | Description |
| :--- | :--- |
| `touching(target, other)` | AABB collision check → `Boolean` |
| `touching_color(target, color)` | Color collision check → `Boolean` |
| `key_pressed(key)` | Key held check → `Boolean` |
| `mouse_down()` | Mouse button check → `Boolean` |
| `mouse_x()` | Mouse X coordinate |
| `mouse_y()` | Mouse Y coordinate |
| `distance_to(target, other)` | Euclidean distance |
| `get_timer()` | Elapsed seconds |
| `reset_timer()` | Reset timer to zero |
| `ask(question)` | Show text input prompt |
| `get_answer()` | Return last user input |

### Math & Strings

| Function | Description |
| :--- | :--- |
| `random(min, max)` | Random number in range |
| `math.abs(n)` | Absolute value |
| `math.round(n)` | Round to integer |
| `math.floor(n)` | Floor |
| `math.ceil(n)` | Ceiling |
| `math.sqrt(n)` | Square root |
| `math.sin(n)`, `math.cos(n)`, `math.tan(n)` | Trigonometry |
| `math.log(n)` | Natural log |
| `math.pow(base, exp)` | Exponentiation |
| `math.min(a, b)`, `math.max(a, b)` | Min/Max |
| `math.clamp(n, lo, hi)` | Clamp value in range |
| `math.mod(a, b)` | Modulo remainder |
| `text.join(a, b)` | Concatenate strings |
| `text.length(s)` | String length |
| `text.letter_at(s, i)` | Character at index |
| `text.contains(s, sub)` | Substring check |
| `text.to_upper(s)`, `text.to_lower(s)` | Case conversion |
| `text.trim(s)` | Strip whitespace |

### Control

| Function | Description |
| :--- | :--- |
| `wait(secs)` | Non-blocking delay |
| `stop_all()` | Halt all scripts |
| `broadcast(msg)` | Send global event |
| `broadcast_and_wait(msg)` | Send and await completion |
| `clone(target)` | Instantiate dynamic clone |
| `delete_clone(target)` | Remove clone from world |

---

## Project Structure

A scratch-lang project looks like this:

```
my_game/
├── project.yaml        # Project metadata (name, author, version, scenes)
├── src/
│   └── main.sch        # Main game script
└── assets/
    ├── sprites/        # Sprite images (.png, .svg)
    └── sounds/         # Sound files (.wav, .mp3)
```

`project.yaml` example:

```yaml
name: my_game
title: My Awesome Game
author: YourName
version: 0.1.0
scenes:
  - menu
  - level1
  - level2
assets:
  - name: Player
    kind: sprite
    path: assets/sprites/player.png
  - name: Coin
    kind: sprite
    path: assets/sprites/coin.png
  - name: coin
    kind: sound
    path: assets/sounds/coin.wav
```

---

## CLI Reference

The `scratch` CLI is the unified developer toolchain for scratch-lang projects.

```
Command-line interface for Scratch

Usage: scratch <COMMAND>

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

### `scratch new`

```bash
scratch new my_game                          # Blank starter template
scratch new my_platformer --template platformer
scratch new my_rpg --template dialogue --author "You"
```

Available templates: `starter`, `platformer`, `coins`, `dialogue`, `motion`, `lists`

### `scratch project`

```bash
scratch project check       # Validate syntax, assets, scenes
scratch project lint        # Find anti-patterns and unknown blocks
scratch project format      # Auto-format source code
scratch project test        # Run validation suite
scratch project info        # Print project metadata
scratch project add-scene Level2
scratch project add-asset --name Cat --kind sprite --path assets/cat.png
scratch project studio      # Launch native Rust Desktop IDE
```

### `scratch run`

```bash
scratch run                   # Launch live preview in native Rust popup window
scratch run ./my_game         # Preview a specific project
scratch run --headless        # Run in terminal without a window (CI / headless simulation)
```

`scratch run` launches a **live native Rust desktop popup window** powered by `eframe` / `egui`. It compiles and runs the game in real time with 60 FPS rendering, dynamic sprite positioning, speech/thought bubbles, variable HUD monitors, list displays, and keyboard (`WASD` / Arrow keys / `Space`) & mouse input. No browser or web runtime required.

### `scratch project studio`

```bash
scratch project studio        # Open native Scratch Studio desktop IDE
scratch studio                # Top-level shortcut alias
```

Launches an interactive, **native Rust desktop IDE**:
- **Scratch Block Palette**: Authentic colorful category chips (Motion, Looks, Sound, Events, Control, Sensing, Operators, Variables, Lists) with click-to-insert code snippets and documentation.
- **Code Editor**: Monospace `.sch` editor with line numbers, live syntax diagnostics, format on save (`Ctrl+S`), and auto-indentation.
- **Live Stage Preview**: Embedded real-time 60 FPS game stage with play/pause/reset controls.
- **Inspectors**: Real-time variables & lists monitor, entity properties inspector, and diagnostic error output.

### `scratch build`

```bash
scratch build                           # Native executable (default)
scratch build --target native           # Standalone desktop binary
scratch build --target web              # HTML5 / WebAssembly player
scratch build --target studio           # Scratch Studio bundle
scratch build --target bytecode         # Compiled .scb bytecode
scratch build --target all              # All distribution formats
scratch build --release --out-dir dist  # Optimized release build
```

### `scratch package`

```bash
scratch package pack ./my_game -o dist/my_game.scratch
scratch package unpack dist/my_game.scratch -o extracted/
scratch package info dist/my_game.scratch
scratch package export-web -o dist/web
scratch package export-studio -o dist/studio
```

### `scratch config`

```bash
scratch config list
scratch config get author
scratch config set author "YourName"
scratch config set default_template platformer
scratch config reset
```

### `scratch sdk`

```bash
scratch sdk info       # SDK paths and version
scratch sdk version    # Version string
scratch sdk path       # SDK root directory
scratch sdk check      # Diagnostic health check
```

---

## Editor Support (VS Code)

scratch-lang ships a full Language Server Protocol (LSP) server providing:

- ✅ **Syntax highlighting** — Rich TextMate grammar for `.sch` files
- ✅ **Real-time diagnostics** — Unknown blocks, missing assets, typos with "did you mean?" suggestions
- ✅ **Autocompletion** — Built-in blocks, input actions, event types, entity names
- ✅ **Hover documentation** — Parameter types, descriptions, and code examples inline
- ✅ **Document formatting** — Auto-format on save via `scratch project format`
- ✅ **Document outline** — Symbol navigation for all `when` event blocks

**Starting the LSP server:**

```bash
scratch lsp
```

Configure your editor with the LSP server path:

```json
{
  "scratch.lsp.path": "scratch"
}
```

---

## Architecture

scratch-lang is built as a Rust workspace with clearly separated crates:

```
crates/
├── scratch-language    # Lexer, Parser, AST, Formatter, Linter
├── scratch-ir          # AST → Intermediate Representation lowering
├── scratch-bytecode    # IR → .scb bytecode compiler
├── scratch-vm          # Bytecode virtual machine & executor
├── scratch-runtime     # Entity state, motion, sensing, audio, lists
├── scratch-scenes      # Scene graph and stage management
├── scratch-assets      # Asset manifest indexing and lookup
├── scratch-project     # project.yaml config schema and I/O
├── scratch-blocks      # Block registry with signatures and docs
└── scratch-lsp         # Full LSP server implementation

runtimes/
└── native              # Headless tick-loop runner for native targets

apps/
└── scratch-cli         # Unified developer CLI (scratch)

editors/
└── vscode              # VS Code extension with LSP client
```

**Compilation Pipeline:**

```
.sch source
    │
    ▼
[scratch-language]  Lexer → Tokens → Parser → AST
    │
    ▼
[scratch-ir]        AST → Lowered IR (control flow, expression trees)
    │
    ▼
[scratch-bytecode]  IR → Opcode bytecode (.scb)
    │
    ▼
[scratch-vm]        Bytecode interpreter + event dispatch
    │
    ▼
[scratch-runtime]   Entity state, physics, sensing, audio, lists
```

---

## Installing from Source

**Prerequisites:** Rust 1.75+ (via [rustup](https://rustup.rs))

```bash
# Clone the repository
git clone https://github.com/your-org/scratch-lang
cd scratch-lang

# Build and install the CLI
cargo install --path apps/scratch-cli

# Verify installation
scratch --version
```

**Run all tests:**

```bash
cargo test --workspace
```

Expected output: **42 tests across 11 crates, 0 failures.**

---

## Examples

Two example projects are included in [`examples/`](examples/):

| Example | Description |
| :--- | :--- |
| [`hello-game`](examples/hello-game/) | Minimal starter — movement, background, score |
| [`platformer`](examples/platformer/) | Full platformer with health, hazards, coins, scenes, timers |

Run an example:

```bash
scratch run examples/platformer
```

---

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

at your option.
