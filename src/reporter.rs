/// reporter.rs — Outputs the final machine-readable JSON result to stdout.
/// All other messages in the codebase MUST use eprintln! (stderr).
use std::io::{self, Write};

/// Print the success JSON result to stdout.
/// Format: {"success":true,"count":N}
/// This is the only function in the codebase permitted to write to stdout.
pub fn report_success(count: usize) {
    // Use print! + flush to guarantee stdout is flushed before process exits
    print!("{{\"success\":true,\"count\":{}}}", count);
    io::stdout().flush().ok();
    println!(); // trailing newline for shell friendliness
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: capturing stdout in Rust unit tests requires process-level redirection.
    // The JSON format is validated by integration tests (tests/integration_test.rs).
    // This module-level test verifies the function is callable without panic.
    #[test]
    fn test_report_success_does_not_panic() {
        // Cannot capture stdout in unit tests without extra crates;
        // functional correctness covered by integration tests.
        report_success(0);
        report_success(42);
    }
}
