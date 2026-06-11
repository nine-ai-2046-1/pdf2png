## ADDED Requirements

### Requirement: Vertically stack images of identical dimensions
The stacker SHALL accept a non-empty list of images that share the same width and height, and return a single image where all input images are stacked vertically (top to bottom) in order.

#### Scenario: Stack two same-size images
- **WHEN** given two images of size 800×1131
- **THEN** the output image has size 800×2262

#### Scenario: Stack single image (passthrough)
- **WHEN** given a list with one image
- **THEN** the output image is identical to the input image

#### Scenario: Stack multiple images
- **WHEN** given N images each of height H and width W
- **THEN** the output image has width W and height N×H

---

### Requirement: Stacker requires all images to have identical dimensions
The stacker SHALL return an error if any image in the list has dimensions that differ from the first image.

#### Scenario: Mismatched dimensions
- **WHEN** given images with different widths or heights
- **THEN** the stacker returns an error

---

### Requirement: No padding between stacked pages
The stacker SHALL stack images with zero pixel gap between them (pages touch directly).

#### Scenario: No gap in output
- **WHEN** two pages are stacked
- **THEN** the last row of page 1 is immediately above the first row of page 2 in the output
