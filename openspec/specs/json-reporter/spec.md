## ADDED Requirements

### Requirement: Output JSON result to stdout on success
Upon successful completion, the tool SHALL print a single-line JSON object to stdout as the final output: `{"success":true,"count":N}` where N is the number of PNG files written.

#### Scenario: Successful conversion
- **WHEN** conversion completes and 3 PNG files were written
- **THEN** stdout contains exactly `{"success":true,"count":3}`

#### Scenario: Count reflects actual files written
- **WHEN** a PDF with 5 pages is converted in normal mode
- **THEN** stdout contains `{"success":true,"count":5}`

#### Scenario: Count reflects group count in smart-group mode
- **WHEN** a PDF with 5 pages forms 3 dimension groups
- **THEN** stdout contains `{"success":true,"count":3}`

---

### Requirement: JSON is the only content written to stdout
All warnings, prompts, and progress messages SHALL be written to stderr. Stdout SHALL contain only the final JSON object.

#### Scenario: Warning goes to stderr
- **WHEN** a 10-page PDF triggers the large-PDF warning in `-s` mode
- **THEN** the warning text appears on stderr, not stdout

#### Scenario: stdout is clean JSON
- **WHEN** the tool runs successfully
- **THEN** stdout contains only the JSON line and nothing else

---

### Requirement: No JSON output on failure
If the tool exits with an error (invalid args, file not found, user declined), it SHALL NOT print the JSON result to stdout.

#### Scenario: User declines confirmation
- **WHEN** user responds `n` to the large-PDF prompt
- **THEN** no JSON is printed to stdout

#### Scenario: Invalid input file
- **WHEN** the input PDF cannot be opened
- **THEN** no JSON is printed to stdout
