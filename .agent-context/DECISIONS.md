# Architectural Decisions (ADR)

## ADR-001: 2026-09-16 - Rust Workspace & Bevy Decoupling

### Context
scratch-lang requires a beginner-friendly language frontend while providing high-performance 2D game execution.
Bevy is chosen as the underlying game engine for desktop, but the language and compiler must not depend on Bevy directly.

### Decision
Organize the repository as a Cargo multi-crate workspace:
- `scratch-blocks`, `scratch-language`, `scratch-ir`, `scratch-bytecode`, `scratch-vm`, and `scratch-runtime` have zero dependencies on Bevy.
- `scratch-runtime` provides a headless game state and engine interface.
- `runtimes/native` bridges `scratch-runtime` with Bevy for native desktop graphics, windowing, and audio.

### Rationale
- Allows sub-second testing of all compiler stages, IR, VM, and game logic without compiling Bevy or needing GPU window contexts.
- Keeps WASM and future engine targets straightforward to implement.

### Consequences
Requires an explicit synchronization layer between `scratch-runtime` entities and Bevy ECS components during each frame tick.

---

## ADR-002: 2026-09-16 - Indentation-based Syntax via Lexer Virtual Tokens

### Context
scratch-lang uses Python/Scratch-style indentation for blocks (`when start:`, `if ...:`).

### Decision
The lexer maintains an indentation level stack and emits virtual `INDENT`, `DEDENT`, and `NEWLINE` tokens.

### Rationale
Simplifies recursive descent parsing by turning indentation blocks into standard block delimiters without requiring whitespace-aware grammar quirks in the parser.

---

## ADR-003: 2026-09-16 - Block Registry as Authoritative Primitive Catalog

### Context
Language commands (`move`, `jump`, `sound.play`, `touching`) are shared across the parser, semantic analyzer, IR, autocomplete, linter, runtime, and future visual block editor.

### Decision
Define a unified `scratch-blocks` crate. Every game block is defined with metadata, argument types, return types, educational docstrings, and runtime handler keys.
