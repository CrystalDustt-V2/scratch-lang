# Changelog

All notable changes to the scratch-lang codebase will be documented in this file.

## [0.2.0] - 2026-09-17 (Session 002)
### Added
- **Developer Tooling**:
  - `scratch-language::formatter`: Deterministic source code formatter (`scratch format`).
  - `scratch-language::linter`: Game-aware educational linter (`scratch lint`) with rules `SL001` (unknown object), `SL002` (unassigned variable), `SL003` (unknown command), `SL004` (parameter mismatch), `SL006` (unreachable code), `SL007` (per-frame misuse), `SL008` (non-positive loop count), and `SL010` (unknown input action).
- **Bytecode & VM**:
  - `crates/scratch-bytecode`: Opcode instruction set, Chunk with constant pool and jump patching, and Bytecode Compiler from Game IR.
  - `crates/scratch-vm`: Stack-based Virtual Machine with fuel limit protection against infinite loops, and `VmRuntime` for running game events.
  - CLI `scratch check` now verifies AST, Game IR, and Bytecode generation.
  - CLI `scratch test` now executes headless game simulation via Bytecode VM.
- 5 new unit tests across `scratch-language`, `scratch-bytecode`, and `scratch-vm` (Total tests: 18).

## [0.1.0] - 2026-09-16 (Session 001)
### Added
- Git repository initialization with strict `.agent-context/` tracking.
- Rust multi-crate workspace setup (`scratch-blocks`, `scratch-language`, `scratch-ir`, `scratch-runtime`, `scratch-project`, `runtimes/native`, `apps/scratch-cli`).
- Milestone 1 vertical slice implementation.
