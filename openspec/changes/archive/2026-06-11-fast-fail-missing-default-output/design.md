## Context

A small improvement to the CLI validation layer. The current `validate_args` function in `cli.rs` rejects any output path that doesn't exist, with a generic message. The user wants a more helpful message when the *default* output path (`./output`) is missing — one that shows a usage example so the user knows exactly what to do.

## Goals / Non-Goals

**Goals:**
- Show a usage example when `./output` (default) doesn't exist
- Keep the existing generic error for explicit `-o` with a bad path
- Track whether `-o` was explicitly provided by the user

**Non-Goals:**
- Auto-creating the output directory
- Changing validation for other flags

## Decisions

### D1: Track `-o` explicitness via a `skip` field

Clap's `default_value` doesn't distinguish "user typed `-o .`" from "user omitted `-o`". We add a `#[arg(skip)]` field `output_provided: bool` that is set to `true` only when `-o` is explicitly passed.

```rust
#[arg(short = 'o', long = "output", default_value = "./output")]
pub output: PathBuf,

/// Whether the user explicitly provided -o
#[arg(skip)]
pub output_provided: bool,
```

In `parse_args()`, after `Args::parse()`, we detect `-o` presence by checking `std::env::args()` for the `-o` flag. Alternatively, we use a clap `group` or `overrides_with`, but the simplest approach is the `skip` field set in a post-parse step.

### D2: Error messages

| Scenario | Error message |
|---|---|
| Default `./output` missing | `Output directory not found: ./output\n\nHint: create it first, or specify a different path:\n  pdf2png -i input.pdf -o output-folder-path` |
| Explicit `-o /bad/path` missing | `Output directory not found: /bad/path` (unchanged) |

## Risks / Trade-offs

- **[Risk] Detecting `-o` presence** → `std::env::args()` is fragile if clap changes. **Mitigation**: simple, well-understood pattern; alternative is clap's `ArgAction::Set` detection, but skip field is simpler.
