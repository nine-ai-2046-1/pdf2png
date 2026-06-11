/// stacker.rs — Vertically stacks multiple same-dimension images into one.
/// Used in smart-group mode to combine pages of a single group into one PNG.
use image::{DynamicImage, GenericImage, RgbaImage};

/// Vertically stack a list of images (top to bottom) into a single DynamicImage.
/// All images MUST share identical width and height.
/// Returns an error if the list is empty or contains mismatched dimensions.
pub fn stack_images(images: Vec<DynamicImage>) -> Result<DynamicImage, String> {
    if images.is_empty() {
        return Err("Cannot stack an empty list of images".to_string());
    }

    // Single image: passthrough without copying
    if images.len() == 1 {
        return Ok(images.into_iter().next().unwrap());
    }

    let reference_width = images[0].width();
    let reference_height = images[0].height();

    // Validate all images share identical dimensions
    for (i, img) in images.iter().enumerate() {
        if img.width() != reference_width || img.height() != reference_height {
            return Err(format!(
                "Image {} has dimensions {}x{} but expected {}x{}",
                i,
                img.width(),
                img.height(),
                reference_width,
                reference_height
            ));
        }
    }

    let total_height = reference_height * images.len() as u32;

    // Create a blank RGBA canvas with the combined height
    let mut canvas = RgbaImage::new(reference_width, total_height);

    // Copy each image into the canvas at the correct vertical offset
    for (i, img) in images.iter().enumerate() {
        let y_offset = i as u32 * reference_height;
        let rgba = img.to_rgba8();
        canvas
            .copy_from(&rgba, 0, y_offset)
            .map_err(|e| format!("Failed to copy image {} onto canvas: {}", i, e))?;
    }

    Ok(DynamicImage::ImageRgba8(canvas))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};

    /// Create a solid-colour RGBA image for testing.
    fn make_image(width: u32, height: u32) -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::new(width, height))
    }

    #[test]
    fn test_stack_two_same_size_images() {
        let images = vec![make_image(100, 200), make_image(100, 200)];
        let result = stack_images(images).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 400); // 200 + 200
    }

    #[test]
    fn test_stack_single_image_passthrough() {
        let images = vec![make_image(100, 200)];
        let result = stack_images(images).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 200);
    }

    #[test]
    fn test_stack_multiple_images() {
        let n = 5u32;
        let images = (0..n).map(|_| make_image(80, 120)).collect();
        let result = stack_images(images).unwrap();
        assert_eq!(result.width(), 80);
        assert_eq!(result.height(), 600); // 120 * 5
    }

    #[test]
    fn test_mismatch_dimensions_returns_error() {
        let images = vec![make_image(100, 200), make_image(100, 300)];
        let result = stack_images(images);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("dimensions"));
    }

    #[test]
    fn test_empty_input_returns_error() {
        let result = stack_images(vec![]);
        assert!(result.is_err());
    }
}
