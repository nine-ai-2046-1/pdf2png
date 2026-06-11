## ADDED Requirements

### Requirement: Write PNG files with sequential integer names
The writer SHALL write each output image to the specified directory as `<N>.png` where N starts at 1 and increments by 1 for each file written, in order.

#### Scenario: Write first file
- **WHEN** the first PNG is written
- **THEN** it is saved as `1.png` in the output directory

#### Scenario: Write multiple files
- **WHEN** N PNG images are written sequentially
- **THEN** files are named `1.png`, `2.png`, ..., `N.png`

---

### Requirement: Output directory must exist before writing
The writer SHALL verify the output directory exists before writing any files. If the directory does not exist, the writer SHALL return an error without writing any files.

#### Scenario: Output directory exists
- **WHEN** the output directory is a valid existing path
- **THEN** PNG files are written successfully

#### Scenario: Output directory missing
- **WHEN** the output directory does not exist
- **THEN** the writer returns an error and no files are written

---

### Requirement: Return count of files written
The writer SHALL return the total number of PNG files successfully written upon completion.

#### Scenario: Count returned after writes
- **WHEN** 3 PNG files are written successfully
- **THEN** the writer returns the value 3
