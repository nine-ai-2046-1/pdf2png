## Context

This is a housekeeping change applied to an already-built Rust CLI (`pdf2png`). The codebase is complete and tested. Two small structural adjustments are needed: a directory reorganisation and a default value change.

Current state: the Rust crate root sits at `pdf2png/pdf2png/` when it should be at `pdf2png/`. The `-o` default outputs to the current directory, which is inconvenient and risks file clutter.

## Goals / Non-Goals

**Goals:**
- Move the crate root to the project root directory
- Remove the redundant nested subfolder
- Change the default `-o` value to `./output`
- Preserve all existing validation: `-o` path must exist or CLI rejects

**Non-Goals:**
- Auto-creating the output directory (not requested)
- Changing any validation logic
- Changing any rendering, grouping, or prompting behaviour

## Decisions

### D1: Move strategy — direct file move, no git history rewrite

Move all tracked files (`Cargo.toml`, `src/`, `build.rs`, `tests/`, `.gitignore`, `Cargo.lock`) and the `.git/` directory up one level, then remove the empty subfolder.

The `openspec/` and `.opencode/` directories already live at the project root and are unaffected.

**Rationale**: A simple `mv` preserves the full git history. No re-clone or re-init needed.

### D2: Default output path = `./output`

**Before**: `-o` defaults to `"."` (current working directory)
**After**: `-o` defaults to `"./output"`

Validation is unchanged: if the resolved `-o` path does not exist, the CLI prints an error and exits with code 1. The user is expected to create the directory themselves or explicitly pass an existing path.

**Rationale**: Keeps the working directory clean. Provides a well-known default location callers can rely on.

## Risks / Trade-offs

- **[Risk] Users relying on old default output location** → Breaking change. Users who previously ran `pdf2png -i doc.pdf` without `-o` expected files in `.`. After this change, they must either create `./output/` or pass `-o .`. **Mitigation**: This is a new tool with no existing users yet.
- **[Risk] Git history may show a rename detection spike** → After the move, `git log --follow` may see all files as renamed. **Mitigation**: cosmetic only; `git log --follow -M` handles this correctly.
