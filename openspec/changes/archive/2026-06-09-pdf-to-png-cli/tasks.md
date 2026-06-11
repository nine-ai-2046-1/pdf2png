## 1. Project Bootstrap

- [x] 1.1 Run `cargo new pdf2png --bin` to initialise the binary crate
- [x] 1.2 Add dependencies to `Cargo.toml`: `clap`, `pdfium-render` (with `pdfium_download_binaries` feature), `image`, `serde_json`
- [x] 1.3 Create module files: `cli.rs`, `renderer.rs`, `grouper.rs`, `stacker.rs`, `writer.rs`, `prompt.rs`, `reporter.rs`
- [x] 1.4 Declare all modules in `main.rs` and verify `cargo build` compiles without errors

## 2. CLI Argument Parsing (`cli.rs`)

- [x] 2.1 Define `Args` struct with fields: `input: PathBuf`, `output: PathBuf`, `dpi: u32`, `smart_group: bool`, `yes: bool`
- [x] 2.2 Implement `parse_args() -> Args` using `clap` with flags: `-i`, `-o`, `-d`, `-s`, `-y`
- [x] 2.3 Set defaults: `output` = current working directory, `dpi` = 300
- [x] 2.4 Validate: `-i` file must exist; `-o` directory must exist; `-d` must be positive integer
- [x] 2.5 Write unit tests for: missing `-i`, missing output dir, invalid DPI (0, negative, non-integer)

## 3. PDF Renderer (`renderer.rs`)

- [x] 3.1 Implement `open_pdf(path: &Path) -> Result<PdfDocument>` wrapping pdfium-render document load
- [x] 3.2 Implement `page_dimensions(doc: &PdfDocument, page_index: usize, dpi: u32) -> (u32, u32)` returning pixel width/height
- [x] 3.3 Implement `render_page(doc: &PdfDocument, page_index: usize, dpi: u32) -> Result<DynamicImage>` returning rasterised image
- [x] 3.4 Write unit tests using a sample PDF: verify page count, dimensions at 300 DPI, successful render

## 4. Page Grouper (`grouper.rs`)

- [x] 4.1 Define `PageGroup` struct: `start_index: usize`, `page_count: usize`, `dimensions: (u32, u32)`
- [x] 4.2 Implement `group_pages(dimensions: &[(u32, u32)]) -> Vec<PageGroup>` grouping consecutive same-dimension pages
- [x] 4.3 Write unit tests: all same, all different, mixed, non-consecutive same-size not merged

## 5. Image Stacker (`stacker.rs`)

- [x] 5.1 Implement `stack_images(images: Vec<DynamicImage>) -> Result<DynamicImage>` stacking vertically
- [x] 5.2 Return error if images have mismatched dimensions
- [x] 5.3 Handle single-image input as passthrough (no stacking needed)
- [x] 5.4 Write unit tests: stack 2 same-size images (verify output height = sum), single image passthrough, mismatch error

## 6. File Writer (`writer.rs`)

- [x] 6.1 Implement `write_pngs(images: Vec<DynamicImage>, output_dir: &Path) -> Result<usize>` writing `1.png`, `2.png`, etc.
- [x] 6.2 Verify output directory exists before writing; return error if missing
- [x] 6.3 Return the count of successfully written files
- [x] 6.4 Write unit tests: sequential naming, count returned, error on missing directory

## 7. Confirmation Prompt (`prompt.rs`)

- [x] 7.1 Implement `check_page_limit(page_count: usize, yes: bool, smart_group: bool) -> Result<()>` encapsulating warning + confirm logic
- [x] 7.2 Print warning to stderr when `smart_group && page_count > 5`
- [x] 7.3 If `yes` is true, skip prompt and return `Ok(())`
- [x] 7.4 If `yes` is false, read stdin for `y`/`n`; return `Err` on `n` or unrecognised input
- [x] 7.5 Write unit tests: no prompt when page_count ≤ 5, skip prompt with `yes=true`, returns err on `n`

## 8. JSON Reporter (`reporter.rs`)

- [x] 8.1 Implement `report_success(count: usize)` printing `{"success":true,"count":N}` to stdout
- [x] 8.2 Ensure all other messages in codebase use `eprintln!` (stderr), never `println!`
- [x] 8.3 Write unit test: verify stdout output matches exact JSON format

## 9. Main Orchestration (`main.rs`)

- [x] 9.1 Wire pipeline: parse args → open PDF → check page limit → collect dimensions → group (if `-s`) → render → stack (if `-s`) → write → report
- [x] 9.2 Propagate errors: any `Err` prints message to stderr and exits with code 1
- [x] 9.3 Ensure no JSON is printed to stdout on error paths

## 10. Integration Tests

- [x] 10.1 Create `tests/` directory with a small sample PDF fixture (≤3 pages)
- [x] 10.2 Integration test: normal mode → correct number of PNG files created, named `1.png`...`N.png`
- [x] 10.3 Integration test: `-s` mode with same-dimension pages → single PNG output
- [x] 10.4 Integration test: `-s` mode with mixed dimensions → multiple grouped PNGs
- [x] 10.5 Integration test: stdout JSON is `{"success":true,"count":N}` with correct count
- [x] 10.6 Integration test: `-s` with 6-page PDF + `-y` → proceeds without prompt
- [x] 10.7 Run `cargo test` and confirm all tests pass

## 11. Final Verification

- [x] 11.1 Run `cargo clippy` and resolve all warnings
- [x] 11.2 Run `cargo fmt` to enforce formatting
- [x] 11.3 Verify binary is self-contained: `ldd`/`otool -L` shows no unexpected runtime dependencies
- [x] 11.4 Manual smoke test: convert a real-world PDF and verify PNG output quality at 300 DPI
