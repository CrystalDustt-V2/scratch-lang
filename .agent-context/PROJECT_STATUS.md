# Project Status

## Legend
- Planned
- In Progress
- Implemented
- Partially Implemented
- Blocked
- Deferred

---

## 1. Compiler & Language (`scratch-language`)
- Lexer (indentation tokens, keywords, symbols, literals): Implemented
- AST definition: Implemented
- Parser (events, statements, expressions, assignments, calls, if, repeat): Implemented
- Name resolution & Semantic validation: Partially Implemented (Block Registry validation in IR lowering)
- Diagnostics & Educational error formatting: Implemented
- Formatter (`scratch format`): Planned
- Linter (`SL001`-`SL010`): Planned

## 2. Block Registry (`scratch-blocks`)
- Block schema (name, category, params, return, docs, runtime mapping): Implemented
- Core game blocks (`move`, `jump`, `stop`, `teleport`, `damage`, `heal`, `respawn`, `collect`, `sound.play`, `camera.follow`, `background.set`): Implemented
- Event blocks (`start`, `update`, `action.down`, `action.press`, `action.up`, `touches`): Implemented
- Fuzzy "Did you mean?" suggestions: Implemented

## 3. Game IR (`scratch-ir`)
- Engine-independent Game IR definitions: Implemented
- AST to IR Lowering: Implemented
- IR serialization/display for debugging: Implemented

## 4. Bytecode & VM (`scratch-bytecode`, `scratch-vm`)
- Bytecode instruction set & compiler: Planned
- VM stack machine & execution loop: Planned

## 5. Runtime Architecture (`scratch-runtime`)
- Headless GameState & World: Implemented
- Entity & Component abstraction: Implemented
- Event dispatcher: Implemented
- Movement system: Implemented
- Input mapping: Implemented
- Collision checking (AABB): Implemented

## 6. Low-Level Engine Adapter (`runtimes/native`)
- Bevy 2D integration (1280x720 16:9 window, camera, sprite render): Implemented
- Bevy input to logical action adapter: Implemented
- Game loop tick & runtime synchronization: Implemented
- Fast headless runner fallback: Implemented

## 7. Developer Tooling & Apps
- `scratch-cli` (`new`, `run`, `check`, `test`): Implemented
- `scratch-lsp`: Planned
- `scratch-studio`: Planned

## 8. Export Targets
- Native desktop executable: Implemented
- Web/WASM export: Planned
