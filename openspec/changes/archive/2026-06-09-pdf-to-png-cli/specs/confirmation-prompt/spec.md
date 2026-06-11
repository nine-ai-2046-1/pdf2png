## ADDED Requirements

### Requirement: Warn when total PDF pages exceed 5 in smart-group mode
When `-s` mode is active and the total number of pages in the PDF exceeds 5, the tool SHALL print a warning message to stderr and prompt the user to confirm before proceeding.

#### Scenario: Page count exceeds 5, user confirms
- **WHEN** `-s` is active and PDF has 10 pages and user types `y`
- **THEN** the tool proceeds with conversion

#### Scenario: Page count exceeds 5, user declines
- **WHEN** `-s` is active and PDF has 10 pages and user types `n`
- **THEN** the tool exits with a non-zero code and writes no files

#### Scenario: Page count is exactly 5
- **WHEN** `-s` is active and PDF has exactly 5 pages
- **THEN** no warning is shown and the tool proceeds without prompting

#### Scenario: Page count is 4 or fewer
- **WHEN** `-s` is active and PDF has 4 pages
- **THEN** no warning is shown and the tool proceeds without prompting

---

### Requirement: Auto-confirm flag bypasses prompt
When `-y` is provided, the confirmation prompt SHALL be skipped and the tool SHALL proceed as if the user confirmed.

#### Scenario: Auto-confirm with large PDF
- **WHEN** `-s -y` are both active and PDF has 20 pages
- **THEN** no prompt is shown and the tool proceeds immediately

---

### Requirement: Prompt is not shown outside smart-group mode
The page-count warning and confirmation prompt SHALL only appear when `-s` is active.

#### Scenario: Normal mode with large PDF
- **WHEN** `-s` is NOT active and PDF has 20 pages
- **THEN** no warning or prompt is shown
