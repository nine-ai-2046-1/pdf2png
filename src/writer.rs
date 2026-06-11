/// writer.rs — Writes PNG images to the output directory with sequential integer names.
/// File naming: 1.png, 2.png, 3.png ...
use image::DynamicImage;
use std::path::Path;

/// Write a list of images as numbered PNG files into output_dir.
/// Files are named 1.png, 2.png, ... in order.
/// Returns the count of files successfully written.
/// Returns Err if the output directory does not exist.
pub fn write_pngs(images: Vec<DynamicImage>, output_dir: &Path) -> Result<usize, String> {
    // Verify output directory exists before writing any file
    if !output_dir.exists() || !output_dir.is_dir() {
        return Err(format!(
            "Output directory does not exist: {}",
            output_dir.display()
        ));
    }

    let mut count = 0;

    for (i, img) in images.into_iter().enumerate() {
        // Sequential 1-based naming: 1.png, 2.png ...
        let file_name = format!("{}.png", i + 1);
        let file_path = output_dir.join(&file_name);

        img.save_with_format(&file_path, image::ImageFormat::Png)
            .map_err(|e| format!("Failed to write {}: {}", file_path.display(), e))?;

        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};
    use tempfile::tempdir;

    /// Create a minimal blank image for testing.
    fn blank_image() -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::new(10, 10))
    }

    #[test]
    fn test_sequential_naming_starts_at_one() {
        let dir = tempdir().unwrap();
        let images = vec![blank_image(), blank_image(), blank_image()];
        let count = write_pngs(images, dir.path()).unwrap();
        assert_eq!(count, 3);
        assert!(dir.path().join("1.png").exists());
        assert!(dir.path().join("2.png").exists());
        assert!(dir.path().join("3.png").exists());
        assert!(!dir.path().join("4.png").exists());
    }

    #[test]
    fn test_count_returned_equals_images_written() {
        let dir = tempdir().unwrap();
        let images = vec![blank_image(), blank_image()];
        let count = write_pngs(images, dir.path()).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_error_on_missing_output_directory() {
        let result = write_pngs(vec![blank_image()], Path::new("/nonexistent/dir/xyz"));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Output directory does not exist")
        );
    }

    #[test]
    fn test_empty_image_list_returns_zero() {
        let dir = tempdir().unwrap();
        let count = write_pngs(vec![], dir.path()).unwrap();
        assert_eq!(count, 0);
    }
}
