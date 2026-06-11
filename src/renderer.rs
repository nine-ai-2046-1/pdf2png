/// renderer.rs — PDF rendering via pdfium-render.
/// Loads the embedded pdfium library (bundled at build time) into a temp
/// directory, initialises Pdfium, and exposes page dimension + render helpers.
use image::DynamicImage;
use pdfium_render::prelude::*;
use std::path::Path;
use tempfile::TempDir;

// Include the auto-generated file from build.rs that contains PDFIUM_LIB_BYTES
// and PDFIUM_LIB_NAME.
include!(concat!(env!("OUT_DIR"), "/pdfium_embedded.rs"));

/// Initialise Pdfium by extracting the embedded library into a temporary
/// directory and loading it dynamically.  Returns both the Pdfium handle and
/// the TempDir — the caller MUST keep TempDir alive for as long as Pdfium is used.
pub fn init_pdfium() -> Result<(Pdfium, TempDir), String> {
    // Write embedded bytes to a temp directory so dlopen / LoadLibrary can find it
    let tmp =
        TempDir::new().map_err(|e| format!("Failed to create temp directory for pdfium: {e}"))?;

    let lib_path = tmp.path().join(PDFIUM_LIB_NAME);

    std::fs::write(&lib_path, PDFIUM_LIB_BYTES)
        .map_err(|e| format!("Failed to extract embedded pdfium library: {e}"))?;

    let bindings = Pdfium::bind_to_library(&lib_path)
        .map_err(|e| format!("Failed to load pdfium library: {e}"))?;

    // Pdfium::new() takes ownership of bindings and returns Pdfium directly
    let pdfium = Pdfium::new(bindings);

    Ok((pdfium, tmp))
}

/// Open a PDF document from the given file path.
/// Returns an error if the file cannot be loaded as a valid PDF.
pub fn open_pdf<'a>(pdfium: &'a Pdfium, path: &Path) -> Result<PdfDocument<'a>, String> {
    pdfium
        .load_pdf_from_file(path, None)
        .map_err(|e| format!("Failed to open PDF '{}': {e}", path.display()))
}

/// Return the pixel dimensions (width, height) of a single PDF page at the given DPI.
/// page_index is 0-based.
pub fn page_dimensions(
    doc: &PdfDocument,
    page_index: usize,
    dpi: u32,
) -> Result<(u32, u32), String> {
    let pages = doc.pages();
    let page = pages
        .get(page_index as u16)
        .map_err(|e| format!("Failed to access page {page_index}: {e}"))?;

    // Convert PDF points (1 pt = 1/72 inch) to pixels at target DPI
    let width_px = (page.width().value * dpi as f32 / 72.0).round() as u32;
    let height_px = (page.height().value * dpi as f32 / 72.0).round() as u32;

    Ok((width_px, height_px))
}

/// Render a single PDF page to a DynamicImage at the given DPI.
/// page_index is 0-based.
pub fn render_page(doc: &PdfDocument, page_index: usize, dpi: u32) -> Result<DynamicImage, String> {
    let pages = doc.pages();
    let page = pages
        .get(page_index as u16)
        .map_err(|e| format!("Failed to access page {page_index}: {e}"))?;

    let target_width = (page.width().value * dpi as f32 / 72.0).round() as i32;
    let target_height = (page.height().value * dpi as f32 / 72.0).round() as i32;

    // Render to an RGB bitmap at the target DPI
    let bitmap = page
        .render_with_config(
            &PdfRenderConfig::new()
                .set_target_width(target_width)
                .set_maximum_height(target_height)
                .rotate_if_landscape(PdfPageRenderRotation::None, true),
        )
        .map_err(|e| format!("Failed to render page {page_index}: {e}"))?;

    let rgb = bitmap.as_image().into_rgb8();
    Ok(DynamicImage::ImageRgb8(rgb))
}
