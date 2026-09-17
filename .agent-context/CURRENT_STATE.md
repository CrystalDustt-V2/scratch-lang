# Current State

Last Updated: 2026-09-17 (Session 002)

## Current Phase
Phase 2 & Phase 3: Developer Tooling (Formatter & Linter) and Bytecode VM (Completed)

## Implemented Features
- **Git Repository & Agent Continuity System**: Full `.agent-context` protocol established, updated, and strictly tracked in git.
- **Cargo Workspace Architecture**: Multi-crate decoupled architecture (`scratch-blocks`, `scratch-language`, `scratch-ir`, `scratch-bytecode`, `scratch-vm`, `scratch-runtime`, `scratch-project`, `runtimes/native`, `apps/scratch-cli`).
- **Block Registry (`scratch-blocks`)**: Single source of truth for primitives, parameter typing, docstrings, examples, and Levenshtein suggestion matching.
- **Language Frontend (`scratch-language`)**:
  - Indentation-aware lexer emitting virtual `Indent`, `Dedent`, `Newline` tokens with span tracking.
  - Full AST covering events, statements, and expressions with precedence.
  - Recursive descent parser with block structure.
  - Educational diagnostic rendering with exact line/column carets and suggestions.
  - **Deterministic Formatter (`format_source`)**: Canonical 4-space indentation, operator spacing, and event block separation.
  - **Educational Linter (`lint_source`)**: Educational diagnostic rules (`SL001` unknown object, `SL002` unassigned variable, `SL003` unknown command, `SL004` parameter mismatch, `SL007` per-frame misuse, `SL008` non-positive loop count, `SL010` unknown input action).
- **Game IR (`scratch-ir`)**: Intermediate Representation independent of Bevy/OS and AST lowering with block catalog validation.
- **Bytecode & VM (`scratch-bytecode`, `scratch-vm`)**:
  - Complete opcode instruction set (`OP_PUSH_CONST`, `OP_LOAD_VAR`, `OP_STORE_VAR`, `OP_ADD`, `OP_SUB`, `OP_MUL`, `OP_DIV`, `OP_MOD`, `OP_EQUAL`, `OP_NOT_EQUAL`, `OP_LESS`, `OP_GREATER`, `OP_AND`, `OP_OR`, `OP_NOT`, `OP_NEG`, `OP_JUMP`, `OP_JUMP_IF_FALSE`, `OP_CALL_BLOCK`, `OP_ASSERT`, `OP_HALT`).
  - Bytecode Chunk format with constant pool and jump patching.
  - Bytecode compiler lowering `IrProgram` to `BytecodeProgram`.
  - Stack-based Virtual Machine (`Vm`) with step-fuel limits (100,000 instructions per tick) to prevent infinite loops from hanging the application.
  - `VmRuntime` orchestrating input, collision, update, and start event bytecode chunks against `World`.
- **Headless Runtime (`scratch-runtime`)**: World entity model, dynamic variable store, movement system, AABB collision detection, and event dispatcher.
- **Bevy Desktop Adapter (`runtimes/native`)**: 1280x720 16:9 window, 2D camera, sprite sync, physical keyboard input translation, and headless simulation fallback.
- **CLI (`scratch-cli`)**:
  - `scratch new <project>`: Scaffolds a full project layout.
  - `scratch run [path]`: Compiles and executes project.
  - `scratch check [path]`: Validates AST, Game IR, and Bytecode generation.
  - `scratch lint [path]`: Lints `.sch` files with educational diagnostic carets.
  - `scratch format [path]` (with `--check`): Deterministically formats source files.
  - `scratch test [path]`: Runs automated headless game simulation via Bytecode VM.
- **Example Game**: `examples/hello-game/` containing `project.schproj` and `src/main.sch`.

## Currently Modified Systems
- Completed Phase 2 (Developer Tooling: Formatter & Linter) and Phase 3 (Bytecode VM).

## Current Blockers
- None. All 18 unit/integration tests pass.

## Next Recommended Task
- Phase 4: Scenes & Asset Validation:
  1. `scratch-scenes`: `.scene` file format parser and multi-scene switching (`scene.switch`, `scene.restart`).
  2. `scratch-assets`: static verification of referenced sprite and sound files in `assets/`.
  3. Temporal clock system (`every X seconds:`, `after X seconds:`).

## Test State
- 18 passed, 0 failed, 0 warnings across all 8 workspace crates.
