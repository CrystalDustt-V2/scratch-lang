# scratch-lang VS Code Extension

Official Language Support for **scratch-lang** (`.sch`), the beginner-first, code-based 2D game platform.

## Features

- **Syntax Highlighting**: Rich TextMate grammar for `.sch` files (events, control flow, built-in blocks, variables, comments).
- **Language Server Protocol (LSP)**:
  - Real-time syntax and educational linter diagnostics (`SL001` - `SL010`).
  - Intelligent autocompletion for built-in blocks, input actions, events, and scene targets.
  - Rich hover tooltips with parameter documentation, categories, and code examples directly from the authoritative Block Registry.
  - Deterministic document formatting via `scratch format`.
  - Document outline / symbols for `when` event blocks.

## Usage

### 1. Starting the LSP Server
The `scratch` CLI provides an embedded LSP server:
```bash
scratch lsp
```

### 2. VS Code Setup
In your VS Code `settings.json`, configure the LSP client (using generic LSP client extensions like `glsp` or this extension):
```json
{
  "scratch.lsp.path": "scratch"
}
```
