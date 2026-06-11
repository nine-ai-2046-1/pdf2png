## ADDED Requirements

### Requirement: CLI accepts required input file flag
The CLI SHALL accept a `-i <path>` flag specifying the path to the input PDF file. If `-i` is not provided, the CLI SHALL print an error message to stderr and exit with a non-zero exit code.

#### Scenario: Valid input flag provided
- **WHEN** user runs `pdf2png -i document.pdf`
- **THEN** the tool proceeds with `document.pdf` as the input file

#### Scenario: Input flag omitted
- **WHEN** user runs `pdf2png` without `-i`
- **THEN** the tool prints an error to stderr and exits with a non-zero code

#### Scenario: Input file does not exist
- **WHEN** user runs `pdf2png -i missing.pdf` and the file does not exist
- **THEN** the tool prints an error to stderr and exits with a non-zero code

---

### Requirement: CLI accepts optional output folder flag
The CLI SHALL accept a `-o <path>` flag specifying the output folder. If `-o` is not provided, the output folder SHALL default to the current working directory.

#### Scenario: Output flag provided
- **WHEN** user runs `pdf2png -i doc.pdf -o /tmp/output`
- **THEN** all PNG files are written to `/tmp/output/`

#### Scenario: Output flag omitted
- **WHEN** user runs `pdf2png -i doc.pdf` without `-o`
- **THEN** all PNG files are written to the current working directory

#### Scenario: Output directory does not exist
- **WHEN** user specifies `-o /nonexistent/path` and the directory does not exist
- **THEN** the tool prints an error to stderr and exits with a non-zero code

---

### Requirement: CLI accepts optional DPI flag
The CLI SHALL accept a `-d <num>` flag specifying the render DPI as a positive integer. If `-d` is not provided, DPI SHALL default to 300.

#### Scenario: DPI flag provided
- **WHEN** user runs `pdf2png -i doc.pdf -d 150`
- **THEN** pages are rendered at 150 DPI

#### Scenario: DPI flag omitted
- **WHEN** user runs `pdf2png -i doc.pdf` without `-d`
- **THEN** pages are rendered at 300 DPI

#### Scenario: Invalid DPI value
- **WHEN** user runs `pdf2png -i doc.pdf -d 0` or `-d -100`
- **THEN** the tool prints an error to stderr and exits with a non-zero code

---

### Requirement: CLI accepts smart-group mode flag
The CLI SHALL accept a `-s` flag (boolean, no value) that activates smart-group mode.

#### Scenario: Smart-group flag provided
- **WHEN** user runs `pdf2png -i doc.pdf -s`
- **THEN** the tool activates smart-group mode

#### Scenario: Smart-group flag omitted
- **WHEN** user runs `pdf2png -i doc.pdf` without `-s`
- **THEN** the tool uses default per-page mode (one PNG per page)

---

### Requirement: CLI accepts auto-confirm flag
The CLI SHALL accept a `-y` flag (boolean, no value) that bypasses all confirmation prompts.

#### Scenario: Auto-confirm flag provided
- **WHEN** user runs `pdf2png -i doc.pdf -s -y`
- **THEN** all confirmation prompts are skipped automatically

#### Scenario: Auto-confirm flag omitted
- **WHEN** a confirmation prompt is triggered without `-y`
- **THEN** the tool waits for user input before proceeding
