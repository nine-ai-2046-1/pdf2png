## Context

This is a greenfield Rust CLI project. No existing codebase to integrate with. The tool must be fully standalone — end users should not need to install any system libraries. The rendering engine (pdfium) is bundled at build time via a build-time feature flag.

The tool is intended to be called by other programs (shell scripts, CI pipelines), so machine-readable stdout output is a first-class requirement.

## Goals / Non-Goals

**Goals:**
- Produce a single self-contained binary with no runtime dependencies
- Render PDF pages to PNG at configurable DPI (default 300)
- Support two output modes: per-page and smart-group (`-s`)
- Output structured JSON to stdout on completion
- Modular, testable codebase with clear separation of concerns

**Non-Goals:**
- GUI or interactive UI beyond the single confirmation prompt
- Support for password-protected PDFs
- Image format output other than PNG
- Horizontal page stacking (always vertical in `-s` mode)
- PDF generation or manipulation

## Decisions

### D1: Rendering Engine — pdfium-render with bundled binaries

**Decision**: Use `pdfium-render` with the `pdfium_download_binaries` feature flag so that the pdfium shared library is downloaded and linked at build time.

**Rationale**: pdfium (Google Chromium's PDF engine) delivers the highest rendering quality of any available Rust-accessible PDF library. The bundled binary approach ensures end users need zero additional installation.

**Alternatives considered**:
- `mupdf-sys`: Good quality but complex C build setup and harder cross-compilation
- `poppler` bindings: Mature on Linux but unreliable on macOS without Homebrew
- Pure Rust renderers (`pdf-rs`, `lopdf`): No rasterization support — text-only extraction

---

### D2: Module Structure

**Decision**: Separate the codebase into focused modules with no cross-module business logic.

```
src/
├── main.rs            // Entry point: wire modules, run pipeline
├── cli.rs             // Argument parsing (clap), Args struct
├── renderer.rs        // pdfium-render wrapper: page → DynamicImage
├── grouper.rs         // -s mode: group consecutive same-dimension pages
├── stacker.rs         // Vertically stack Vec<DynamicImage> → DynamicImage
├── writer.rs          // Write numbered PNG files to output dir
├── prompt.rs          // Warn/confirm logic; respects -y flag
└── reporter.rs        // Emit {"success":true,"count":N} to stdout
```

**Rationale**: Low coupling, high testability. Each module can be unit-tested in isolation. `main.rs` only orchestrates.

---

### D3: Page Grouping Algorithm (`-s` mode)

**Decision**: Group consecutive pages by exact pixel dimensions `(width_px, height_px)` computed at the target DPI. A new group starts whenever a page's dimensions differ from the previous page.

```
Pages: [A(800×1131), B(800×1131), C(1131×800), D(1131×800), E(800×1131)]
Groups:
  Group 1: [A, B]  → 1.png  (stacked, portrait)
  Group 2: [C, D]  → 2.png  (stacked, landscape)
  Group 3: [E]     → 3.png  (solo, portrait)
```

**Rationale**: Non-consecutive same-size pages should NOT be merged — preserves document order and avoids confusing output.

---

### D4: Output File Naming

**Decision**: Sequential integers starting from 1: `1.png`, `2.png`, `3.png` ...

**Rationale**: Simple, predictable, and caller-friendly (easy to enumerate). No zero-padding required since callers receive the exact count via JSON.

---

### D5: JSON Output to stdout

**Decision**: Print a single-line JSON object to stdout as the very last operation: `{"success":true,"count":N}`. All other informational output (warnings, prompts) goes to stderr.

**Rationale**: Separating machine-readable output (stdout) from human-readable messages (stderr) is a Unix best practice and allows callers to capture JSON without filtering noise.

---

### D6: Default DPI = 300

**Decision**: Default DPI is 300 (print quality). Users may override with `-d <num>`.

**Rationale**: 300 DPI is the professional standard for print-quality rasterization. It is deterministic and avoids the ambiguity of "max possible".

## Risks / Trade-offs

- **[Risk] pdfium binary download at build time** → If the CDN is unavailable in CI, build fails. Mitigation: cache the pdfium binary in CI. Alternative: vendor the binary into the repo.
- **[Risk] Large output file sizes at 300 DPI** → High-res PNGs can be large (multi-MB per page). Mitigation: document this; users can lower DPI via `-d`.
- **[Risk] Memory usage for large PDFs in `-s` mode** → All pages in a group must be held in memory simultaneously for stacking. Mitigation: process groups one at a time, free memory after writing each PNG.
- **[Risk] Non-standard PDF features** → Some PDFs with embedded fonts or exotic color spaces may render incorrectly. Mitigation: pdfium handles the vast majority of real-world PDFs; document as known limitation.
