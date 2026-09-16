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
   - Pure Rust text analysis: Lexer -> Indentation/Token Stream -> AST -> Semantic Analyzer -> Diagnostics.
   - No Bevy dependency.
   - Educational diagnostic formatting (Did you mean "Player"?).

3. **`scratch-ir` (Game IR)**
   - Platform-independent representation of game logic and event handlers.
   - Lowers high-level scratch-lang constructs into clean execution steps.

4. **`scratch-bytecode` & `scratch-vm`**
   - High-performance, deterministic execution VM for game logic.
   - Interprets bytecode against the runtime state.

5. **`scratch-runtime`**
   - Headless core game state, entities, logical input mapping, event dispatcher, movement, collision, camera, sound abstractions.
   - Can run headlessly for fast automated unit tests (`scratch test`).

6. **`runtimes/native` (Bevy Adapter)**
   - Drives the windowing (1280x720 16:9), 2D rendering, audio output, and physical keyboard/mouse input translation.
   - Interacts strictly through `scratch-runtime` interfaces.
