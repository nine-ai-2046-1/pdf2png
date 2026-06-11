## 1. Track output flag explicitness

- [x] 1.1 Add `#[arg(skip)] pub output_provided: bool` field to `Args` struct
- [x] 1.2 After `Args::parse()`, detect `-o` in `std::env::args()` and set `output_provided`

## 2. Update validation error message

- [x] 2.1 In `validate_args`: when `!args.output_provided && !args.output.exists()`, return error with usage example: `"Output directory not found: ./output\n\nHint: create it first, or specify a different path:\n  pdf2png -i input.pdf -o output-folder-path"`
- [x] 2.2 When `args.output_provided && !args.output.exists()`, keep existing generic error: `"Output directory not found: {path}"`

## 3. Update tests

- [x] 3.1 Update `test_missing_output_dir_returns_error` to verify the generic error for explicit `-o`
- [x] 3.2 Add test: default `./output` missing → error contains usage example
- [x] 3.3 Run `cargo test` — confirm all tests pass
