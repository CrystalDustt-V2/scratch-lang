# Master Architecture

## System Diagram
```text
                         SCRATCH-STUDIO / SCRATCH-CLI
                                      │
                                project.schproj
                                      │
                             ┌────────┴────────┐
                             │                 │
                           *.sch            *.scene
                             │                 │
                             ▼                 ▼
                      scratch-language    scratch-scenes
                             │                 │
                            AST                │
                             │                 │
                      scratch-blocks           │
                     (Block Registry)          │
                             │                 │
                             ▼                 │
                         scratch-ir            │
                          (Game IR)            │
                             │                 │
                             ▼                 │
                      scratch-bytecode         │
                             │                 │
                             ▼                 │
                         scratch-vm            │
                             │                 │
                             ▼                 ▼
                      scratch-runtime ◄────────┘
                       (Game State)
                             │
                             ▼
                    Platform Engine Adapter
                        (Bevy / WASM)
```

## Architectural Boundaries

1. **`scratch-blocks` (Block Registry)**
   - The authoritative single source of truth for all game-level primitives.
   - Defines signatures, categories, metadata, documentation, and maps to runtime systems.
   - Decoupled from AST parser and Bevy internals.

2. **`scratch-language`**
   - Pure Rust text analysis: Lexer -> Virtual Indent Tokens -> AST -> Recursive Descent Parser.
   - **Formatter (`format_source`)**: Canonical indentation and spacing pretty-printer.
   - **Linter (`lint_source`)**: Educational diagnostic rules (`SL001`–`SL010`) with exact caret highlighting and "Did you mean?" suggestions.
   - No Bevy dependency.

3. **`scratch-ir` (Game IR)**
   - Platform-independent representation of game logic and event handlers.
   - Lowers high-level scratch-lang constructs into clean execution steps.

4. **`scratch-bytecode` & `scratch-vm`**
   - High-performance, deterministic execution VM for game logic.
   - `scratch-bytecode`: OpCodes, constant pools, jump labels/patching, and Bytecode Compiler.
   - `scratch-vm`: Stack-based virtual machine with per-tick step/fuel limits to prevent infinite loops from locking the game or window thread.
   - `VmRuntime`: Orchestrates input, collision, update, and start event chunks against the runtime `World`.

5. **`scratch-runtime`**
   - Headless core game state, entities, logical input mapping, event dispatcher, movement, collision, camera, sound abstractions.
   - Can run headlessly for fast automated unit tests (`scratch test`).

6. **`runtimes/native` (Bevy Adapter)**
   - Drives the windowing (1280x720 16:9), 2D rendering, audio output, and physical keyboard/mouse input translation.
   - Interacts strictly through `scratch-runtime` interfaces.
