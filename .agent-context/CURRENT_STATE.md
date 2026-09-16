# Current State

Last Updated: 2026-09-16 (Session 001)

## Current Phase
Phase 1: Language Foundation & Vertical Slice (Milestone 1 Completed)

## Implemented Features
- **Git Repository & Agent Continuity System**: Full `.agent-context` protocol established and strictly tracked in git.
- **Cargo Workspace Architecture**: Multi-crate decoupled architecture (`scratch-blocks`, `scratch-language`, `scratch-ir`, `scratch-runtime`, `scratch-project`, `runtimes/native`, `apps/scratch-cli`).
- **Block Registry (`scratch-blocks`)**: Single source of truth for primitives (`move`, `jump`, `stop`, `teleport`, `touching`, `damage`, `heal`, `respawn`, `collect`, `sound.play`, `camera.follow`, `background.set`). Contains parameter specs, docstrings, examples, and Levenshtein suggestion helper for beginner error diagnostics.
- **Language Frontend (`scratch-language`)**:
  - Indentation-aware lexer emitting virtual `Indent`, `Dedent`, `Newline` tokens with span tracking.
  - Full AST covering events (`when start`, `when update`, `when action.down/press/up`, `when touches`, `every/after seconds`), statements (`Assign`, `Call`, `If`, `Repeat`, `Return`, `Assert`), and expressions with precedence.
  - Recursive descent parser with clean block structuring.
  - Educational diagnostic structure with line/column pointers and "Did you mean?" suggestions.
- **Game IR (`scratch-ir`)**:
  - Engine-independent intermediate representation (`IrProgram`, `IrEventHandler`, `IrInstruction`, `IrExpr`).
  - AST to IR lowering with block validation and similarity checks.
- **Headless Runtime (`scratch-runtime`)**:
  - `World` entity model (`Entity`, `Transform2D`, `EntityId`, tags, visibility, color, size).
  - Dynamic variable storage (`score`, `health`).
  - Event dispatcher (`OnStart`, `OnUpdate`, `OnActionDown/Press/Up`, `OnTouches`).
  - Movement system (`move`, `jump`, `stop`, `teleport`).
  - Collision checking (AABB overlap).
  - Headless tick simulation for automated game tests.
- **Bevy Desktop Adapter (`runtimes/native`)**:
  - 1280x720 16:9 native desktop window with white background.
  - Bevy 0.15 2D camera & sprite synchronization.
  - Physical keyboard mapping (Arrow keys & WASD to logical actions).
  - Headless runner mode fallback for fast testing.
- **CLI (`scratch-cli`)**:
  - `scratch new <project>`: Scaffolds a full project layout.
  - `scratch run [path]`: Compiles and executes project.
  - `scratch check [path]`: Verifies `.sch` syntax and lowering.
  - `scratch test [path]`: Runs automated headless game simulation.
- **Example Game**: `examples/hello-game/` containing `project.schproj` and `src/main.sch`.

## Currently Modified Systems
- Completed Milestone 1 vertical slice.

## Current Blockers
- None. All 13 unit/integration tests pass.

## Next Recommended Task
- Phase 2: Game Primitives & Extended Features:
  1. Add bytecode VM (`scratch-bytecode`, `scratch-vm`) compiler stage.
  2. Implement `scratch format` formatter tool in `scratch-cli`.
  3. Implement `scratch lint` with educational codes `SL001`-`SL010`.
  4. Expand scene model (`.scene` files) in `scratch-scenes`.

## Test State
- 13 passed, 0 failed, 0 warnings.
