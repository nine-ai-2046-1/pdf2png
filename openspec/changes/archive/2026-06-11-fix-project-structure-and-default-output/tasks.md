## 1. Move project to root

- [x] 1.1 Move `pdf2png/pdf2png/Cargo.toml` → `pdf2png/Cargo.toml`
- [x] 1.2 Move `pdf2png/pdf2png/Cargo.lock` → `pdf2png/Cargo.lock`
- [x] 1.3 Move `pdf2png/pdf2png/build.rs` → `pdf2png/build.rs`
- [x] 1.4 Move `pdf2png/pdf2png/src/` → `pdf2png/src/`
- [x] 1.5 Move `pdf2png/pdf2png/tests/` → `pdf2png/tests/`
- [x] 1.6 Move `pdf2png/pdf2png/.git/` → `pdf2png/.git/`
- [x] 1.7 Move `pdf2png/pdf2png/.gitignore` → `pdf2png/.gitignore`
- [x] 1.8 Remove the now-empty `pdf2png/pdf2png/` directory
- [x] 1.9 Verify `cargo build` succeeds from project root

## 2. Change default output path

- [x] 2.1 In `src/cli.rs`: change `-o` default_value from `"."` to `"./output"`
- [x] 2.2 Run `cargo test` — confirm all existing tests pass (no test changes needed)
- [x] 2.3 Run `cargo clippy` — confirm zero warnings
