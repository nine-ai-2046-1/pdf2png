## ADDED Requirements

### Requirement: Render PDF page to PNG image in memory
The renderer SHALL accept a PDF page reference and a DPI value, and return a rasterized image in memory (RGBA or RGB). The renderer SHALL use pdfium-render as the underlying engine.

#### Scenario: Render single page at specified DPI
- **WHEN** the renderer is called with a valid PDF page and DPI=300
- **THEN** it returns a valid in-memory raster image at the correct pixel dimensions

#### Scenario: Render at custom DPI
- **WHEN** the renderer is called with DPI=150
- **THEN** the output image dimensions are proportionally halved compared to DPI=300

---

### Requirement: Load PDF from file path
The renderer SHALL open a PDF from a file path and return a handle for page iteration. If the file cannot be opened as a valid PDF, an error SHALL be returned.

#### Scenario: Valid PDF file loaded
- **WHEN** given a path to a valid PDF file
- **THEN** the renderer returns a document handle with the correct page count

#### Scenario: Invalid or corrupt PDF file
- **WHEN** given a path to a non-PDF or corrupt file
- **THEN** the renderer returns an error

---

### Requirement: Report page dimensions
The renderer SHALL be able to return the pixel dimensions `(width, height)` of any given page at the specified DPI, without fully rendering it.

#### Scenario: Query page dimensions
- **WHEN** dimensions are requested for page N at DPI=300
- **THEN** the renderer returns the correct `(width_px, height_px)` tuple
