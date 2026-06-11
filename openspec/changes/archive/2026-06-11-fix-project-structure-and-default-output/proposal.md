## Why

The Rust project was accidentally created inside a nested `pdf2png/pdf2png/` subdirectory, placing `Cargo.toml`, `src/`, `build.rs` etc. one level deeper than intended. This adds unnecessary nesting and breaks the standard convention of keeping the crate root at the project root. Additionally, the default `-o` output path (`.`) risks polluting the current working directory with PNG files — changing it to `./output` provides a cleaner, dedicated output location.

## What Changes

- Move all Rust project files (`Cargo.toml`, `src/`, `build.rs`, `tests/`, `.git/`, `.gitignore`, `Cargo.lock`) from `pdf2png/pdf2png/` up to `pdf2png/`
- Remove the now-empty `pdf2png/pdf2png/` subdirectory
- **BREAKING**: Change the default `-o` output folder from `.` (current directory) to `./output`
- Update the `cli-args` spec requirement for `-o` default value

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `cli-args`: The `-o` flag default value changes from the current working directory to `./output`. The validation rule (reject if path does not exist) remains unchanged.

## Impact

- File system: project root restructuring (move + remove subfolder)
- `src/cli.rs`: default_value change from `"."` to `"./output"`
- Tests in `src/cli.rs`: update default value test expectation
- Integration tests: may need path updates if they rely on `"."` as default output
- No runtime behavior change beyond the new default path
