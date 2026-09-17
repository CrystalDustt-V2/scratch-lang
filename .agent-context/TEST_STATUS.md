# Test Status

## Current Test Metrics
- **Total Tests**: 18
- **Passed**: 18
- **Failed**: 0
- **Ignored**: 0

## Test Results by Crate
- `crates/scratch-blocks`:
  - `test_core_registry_contains_primitives` (Passed)
  - `test_did_you_mean_suggestion` (Passed)
- `crates/scratch-bytecode`:
  - `test_compile_milestone1_to_bytecode` (Passed)
- `crates/scratch-language`:
  - `lexer::test_lex_action_down` (Passed)
  - `lexer::test_lex_simple_event` (Passed)
  - `lexer::test_lex_touches_event` (Passed)
  - `tests::test_formatter_roundtrip` (Passed)
  - `tests::test_linter_catches_typo` (Passed)
  - `tests::test_linter_catches_unknown_command` (Passed)
  - `tests::test_parse_if_condition` (Passed)
  - `tests::test_parse_milestone1_program` (Passed)
- `crates/scratch-ir`:
  - `test_lower_milestone1_program` (Passed)
- `crates/scratch-runtime`:
  - `test_runtime_start_and_move` (Passed)
  - `test_collision_event` (Passed)
- `crates/scratch-vm`:
  - `test_vm_executes_start_and_move` (Passed)
  - `test_vm_runtime_tick` (Passed)
  - `test_vm_infinite_loop_protection` (Passed)
- `crates/scratch-project`:
  - `test_default_project_config` (Passed)
  - `test_yaml_roundtrip` (Passed)
- `runtimes/native`:
  - `test_headless_runner_ticks` (Passed)

## CLI Automated Verification
- `scratch check examples/hello-game`: Passed (AST + IR + Bytecode validation)
- `scratch lint examples/hello-game`: Passed (0 lint warnings)
- `scratch format --check examples/hello-game`: Passed
- `scratch test examples/hello-game`: Passed (10 simulation ticks via Bytecode VM)
- `scratch run examples/hello-game`: Passed
