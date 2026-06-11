## Context

Pure documentation change — no code modification. The project is a Rust CLI tool that converts PDF pages to PNG images. Documentation should be accurate, concise, and cover: background/motivation, installation (cargo install), usage examples for both normal and smart-group modes, full CLI reference, explanation of the smart-group orientation logic, and a project structure diagram.

## Goals / Non-Goals

**Goals:**
- Clear, accurate README that gets a developer from zero to running the tool
- Cover smart-group mode logic (consecutive same-dimension page grouping)
- Cover orientation handling (portrait/landscape detection)
- Bilingual: English + Cantonese (Traditional Chinese)

**Non-Goals:**
- API documentation (internal modules, not public API)
- Contributing guide (not needed yet)
- Changelog (no releases yet)

## Decisions

### D1: Document structure

Both READMEs follow the same structure:

```
1. Title + one-line description
2. Background / Why this tool
3. Features
4. Installation
5. Quick Start
6. CLI Reference (flags table)
7. Smart-Group Mode (orientation logic + examples)
8. Project Structure
9. JSON Output
10. License
```

### D2: Cantonese localisation

`README-HK.md` will be written in Cantonese with Traditional Chinese characters, matching the team's native language. Technical terms (CLI flags, file names, code) remain in English.

## Risks / Trade-offs

- **[Risk] Documentation drift** → Docs may become outdated as features change. **Mitigation**: keep docs minimal and close to the code.
