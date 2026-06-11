# pdf2png

> Convert PDF pages to PNG images — a standalone Rust CLI tool.

## Background

Most PDF-to-PNG tools require system libraries (poppler, Ghostscript) or produce
inconsistent results across platforms. `pdf2png` bundles Google's pdfium engine
at build time, producing a **single self-contained binary** with zero runtime
dependencies. Run it anywhere — no `brew install`, no `apt-get`, no DLL hell.

## Features

- **One PNG per page** — default mode, numbered `1.png`, `2.png`, ...
- **Smart-group mode** (`-s`) — consecutive pages with the same dimensions are
  vertically stacked into a single PNG
- **Configurable DPI** — default 300 (print quality), override with `-d`
- **Machine-readable output** — prints `{"success":true,"count":N}` to stdout
  on completion, making it easy to integrate into shell scripts and CI pipelines
- **Fully standalone** — pdfium is embedded in the binary at compile time

## Installation

```bash
# Build from source (requires Rust toolchain)
git clone <repo-url> && cd pdf2png
cargo build --release

# The binary is at target/release/pdf2png
```

## Quick Start

```bash
# Create the output directory first
mkdir -p output

# Convert every page to a separate PNG
pdf2png -i document.pdf -o output

# Stack consecutive same-dimension pages into fewer PNGs
pdf2png -i document.pdf -o output -s -y

# Custom DPI (lower = smaller files)
pdf2png -i document.pdf -o output -d 150
```

## CLI Reference

| Flag | Long | Description | Default |
|------|------|-------------|---------|
| `-i` | `--input` | Path to the input PDF file | *(required)* |
| `-o` | `--output` | Output folder for PNG files | `./output` |
| `-d` | `--dpi` | Render DPI (higher = sharper, larger files) | `300` |
| `-s` | `--smart-group` | Group consecutive same-dimension pages into one PNG | off |
| `-y` | `--yes` | Auto-confirm all prompts (bypass warnings) | off |

## Smart-Group Mode

When `-s` is active, the tool groups **consecutive** pages that share identical
pixel dimensions. Each group is rendered as a single vertically-stacked PNG.

### How orientation is handled

Page dimensions are computed at the target DPI. Two consecutive pages are
grouped only if their pixel width and height are **exactly equal**. A portrait
page followed by a landscape page (or vice versa) starts a new group.

### Example

Given a 5-page PDF:

```
Page 1: portrait  (800×1131 px)
Page 2: portrait  (800×1131 px)
Page 3: landscape (1131×800 px)
Page 4: landscape (1131×800 px)
Page 5: portrait  (800×1131 px)
```

The smart-group output:

```
output/
├── 1.png   ← Pages 1+2 stacked (both portrait, same size)
├── 2.png   ← Pages 3+4 stacked (both landscape, same size)
└── 3.png   ← Page 5 only (portrait, different from previous group)
```

**Key rule:** non-consecutive pages with the same dimensions are **not** merged.
Page 5 does not join group 1 even though it has the same size — document order
is preserved.

### Large PDF warning

In smart-group mode, if the PDF has **more than 5 pages**, the tool warns the
user and asks for confirmation before proceeding. Use `-y` to skip this prompt.

## Project Structure

```
pdf2png/
├── Cargo.toml           # Dependencies and metadata
├── build.rs             # Downloads pdfium at compile time, embeds as bytes
├── src/
│   ├── main.rs          # Entry point — orchestrates the full pipeline
│   ├── cli.rs           # Argument parsing (clap) and validation
│   ├── renderer.rs      # pdfium-render wrapper: page → DynamicImage
│   ├── grouper.rs       # Consecutive same-dimension page grouping
│   ├── stacker.rs       # Vertical image stacking
│   ├── writer.rs        # Numbered PNG file output
│   ├── prompt.rs        # Confirmation prompt (>5 pages)
│   └── reporter.rs      # JSON stdout output
└── tests/
    ├── fixtures/        # Sample PDF files for testing
    └── integration_tests.rs
```

## JSON Output

On successful completion, `pdf2png` prints a single-line JSON object to stdout:

```json
{"success":true,"count":3}
```

- `success` — always `true` on successful completion
- `count` — number of PNG files written

All warnings and prompts go to **stderr**, so stdout is clean for shell parsing.

```bash
# Capture the count in a script
count=$(pdf2png -i doc.pdf -o out -s -y | jq -r '.count')
echo "Generated $count PNGs"
```
