export const CLI_COMMANDS = [
  {
    name: 'new',
    usage: 'scratch new <project_name> [OPTIONS]',
    description: 'Initializes a fresh, structured scratch-lang project template complete with assets, main.sch, project manifest, and scenes.',
    flags: [
      { flag: '--template <name>', description: 'Starter template: standard, platformer, blank, game (default: standard)' },
      { flag: '--git', description: 'Initialize a git repository in the new project directory' }
    ],
    example: 'scratch new my_awesome_game\nscratch new space_invaders --template game'
  },
  {
    name: 'run',
    usage: 'scratch run [OPTIONS]',
    description: 'Compiles and runs the current project using the native high-performance Rust execution engine and live egui popup window.',
    flags: [
      { flag: '--release', description: 'Run with optimizations enabled' },
      { flag: '--no-preview / --headless', description: 'Run in headless terminal mode without opening GUI window' },
      { flag: '--fps <number>', description: 'Target frames-per-second refresh rate (default: 60)' },
      { flag: '--scene <name>', description: 'Start execution at a specific scene instead of default' }
    ],
    example: 'scratch run\nscratch run --release --fps 120'
  },
  {
    name: 'build',
    usage: 'scratch build [OPTIONS]',
    description: 'Parses, type-checks, lowers to intermediate representation, and compiles the project into an optimized bytecode bundle or native binary.',
    flags: [
      { flag: '--release', description: 'Compile in release mode with full bytecode optimization' },
      { flag: '--target <target>', description: 'Target architecture (native, wasm32, bytecode)' },
      { flag: '--out-dir <dir>', description: 'Output directory for compiled artifacts (default: target/)' }
    ],
    example: 'scratch build --release\nscratch build --target bytecode --out-dir dist/'
  },
  {
    name: 'project studio',
    usage: 'scratch project studio',
    description: 'Launches the interactive native Rust desktop visual IDE, featuring live block inspection, stage preview, entity hierarchy, asset browser, and console.',
    flags: [
      { flag: '--theme <dark|light>', description: 'Select IDE color theme' }
    ],
    example: 'scratch project studio'
  },
  {
    name: 'package',
    usage: 'scratch package <SUBCOMMAND>',
    description: 'Tools for packing, unpacking, and inspecting portable compressed .sb3 and .scratch package bundles.',
    flags: [
      { flag: 'pack', description: 'Bundle source code and assets into a distributable .scratch package' },
      { flag: 'unpack <file>', description: 'Extract assets and scripts from a package into a directory' },
      { flag: 'inspect <file>', description: 'Inspect metadata, assets, scenes, and opcode statistics of a package' }
    ],
    example: 'scratch package pack -o release/game.scratch\nscratch package inspect release/game.scratch'
  },
  {
    name: 'check',
    usage: 'scratch check [OPTIONS]',
    description: 'Runs the scratch-language static analyzer and linter, reporting syntax errors, missing assets, typo detection, and unused variables.',
    flags: [
      { flag: '--strict', description: 'Treat warnings as compiler errors' }
    ],
    example: 'scratch check'
  },
  {
    name: 'lsp',
    usage: 'scratch lsp',
    description: 'Starts the standard Language Server Protocol (LSP) daemon over stdio, enabling IDE diagnostics, autocompletion, hover documentation, and document symbols for VS Code.',
    flags: [
      { flag: '--stdio', description: 'Communicate using standard input/output (default for LSP clients)' }
    ],
    example: 'scratch lsp\n# Used internally by the scratch-lang VS Code extension'
  }
];
