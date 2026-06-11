/// Integration tests for pdf2png.
/// These tests run the binary as a subprocess so they exercise the full
/// pipeline: argument parsing → pdfium rendering → PNG writing → JSON stdout.
///
/// Fixtures:
///   tests/fixtures/sample_3page.pdf  — 3 pages: p1 portrait, p2 portrait, p3 landscape
///   tests/fixtures/sample_6page.pdf  — 6 pages: all portrait (triggers warning with -s)
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

/// Return the path to the compiled pdf2png binary under test.
fn binary_path() -> PathBuf {
    // cargo test sets CARGO_BIN_EXE_<name> for binary crates
    let exe = env!("CARGO_BIN_EXE_pdf2png");
    PathBuf::from(exe)
}

/// Path to a test fixture file.
fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Run pdf2png with the given args, return (stdout, stderr, exit_code).
fn run(args: &[&str]) -> (String, String, i32) {
    let output = Command::new(binary_path())
        .args(args)
        .output()
        .expect("Failed to run pdf2png binary");
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.code().unwrap_or(-1),
    )
}

// ─────────────────────────────────────────────
// 10.2  Normal mode: correct file count + naming
// ─────────────────────────────────────────────
#[test]
fn test_normal_mode_produces_correct_png_files() {
    let out_dir = tempdir().unwrap();
    let (stdout, _stderr, code) = run(&[
        "-i",
        fixture("sample_3page.pdf").to_str().unwrap(),
        "-o",
        out_dir.path().to_str().unwrap(),
    ]);

    assert_eq!(code, 0, "expected exit 0, got stderr above");
    // Normal mode: 3 pages → 3 PNGs
    assert!(out_dir.path().join("1.png").exists(), "1.png missing");
    assert!(out_dir.path().join("2.png").exists(), "2.png missing");
    assert!(out_dir.path().join("3.png").exists(), "3.png missing");
    assert!(!out_dir.path().join("4.png").exists(), "unexpected 4.png");
    // JSON count must match
    let json: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("stdout is not valid JSON");
    assert_eq!(json["count"], 3);
    assert_eq!(json["success"], true);
}

// ─────────────────────────────────────────────
// 10.3  -s mode: same-dimension pages → single PNG
// ─────────────────────────────────────────────
#[test]
fn test_smart_group_same_dimension_produces_one_png() {
    let out_dir = tempdir().unwrap();
    // Pages 1+2 are both portrait 595×842 → should stack into 1.png
    // Page 3 is landscape 842×595 → separate 2.png
    let (stdout, _stderr, code) = run(&[
        "-i",
        fixture("sample_3page.pdf").to_str().unwrap(),
        "-o",
        out_dir.path().to_str().unwrap(),
        "-s",
        "-y", // 3 pages ≤ 5 so no prompt, but -y is harmless
    ]);

    assert_eq!(code, 0);
    // 2 groups: [p1+p2 portrait], [p3 landscape]
    assert!(out_dir.path().join("1.png").exists(), "1.png missing");
    assert!(out_dir.path().join("2.png").exists(), "2.png missing");
    assert!(
        !out_dir.path().join("3.png").exists(),
        "unexpected 3.png — grouping failed"
    );

    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["count"], 2);
}

// ─────────────────────────────────────────────
// 10.4  -s mode: mixed dimensions → multiple grouped PNGs
// ─────────────────────────────────────────────
#[test]
fn test_smart_group_mixed_dimensions_produces_multiple_pngs() {
    let out_dir = tempdir().unwrap();
    let (stdout, _stderr, code) = run(&[
        "-i",
        fixture("sample_3page.pdf").to_str().unwrap(),
        "-o",
        out_dir.path().to_str().unwrap(),
        "-s",
    ]);

    assert_eq!(code, 0);
    // sample_3page.pdf: portrait, portrait, landscape → 2 groups
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(
        json["count"], 2,
        "expected 2 groups from mixed-dimension PDF"
    );
}

// ─────────────────────────────────────────────
// 10.5  stdout JSON format
// ─────────────────────────────────────────────
#[test]
fn test_stdout_json_format_is_correct() {
    let out_dir = tempdir().unwrap();
    let (stdout, _stderr, code) = run(&[
        "-i",
        fixture("sample_3page.pdf").to_str().unwrap(),
        "-o",
        out_dir.path().to_str().unwrap(),
    ]);

    assert_eq!(code, 0);
    let trimmed = stdout.trim();
    // Must be valid JSON with exactly these keys
    let json: serde_json::Value = serde_json::from_str(trimmed).expect("stdout is not valid JSON");
    assert_eq!(json["success"], true);
    assert!(json["count"].is_number());
}

// ─────────────────────────────────────────────
// 10.6  -s with 6-page PDF + -y bypasses prompt
// ─────────────────────────────────────────────
#[test]
fn test_smart_group_large_pdf_with_yes_flag_proceeds() {
    let out_dir = tempdir().unwrap();
    let (stdout, _stderr, code) = run(&[
        "-i",
        fixture("sample_6page.pdf").to_str().unwrap(),
        "-o",
        out_dir.path().to_str().unwrap(),
        "-s",
        "-y", // should bypass the >5 page warning
    ]);

    assert_eq!(code, 0, "expected exit 0 with -y bypassing prompt");
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["success"], true);
    assert!(json["count"].as_u64().unwrap() >= 1);
}

// ─────────────────────────────────────────────
// Error paths: no JSON on failure
// ─────────────────────────────────────────────
#[test]
fn test_missing_input_file_exits_nonzero_and_no_json() {
    let out_dir = tempdir().unwrap();
    let (stdout, _stderr, code) = run(&[
        "-i",
        "/nonexistent/file.pdf",
        "-o",
        out_dir.path().to_str().unwrap(),
    ]);
    assert_ne!(code, 0, "should exit non-zero for missing file");
    assert!(stdout.trim().is_empty(), "stdout should be empty on error");
}
