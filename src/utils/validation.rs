use crate::models::{ImageValidation, ValidationStatus, NormalLayout};
use image::{DynamicImage, GenericImageView};
use std::path::Path;
use crate::utils::LabelError;


pub fn load_image_robustly(path: &Path) -> Result<DynamicImage, LabelError> {
    let bytes = std::fs::read(path)
        .map_err(|e| LabelError::ImageLoading(format!("Failed to read file: {}", e)))?;

    let format = image::guess_format(&bytes)
        .map_err(|e| LabelError::ImageLoading(format!("Could not determine image format: {}", e)))?;

    image::load_from_memory_with_format(&bytes, format)
        .map_err(|e| LabelError::ImageLoading(format!("Failed to decode image: {}", e)))
}

pub fn validate_user_image(image: &DynamicImage) -> ImageValidation {
    let (width, height) = image.dimensions();
    let target = (
        NormalLayout::USER_IMAGE.width,
        NormalLayout::USER_IMAGE.height,
    );

    let source_ratio = width as f32 / height as f32;
    let target_ratio = target.0 as f32 / target.1 as f32;

    let tolerance = 0.02;
    let ratio_diff = (source_ratio - target_ratio).abs();

    if ratio_diff < tolerance {
        ImageValidation {
            status: ValidationStatus::PerfectFit,
            source_dimensions: (width, height),
            target_dimensions: target,
            message: format!("OK: Image is perfect ({}×{})", width, height),
        }
    } else {
        let message = if source_ratio > target_ratio {
            format!(
                "Warning: Image will be cropped ({}×{} → {}×{})",
                width, height, target.0, target.1
            )
        } else {
            format!(
                "Warning: Image will be cropped ({}×{} → {}×{})",
                width, height, target.0, target.1
            )
        };

        ImageValidation {
            status: ValidationStatus::WillCrop,
            source_dimensions: (width, height),
            target_dimensions: target,
            message,
        }
    }
}