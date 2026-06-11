## MODIFIED Requirements

### Requirement: CLI accepts optional output folder flag
The CLI SHALL accept a `-o <path>` flag specifying the output folder. If `-o` is not provided, the output folder SHALL default to `./output` relative to the current working directory.

#### Scenario: Output flag provided
- **WHEN** user runs `pdf2png -i doc.pdf -o /tmp/output`
- **THEN** all PNG files are written to `/tmp/output/`

#### Scenario: Output flag omitted
- **WHEN** user runs `pdf2png -i doc.pdf` without `-o`
- **THEN** all PNG files are written to `./output` relative to the current working directory

#### Scenario: Output directory does not exist
- **WHEN** user specifies `-o /nonexistent/path` or the default `./output` directory does not exist
- **THEN** the tool prints an error to stderr and exits with a non-zero code
