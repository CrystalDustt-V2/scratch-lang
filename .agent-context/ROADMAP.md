# Roadmap & Milestones

## Phase 1: Language Foundation & Vertical Slice (Milestone 1 - Completed)
- [x] Task 1.1: Rust workspace structure setup.
- [x] Task 1.2: `scratch-blocks` Block Registry core with initial movement and event blocks.
- [x] Task 1.3: `scratch-language` Lexer with indentation handling, AST, and recursive descent parser.
- [x] Task 1.4: `scratch-ir` Game IR lowering.
- [x] Task 1.5: `scratch-runtime` Headless runtime, entities, event dispatch, and input state.
- [x] Task 1.6: `runtimes/native` Bevy integration (1280x720 window, player entity, input binding).
- [x] Task 1.7: `scratch-cli` (`new`, `run`, `check`, `test`).
- [x] Task 1.8: Comprehensive test suite for Phase 1 (13 tests passing).

## Phase 2: Game Primitives & Extended Events
- [x] Task 2.1: Collision events (`when Player touches Enemy:`) and predicates (`if touching(...)`).
- [x] Task 2.2: Loops (`repeat`), conditions (`if`, `else`), logical operators (`and`, `or`, `not`).
- [ ] Task 2.3: Sprites, colors, textures, and asset indexing.
- [ ] Task 2.4: Timers (`every 2 seconds:`, `after 5 seconds:` runtime clock dispatch).

## Phase 3: Bytecode VM & Deterministic Execution
- [ ] Task 3.1: Instruction set design (`OP_LOAD`, `OP_STORE`, `OP_CALL`, `OP_EVENT`, etc.).
- [ ] Task 3.2: Bytecode compiler from Game IR.
- [ ] Task 3.3: VM execution loop with fuel/step limits to protect against infinite loops.

## Phase 4: Project Model & Scenes
- [x] Task 4.1: `project.schproj` YAML parsing and validation.
- [ ] Task 4.2: Scene format (`.scene`) and multi-scene switching.
- [x] Task 4.3: Headless test runner (`scratch test`).

## Phase 5: Developer Tooling
- [ ] Task 5.1: Formatter (`scratch format`).
- [ ] Task 5.2: Linter with educational diagnostics (`SL001`-`SL010`).
- [ ] Task 5.3: `scratch-lsp` Language Server Protocol implementation.

## Phase 6: Scratch Studio & Distribution
- [ ] Task 6.1: Native release packaging (`dist/MyGame.exe`).
- [ ] Task 6.2: Web/WASM target export.
- [ ] Task 6.3: Scratch Studio GUI editor.
