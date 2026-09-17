# Session Log

## 2026-09-17 - Session 002
- **Goal**: Establish comprehensive reflection targets on `.agent-context/`, implement Phase 2 (Developer Tooling: Formatter & Linter), and Phase 3 (Bytecode VM).
- **Completed**:
  - Expanded `.agent-context/ROADMAP.md` into an authoritative reflection target and implementation checklist with acceptance criteria.
  - Recorded ADR-004 (Stack VM & Fuel Limits) and ADR-005 (Educational Linter & Formatter) in `DECISIONS.md`.
  - Implemented `scratch-language::formatter` (`format_source`) for deterministic 4-space indentation and canonical formatting.
  - Implemented `scratch-language::linter` (`lint_source`) with educational diagnostic rules (`SL001`, `SL002`, `SL003`, `SL004`, `SL006`, `SL007`, `SL008`, `SL010`) and "Did you mean?" suggestions.
  - Created `crates/scratch-bytecode`: OpCodes, Bytecode Chunk with jump patching, and Bytecode Compiler from Game IR.
  - Created `crates/scratch-vm`: Stack VM with 100,000 instruction fuel limit protection against infinite loops, and `VmRuntime` for running game events.
  - Updated `scratch-cli`:
    - `scratch format`: auto-formats `.sch` files (or verifies via `--check`).
    - `scratch lint`: checks for beginner mistakes and prints educational diagnostics.
    - `scratch check`: verifies all 3 stages (AST + Game IR + Bytecode chunks).
    - `scratch test`: simulates frames directly through Bytecode VM.
- **Verification**:
  - All 18 tests passing across the 8 workspace crates in 6.5s.
  - Verified `scratch check`, `scratch lint`, `scratch format --check`, and `scratch test` on `examples/hello-game`.
- **Next**: Phase 4 (Scenes & Asset Validation).

## 2026-09-16 - Session 001
- **Goal**: Establish repository, implement persistent agent context, setup Rust workspace, and deliver Milestone 1 vertical slice.
- **Completed**:
  - Initialized git repository with strict `.agent-context/` tracking.
  - Built core crates (`scratch-blocks`, `scratch-language`, `scratch-ir`, `scratch-runtime`, `scratch-project`, `runtimes/native`, `apps/scratch-cli`).
  - Implemented working starter game in `examples/hello-game`.
