export const VSCODE_REFERENCE = {
  overview: `The scratch-lang VS Code extension provides first-class language tooling for writing Scratch projects using clean text syntax (.sch). It connects to the native Rust-based 'scratch lsp' daemon to deliver real-time diagnostics, autocompletion, hover documentation, syntax highlighting, and snippet expansion.`,
  
  extensionStructure: `editors/vscode/
├── .vscodeignore
├── language-configuration.json
├── package.json
├── README.md
├── syntaxes/
│   └── scratch.tmLanguage.json
├── snippets/
│   └── scratch.code-snippets
└── src/
    └── extension.ts`,

  packageJsonConfig: `{
  "name": "scratch-lang",
  "displayName": "scratch-lang",
  "description": "Language support, syntax highlighting, and LSP integration for scratch-lang (.sch)",
  "version": "1.0.0",
  "publisher": "scratch-lang",
  "engines": {
    "vscode": "^1.75.0"
  },
  "categories": ["Programming Languages", "Snippets"],
  "contributes": {
    "languages": [
      {
        "id": "scratch",
        "aliases": ["scratch-lang", "scratch"],
        "extensions": [".sch"],
        "configuration": "./language-configuration.json"
      }
    ],
    "grammars": [
      {
        "language": "scratch",
        "scopeName": "source.scratch",
        "path": "./syntaxes/scratch.tmLanguage.json"
      }
    ],
    "snippets": [
      {
        "language": "scratch",
        "path": "./snippets/scratch.code-snippets"
      }
    ],
    "configuration": {
      "type": "object",
      "title": "scratch-lang Configuration",
      "properties": {
        "scratch.lsp.path": {
          "type": "string",
          "default": "scratch",
          "description": "Path to the scratch CLI executable (which supports 'scratch lsp')"
        }
      }
    }
  }
}`,

  languageConfig: `{
  "comments": {
    "lineComment": "#"
  },
  "brackets": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"]
  ],
  "autoClosingPairs": [
    { "open": "{", "close": "}" },
    { "open": "[", "close": "]" },
    { "open": "(", "close": ")" },
    { "open": "\"", "close": "\"", "notIn": ["string"] },
    { "open": "'", "close": "'", "notIn": ["string", "comment"] }
  ],
  "surroundingPairs": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["\"", "\""],
    ["'", "'"]
  ],
  "indentationRules": {
    "increaseIndentPattern": "^\\\\s*(when\\\\s+.*|if\\\\s+.*|else\\\\s*:|repeat\\\\s+.*|while\\\\s+.*|fn\\\\s+.*):\\\\s*$",
    "decreaseIndentPattern": "^\\\\s*(else):\\\\s*$"
  }
}`,

  lspFeatures: [
    {
      feature: 'Hover Tooltips (textDocument/hover)',
      description: 'Hovering over any function name displays its signature, parameter types, Scratch 3.0 visual block opcode, return value, and examples in rich markdown.'
    },
    {
      feature: 'Auto-Completions (textDocument/completion)',
      description: 'IntelliSense autocompletion for all 147 functions, event headers, keywords, and installed project asset files.'
    },
    {
      feature: 'Real-time Diagnostics (textDocument/publishDiagnostics)',
      description: 'Compiler lint errors, unknown function typo suggestions ("did you mean?"), missing asset warnings, and syntax errors.'
    },
    {
      feature: 'Document Symbols & Outline (textDocument/documentSymbol)',
      description: 'Populates the VS Code Outline view with event handlers, functions, and sprite declarations.'
    },
    {
      feature: 'Document Formatting (textDocument/formatting)',
      description: 'Canonical 4-space indentation and syntax formatting via scratch-language formatter.'
    }
  ]
};
