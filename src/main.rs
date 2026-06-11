/// main.rs — Entry point for pdf2png CLI.
/// Orchestrates the full pipeline: parse args → open PDF → prompt → render → write → report.
mod cli;
mod grouper;
mod prompt;
mod renderer;
mod reporter;
mod stacker;
mod writer;

use std::process;

fn main() {
    // Parse CLI arguments via clap
    let args = cli::parse_args();

    // Validate args (file existence, dir existence, dpi > 0)
    if let Err(e) = cli::validate_args(&args) {
        eprintln!("Error: {e}");
        process::exit(1);
    }

    // Run the pipeline; exit with code 1 on any error
    if let Err(e) = run(&args) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

/// Core pipeline: extract pdfium, open PDF, prompt if needed, render, write PNGs, report.
fn run(args: &cli::Args) -> Result<(), String> {
    // Extract embedded pdfium into a temp directory and initialise the engine.
    // _pdfium_tmp MUST be kept alive here so the library remains loaded for the
    // entire duration of the program.
    let (pdfium, _pdfium_tmp) = renderer::init_pdfium()?;

    // Open the PDF document
    let doc = renderer::open_pdf(&pdfium, &args.input)?;
    let page_count = doc.pages().len() as usize;

    // In smart-group mode, warn the user if the PDF has more than 5 pages
    prompt::check_page_limit(page_count, args.yes, args.smart_group)?;

    // Collect pixel dimensions for every page at the target DPI
    let dimensions: Vec<(u32, u32)> = (0..page_count)
        .map(|i| renderer::page_dimensions(&doc, i, args.dpi))
        .collect::<Result<Vec<_>, _>>()?;

    let output_images: Vec<image::DynamicImage> = if args.smart_group {
        // Group consecutive same-dimension pages, render + stack each group
        let groups = grouper::group_pages(&dimensions);
        let mut grouped_images = Vec::with_capacity(groups.len());

        for group in &groups {
            // Render all pages belonging to this group
            let pages: Vec<image::DynamicImage> = (group.start_index
                ..group.start_index + group.page_count)
                .map(|i| renderer::render_page(&doc, i, args.dpi))
                .collect::<Result<Vec<_>, _>>()?;

            // Vertically stack the group into a single image
            let stacked = stacker::stack_images(pages)?;
            grouped_images.push(stacked);
        }

        grouped_images
    } else {
        // Normal mode: one PNG per page
        let mut pages = Vec::with_capacity(page_count);
        for i in 0..page_count {
            pages.push(renderer::render_page(&doc, i, args.dpi)?);
        }
        pages
    };

    // Write numbered PNG files (1.png, 2.png, ...) to the output directory
    let count = writer::write_pngs(output_images, &args.output)?;

    // Print machine-readable JSON result to stdout (only stdout write in entire program)
    reporter::report_success(count);

    Ok(())
}
