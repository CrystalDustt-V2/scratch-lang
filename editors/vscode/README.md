# scratch-lang VS Code Extension

Official language support, real-time linting, formatting, IntelliSense completions, and Language Server Protocol (LSP) integration for **scratch-lang** (`.sch`).

---

## 🌟 Key Features

### 1. Real-time Linting & Diagnostics
- **Compiler Syntax Errors**: Live red squiggly underlines on syntax mistakes with line/column precision.
- **Educational Typo Suggestions**: Catches misspelled commands and offers *"💡 Suggestion: Did you mean 'move'?"* in the Problems panel.
- **Asset Validation**: Warns if a sound or backdrop asset referenced in code does not exist in the project directory.

### 2. Deterministic Code Formatter
- Formats `.sch` source code to canonical 4-space indentation via `scratch-language`'s AST formatter.
- Supports **Format Document** (`Shift + Alt + F` on Windows/Linux, `Shift + Option + F` on macOS) and **Format on Save**.

### 3. IntelliSense Autocompletion & Snippets
- **Complete 147 Function Catalog**: Autocompletion for all Core Scratch 3.0 blocks and official extensions (Pen, Music, TTS, Translate, Makey Makey).
- **Context-Aware Completions**:
  - Typing inside quotes offers input actions (`"left"`, `"right"`, `"space"`, etc.) or project asset names.
  - Typing `when ` offers ready-to-use event templates (`when start:`, `when update:`, `when click:`, `when touches:`).
- **Tabstop Snippets**: Inserts parameter placeholders (`move(${1:Player}, ${2:10})`) allowing smooth tab navigation between arguments.

### 4. Rich Hover Tooltips
- Hovering over any function name displays its full signature, Scratch shape, Scratch 3.0 visual block opcode, parameter types, description, and runnable code examples.

### 5. Document Symbols & Outline View
- Populates the VS Code **Outline** panel with game event handlers (`when start`, `when update`, `when action.press`, etc.) and functions.

### 6. Interactive Toolbar & Commands
- **Play Button in Editor Title**: Click the $(play) button in the upper-right corner of any `.sch` editor to run the game with live preview.
- **Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`)**:
  - `Scratch: Run Project (GUI Preview)` (`scratch run`)
  - `Scratch: Check & Lint Project` (`scratch check`)
  - `Scratch: Open Visual Project Studio` (`scratch project studio`)
  - `Scratch: Restart Language Server` (`scratch lsp`)

---

## 📦 Extension Settings

| Setting | Default | Description |
| :--- | :--- | :--- |
| `scratch.lsp.path` | `"scratch"` | Path or executable name for the `scratch` CLI binary. |
| `scratch.lsp.enable` | `true` | Enables/disables the background Language Server daemon. |
| `scratch.format.enable` | `true` | Enables/disables document formatting on save or format commands. |
| `scratch.lint.enable` | `true` | Enables/disables real-time diagnostic reporting in the Problems tab. |

---

## 🛠️ Testing & Installing Locally

### Option A: Install Directly from Source Folder
1. Open VS Code.
2. Press `F1` (or `Ctrl+Shift+P`), type **Developer: Install Extension from Location...** and choose the `editors/vscode` directory.

### Option B: Package as `.vsix`
```bash
cd editors/vscode
npx @vscode/vsce package
```
This generates `scratch-lang-0.1.0.vsix`. You can install it with:
```bash
code --install-extension scratch-lang-0.1.0.vsix
```
