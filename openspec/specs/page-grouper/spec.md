## ADDED Requirements

### Requirement: Group consecutive same-dimension pages
The grouper SHALL accept a list of page dimension tuples `[(width_px, height_px)]` and return a list of groups, where each group is a contiguous slice of pages sharing identical dimensions. A new group SHALL start whenever the dimensions change from the previous page.

#### Scenario: All pages same dimension
- **WHEN** all pages have identical dimensions
- **THEN** one group containing all pages is returned

#### Scenario: Mixed dimensions
- **WHEN** pages are `[portrait, portrait, landscape, landscape, portrait]`
- **THEN** three groups are returned: `[pages 1-2]`, `[pages 3-4]`, `[page 5]`

#### Scenario: Every page different dimension
- **WHEN** every page has a unique dimension
- **THEN** each page forms its own group of size 1

#### Scenario: Non-consecutive same-dimension pages are not merged
- **WHEN** pages are `[portrait, landscape, portrait]`
- **THEN** three separate groups are returned, not two

---

### Requirement: Grouper is agnostic to content
The grouper SHALL operate only on dimension tuples and SHALL NOT require access to the rendered image data.

#### Scenario: Grouper receives only dimensions
- **WHEN** given a list of `(u32, u32)` tuples
- **THEN** it returns correct group boundaries without needing image data
