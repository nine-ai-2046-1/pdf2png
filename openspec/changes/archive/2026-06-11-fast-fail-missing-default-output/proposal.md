## Why

When the user runs `pdf2png -i doc.pdf` without `-o`, the default `./output` path is used. If `./output` doesn't exist, the current error message is generic (`"Output directory not found: ./output"`) and doesn't tell the user what to do. A fast-fail with a clear usage example gives the user immediate, actionable guidance.

## What Changes

- Add a usage example to the error message when the default output directory `./output` does not exist
- Distinguish between "user didn't provide `-o`" and "user provided `-o` but path doesn't exist" — the former gets a helpful hint, the latter gets a straightforward error

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `cli-args`: The output directory validation now provides a usage example when the default `./output` is missing. Explicit `-o` with a missing path retains the existing error message.

## Impact

- `src/cli.rs`: add `output_provided` tracking field, update `validate_args` error message, update tests
- No other files affected
