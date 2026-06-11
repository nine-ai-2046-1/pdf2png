/// cli.rs — CLI argument parsing using clap derive macros.
/// Defines the `Args` struct and `parse_args()` entry point.
use clap::Parser;
use std::path::PathBuf;

/// Parsed and validated CLI arguments for pdf2png.
#[derive(Parser, Debug)]
#[command(name = "pdf2png", about = "Convert PDF pages to PNG images")]
pub struct Args {
    /// Path to the input PDF file
    #[arg(short = 'i', long = "input")]
    pub input: PathBuf,

    /// Output folder for PNG files (default: ./output)
    #[arg(short = 'o', long = "output", default_value = "./output")]
    pub output: PathBuf,

    /// Render DPI (default: 300)
    #[arg(short = 'd', long = "dpi", default_value_t = 300)]
    pub dpi: u32,

    /// Smart-group mode: group consecutive same-dimension pages into one PNG
    #[arg(short = 's', long = "smart-group")]
    pub smart_group: bool,

    /// Auto-confirm all prompts (bypass warnings)
    #[arg(short = 'y', long = "yes")]
    pub yes: bool,

    /// Whether the user explicitly provided -o (set in parse_args, not by clap)
    #[arg(skip)]
    pub output_provided: bool,
}

/// Parse and return CLI arguments.
/// Clap handles missing required fields and type errors automatically.
pub fn parse_args() -> Args {
    let mut args = Args::parse();
    // Detect whether -o was explicitly provided by scanning raw argv.
    // This lets us distinguish "user omitted -o" from "user passed -o ./output".
    args.output_provided = std::env::args().any(|a| a == "-o" || a == "--output");
    args
}

/// Validate parsed args: check input file exists, output dir exists, dpi > 0.
/// Returns Err with a human-readable message on failure.
pub fn validate_args(args: &Args) -> Result<(), String> {
    // Input file must exist and be a file
    if !args.input.exists() {
        return Err(format!("Input file not found: {}", args.input.display()));
    }
    if !args.input.is_file() {
        return Err(format!(
            "Input path is not a file: {}",
            args.input.display()
        ));
    }

    // Output directory must exist
    if !args.output.exists() {
        if args.output_provided {
            // User explicitly passed -o but the path doesn't exist
            return Err(format!(
                "Output directory not found: {}",
                args.output.display()
            ));
        } else {
            // User didn't pass -o; default ./output is missing — show usage example
            return Err(
                "Output directory not found: ./output\n\n\
                 Hint: create it first, or specify a different path:\n  \
                 pdf2png -i input.pdf -o output-folder-path"
                    .to_string(),
            );
        }
    }
    if !args.output.is_dir() {
        return Err(format!(
            "Output path is not a directory: {}",
            args.output.display()
        ));
    }

    // DPI must be positive
    if args.dpi == 0 {
        return Err("DPI must be greater than 0".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Helper to build a minimal valid Args for testing.
    fn make_args(input: PathBuf, output: PathBuf, dpi: u32) -> Args {
        Args {
            input,
            output,
            dpi,
            smart_group: false,
            yes: false,
            output_provided: false,
        }
    }

    #[test]
    fn test_missing_input_file_returns_error() {
        let args = make_args(
            PathBuf::from("/nonexistent/file.pdf"),
            PathBuf::from("."),
            300,
        );
        let result = validate_args(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Input file not found"));
    }

    #[test]
    fn test_missing_output_dir_returns_error() {
        // Explicit -o with nonexistent dir → generic error (no usage hint)
        let mut args = make_args(
            PathBuf::from(file!()),
            PathBuf::from("/nonexistent/output/dir"),
            300,
        );
        args.output_provided = true;
        let result = validate_args(&args);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Output directory not found"));
        // Must NOT contain usage hint — that's only for default output
        assert!(!err.contains("Hint:"));
    }

    #[test]
    fn test_zero_dpi_returns_error() {
        let args = make_args(PathBuf::from(file!()), PathBuf::from("."), 0);
        let result = validate_args(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("DPI must be greater than 0"));
    }

    #[test]
    fn test_valid_args_pass_validation() {
        // Use the source file itself as a stand-in "input" (exists), "." as output
        let args = make_args(PathBuf::from(file!()), PathBuf::from("."), 300);
        // output dir "." always exists; input file is this source file
        assert!(validate_args(&args).is_ok());
    }

    #[test]
    fn test_default_output_missing_shows_usage_hint() {
        // Default output (user didn't pass -o) with nonexistent path → usage hint
        let mut args = make_args(
            PathBuf::from(file!()),
            PathBuf::from("/nonexistent/default/output"),
            300,
        );
        args.output_provided = false;
        let result = validate_args(&args);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Output directory not found:"));
        assert!(err.contains("Hint:"));
        assert!(err.contains("pdf2png -i input.pdf -o output-folder-path"));
    }
}
