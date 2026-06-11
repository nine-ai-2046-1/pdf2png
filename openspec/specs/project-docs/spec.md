## ADDED Requirements

### Requirement: Bilingual project documentation
The project SHALL provide user-facing documentation in two languages: English (`README.md`) and Cantonese/Traditional Chinese (`README-HK.md`).

#### Scenario: English README exists
- **WHEN** a user opens `README.md`
- **THEN** it contains: background, features, installation, quick start, CLI reference, smart-group logic, project structure, and JSON output format

#### Scenario: Cantonese README exists
- **WHEN** a user opens `README-HK.md`
- **THEN** it contains the same sections as the English README, localised in Cantonese with Traditional Chinese characters, with technical terms in English

### Requirement: CLI reference completeness
The documentation SHALL include a complete reference table for all CLI flags: `-i`, `-o`, `-d`, `-s`, `-y`, with descriptions, defaults, and required/optional status.

#### Scenario: Flag reference table
- **WHEN** user reads the CLI Reference section
- **THEN** all 5 flags are listed with their short form, long form, description, default value, and whether they are required

### Requirement: Smart-group logic explanation
The documentation SHALL explain how smart-group mode (`-s`) groups consecutive pages by identical dimensions and how orientation (portrait vs landscape) affects grouping.

#### Scenario: Orientation example
- **WHEN** user reads the Smart-Group Mode section
- **THEN** they see a concrete example showing how a PDF with mixed portrait/landscape pages is split into groups
