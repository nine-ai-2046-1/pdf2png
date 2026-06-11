## Why

There is no standalone, dependency-free CLI tool in Rust for converting PDF files to PNG images. Developers and automation scripts need a reliable, scriptable tool that renders PDFs page-by-page (or grouped by page dimensions) to PNG, with machine-readable output for easy integration into pipelines.

## What Changes

- Introduce a new Rust binary crate `pdf2png` as a CLI tool
- Accepts a PDF input file and renders each page to a numbered PNG (`1.png`, `2.png`, ...)
- Supports smart-group mode (`-s`) that groups consecutive pages of identical dimensions into a single stacked PNG
- Supports configurable DPI output (`-d`), defaulting to 300 DPI
- Warns the user when total PDF pages exceed 5 in `-s` mode and requires confirmation (bypassable with `-y`)
- Outputs a machine-readable JSON result to stdout upon completion: `{"success":true,"count":N}`
- Fully standalone binary — no runtime dependencies required by the end user (pdfium bundled at build time)

## Capabilities

### New Capabilities

- `cli-args`: Parse and validate all CLI arguments (`-i`, `-o`, `-d`, `-s`, `-y`)
- `pdf-renderer`: Render individual PDF pages to PNG images using pdfium-render at a specified DPI
- `page-grouper`: In `-s` mode, group consecutive PDF pages by identical dimensions into batches for stacking
- `image-stacker`: Vertically stack multiple same-dimension page images into a single PNG
- `file-writer`: Write numbered PNG files to the output directory
- `confirmation-prompt`: Warn and prompt user when total PDF pages exceed 5 in `-s` mode; respect `-y` flag
- `json-reporter`: Output final `{"success":true,"count":N}` JSON to stdout after all PNGs are written

### Modified Capabilities

## Impact

- New Rust binary crate: `pdf2png`
- Build-time dependency: `pdfium-render` (with `pdfium_download_binaries` feature for bundling)
- Runtime dependencies for end user: **none**
- Dependencies: `clap`, `pdfium-render`, `image`, `serde_json`
- Output: numbered PNG files + stdout JSON
- No existing code modified (greenfield project)
