/// prompt.rs — User confirmation prompt for large PDFs in smart-group mode.
/// Warns on stderr and reads y/n from stdin. Respects the -y auto-confirm flag.
use std::io::{self, BufRead, Write};

/// Check page count and prompt the user if needed.
/// Only triggers when smart_group is true AND page_count > 5.
/// If yes is true, skips the prompt entirely.
/// Returns Ok(()) to proceed, Err if the user declines or input is unrecognised.
pub fn check_page_limit(page_count: usize, yes: bool, smart_group: bool) -> Result<(), String> {
    // Prompt only applies in smart-group mode with more than 5 pages
    if !smart_group || page_count <= 5 {
        return Ok(());
    }

    // Print warning to stderr (not stdout, which is reserved for JSON)
    eprintln!(
        "Warning: PDF has {} pages. In smart-group mode this may produce large stacked images.",
        page_count
    );

    // Auto-confirm: skip stdin read
    if yes {
        return Ok(());
    }

    // Prompt the user
    eprint!("Continue? [y/N]: ");
    io::stderr().flush().ok();

    // Read one line from stdin
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    let answer = line.trim().to_lowercase();

    match answer.as_str() {
        "y" | "yes" => Ok(()),
        _ => Err("Aborted by user.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_prompt_when_page_count_at_limit() {
        // Exactly 5 pages — no prompt, always Ok
        assert!(check_page_limit(5, false, true).is_ok());
    }

    #[test]
    fn test_no_prompt_when_page_count_below_limit() {
        assert!(check_page_limit(3, false, true).is_ok());
    }

    #[test]
    fn test_no_prompt_outside_smart_group_mode() {
        // smart_group = false: even 100 pages, no prompt
        assert!(check_page_limit(100, false, false).is_ok());
    }

    #[test]
    fn test_auto_confirm_with_yes_flag() {
        // smart_group = true, page_count > 5, yes = true → immediate Ok
        assert!(check_page_limit(10, true, true).is_ok());
    }
    // Note: interactive stdin tests (y/n) are covered by integration tests
    // which can pipe stdin. Unit-testing stdin reads requires dependency injection.
}
