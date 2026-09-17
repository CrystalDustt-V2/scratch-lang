# Roadmap & Reflection Targets

This document serves as the authoritative implementation roadmap and **reflection target** for the scratch-lang platform. Each phase includes its goal, deliverables, acceptance criteria, and reflection checklists.

---

## Phase 1: Language Foundation & Vertical Slice (Completed)
- [x] **Task 1.1**: Rust multi-crate workspace (`Cargo.toml`).
- [x] **Task 1.2**: `scratch-blocks` Block Registry with core movement, gameplay, conditions, and fuzzy matching.
- [x] **Task 1.3**: `scratch-language` Indentation-aware lexer, AST, recursive descent parser, and diagnostic formatter.
- [x] **Task 1.4**: `scratch-ir` Engine-independent Game IR definitions and AST lowering.
- [x] **Task 1.5**: `scratch-runtime` Headless runtime, entity store, event dispatcher, movement, and collision checking.
- [x] **Task 1.6**: `runtimes/native` Bevy 0.15 integration (1280x720 16:9 window, camera, sprite sync, input translation) + headless fallback.
- [x] **Task 1.7**: `scratch-cli` (`new`, `run`, `check`, `test`).
- [x] **Task 1.8**: Phase 1 verification tests (13 tests passing).

**Reflection Check**:
- Can a beginner write a 5-line game and move a player with arrow keys? **Yes.**
- Can the compiler run tests in <3 seconds headlessly without a GPU? **Yes.**

---

## Phase 2: Developer Tooling & Quality of Life (Completed)
### Goal
Empower beginners with immediate educational feedback and automated code cleanliness before runtime.

- [x] **Task 2.1: Deterministic Formatter (`scratch format`)**
  - Canonical 4-space indentation.
  - Normalized spacing around operators (`=`, `+=`, `+`, `-`, `*`, `==`, etc.).
  - Proper empty line separators between events and top-level definitions.
  - CLI integration: `scratch format [path]` (with `--check` flag).
- [x] **Task 2.2: Game-Aware Linter (`scratch lint`)**
  - Educational diagnostic rule codes:
    - `SL001`: Undefined object (e.g. `move(Plyer, 5)` -> "Did you mean 'Player'?")
    - `SL002`: Undefined variable read before assignment.
    - `SL003`: Unknown game command/block.
    - `SL004`: Parameter count / type mismatch.
    - `SL006`: Unreachable code after `return`.
    - `SL007`: Potential per-frame misuse in `when update:`.
    - `SL008`: Obvious non-positive loop count detection.
    - `SL010`: Unknown input action (not in action catalog).
  - CLI integration: `scratch lint [path]`.

**Reflection Check**:
- Are error and warning messages educational, with exact caret positions and constructive "Did you mean?" suggestions? **Yes.**
- Does formatting preserve all code semantics while producing deterministic, clean syntax? **Yes.**

---

## Phase 3: Bytecode VM & Deterministic Execution (Completed)
### Goal
Transition from AST/IR direct interpretation to an engine-independent, high-performance, stack-based bytecode VM.

- [x] **Task 3.1: Instruction Set (`crates/scratch-bytecode`)**
  - Stack opcodes: `OP_PUSH_CONST`, `OP_LOAD_VAR`, `OP_STORE_VAR`, `OP_ADD`, `OP_SUB`, `OP_MUL`, `OP_DIV`, `OP_MOD`.
  - Comparison & logic: `OP_EQUAL`, `OP_NOT_EQUAL`, `OP_LESS`, `OP_LESS_EQUAL`, `OP_GREATER`, `OP_GREATER_EQUAL`, `OP_AND`, `OP_OR`, `OP_NOT`, `OP_NEG`.
  - Control flow: `OP_JUMP`, `OP_JUMP_IF_FALSE`.
  - Calls: `OP_CALL_BLOCK` (invoking runtime systems), `OP_RETURN`, `OP_HALT`.
- [x] **Task 3.2: Bytecode Compiler**
  - Lowers Game IR (`scratch-ir`) into bytecode chunks per event and function with jump patching.
- [x] **Task 3.3: Stack VM (`crates/scratch-vm`)**
  - Deterministic evaluation loop.
  - Fuel / step limit guard (100,000 instructions per tick) to prevent infinite loops from freezing games.
  - Integration with `scratch-runtime` World & Block Registry via `VmRuntime`.
  - CLI test command runs simulation frames through Bytecode VM.

**Reflection Check**:
- Does bytecode execution match IR semantics 100%? **Yes.**
- Does an infinite loop `repeat 1000000:` safely trigger a fuel error instead of hanging the desktop app? **Yes.**

---

## Phase 4: Project Model, Assets & Scenes
### Goal
Support multi-level games and rich multimedia asset pipelines.

- [ ] **Task 4.1: Scene Data Format (`scratch-scenes`)**
  - Dedicated `.scene` files separating level layout from game code.
  - Built-in commands: `scene.switch("Level2")`, `scene.restart()`.
- [ ] **Task 4.2: Asset Validation & Indexing (`scratch-assets`)**
  - Index `assets/sprites`, `assets/sounds`, `assets/music`, `assets/fonts`.
  - Static checking of asset identifiers during `check` and `lint`.
- [ ] **Task 4.3: Temporal Runtime Clock**
  - Dispatching `every X seconds:` and `after X seconds:` events in the game loop.

---

## Phase 5: Language Server Protocol (`scratch-lsp`)
### Goal
Provide IDE intelligence to VS Code, Zed, and future Scratch Studio.

- [ ] Completion for blocks, objects, actions, and asset names.
- [ ] Hover tooltips showing block docstrings and example usage from `scratch-blocks`.
- [ ] Live diagnostics powered by `scratch-language` parser and linter.
- [ ] Document formatting.

---

## Phase 6: Distribution & Scratch Studio
### Goal
Export lightweight standalone game deliverables and provide a visual creative IDE.

- [ ] `scratch build`: Produces stripped standalone `dist/MyGame.exe`.
- [ ] `scratch export web`: Compiles to WebAssembly + HTML5 canvas.
- [ ] `scratch-studio`: GUI editor featuring split-view text & visual blocks, scene editor, and live preview.
