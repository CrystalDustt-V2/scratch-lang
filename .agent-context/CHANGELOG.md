# Changelog

All notable changes to the scratch-lang codebase will be documented in this file.

## [0.1.0] - 2026-09-16 (Session 001)
### Added
- Git repository initialization with strict `.agent-context/` tracking and verification.
- Rust multi-crate workspace setup:
  - `crates/scratch-blocks`: Block Registry defining primitives, parameters, return types, docstrings, examples, and fuzzy suggestion matching.
  - `crates/scratch-language`: Indentation-aware lexer, AST, recursive descent parser, and educational diagnostics.
  - `crates/scratch-ir`: Engine-independent Game IR definitions and AST lowering with semantic validation.
  - `crates/scratch-runtime`: Headless game state world, entity model, variables, event dispatcher, movement system, and collision system.
  - `crates/scratch-project`: `project.schproj` YAML configuration management.
  - `runtimes/native`: 1280x720 16:9 Bevy 0.15 native adapter with 2D camera, sprite sync, input translation, and headless simulation fallback.
  - `apps/scratch-cli`: `scratch` CLI supporting `new`, `run`, `check`, and `test`.
  - `examples/hello-game`: Complete working starter game.
- 13 unit and integration tests covering lexer, parser, IR, runtime, project config, and native runner.
