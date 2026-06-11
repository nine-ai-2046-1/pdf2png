## MODIFIED Requirements

### Requirement: CLI accepts optional output folder flag
The CLI SHALL accept a `-o <path>` flag specifying the output folder. If `-o` is not provided, the output folder SHALL default to `./output` relative to the current working directory. When the default output directory does not exist, the CLI SHALL print a helpful error message including a usage example.

#### Scenario: Output flag provided with valid directory
- **WHEN** user runs `pdf2png -i doc.pdf -o /tmp/output`
- **THEN** all PNG files are written to `/tmp/output/`

#### Scenario: Output flag omitted and ./output exists
- **WHEN** user runs `pdf2png -i doc.pdf` without `-o` and `./output` exists
- **THEN** all PNG files are written to `./output`

#### Scenario: Output flag omitted and ./output does not exist
- **WHEN** user runs `pdf2png -i doc.pdf` without `-o` and `./output` does not exist
- **THEN** the tool prints an error to stderr with a usage example and exits with a non-zero code

#### Scenario: Output flag provided with non-existent directory
- **WHEN** user runs `pdf2png -i doc.pdf -o /nonexistent/path`
- **THEN** the tool prints an error to stderr and exits with a non-zero code
