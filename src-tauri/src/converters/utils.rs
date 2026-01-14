use image::ImageReader;
use std::path::Path;
use crate::models::ConversionError;

pub fn get_supported_formats(from_format: &str) -> Vec<String> {
    let format_lower = from_format.to_lowercase();
    
    // Check if it's a video format
    if is_video_format(&format_lower) {
        vec![
            "mp4".to_string(),
            "mkv".to_string(),
            "mov".to_string(),
            "webm".to_string(),
            "avi".to_string(),
            "flv".to_string(),
            "mpg".to_string(),
            "ts".to_string(),
            "ogv".to_string(),
        ]
    } else {
        // Image formats
        vec![
            "png".to_string(),
            "jpg".to_string(),
            "jpeg".to_string(),
            "webp".to_string(),
            "bmp".to_string(),
            "gif".to_string(),
            "tiff".to_string(),
            "ico".to_string(),
            "ppm".to_string(),
            "pgm".to_string(),
            "pbm".to_string(),
        ]
    }
}

pub fn is_image_format(format: &str) -> bool {
    matches!(
        format.to_lowercase().as_str(),
        "png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" | "tiff" | "ico" | "ppm" | "pgm" | "pbm"
        | "avif" | "heic" | "heif" | "tga" | "dds" | "apng" | "cur" | "exr" | "svg" | "pdf" | "psd" | "psb" | "fits" | "dcm" | "pcx" | "jfif"
    )
}

pub fn is_video_format(format: &str) -> bool {
    matches!(
        format.to_lowercase().as_str(),
        "mp4" | "m4v" | "mkv" | "mov" | "webm" | "avi" | "flv" | "mpg" | "mpeg" | "ts" | "m2ts" | "mts" | "ogv" | "ogg"
    )
}

/// Generic image converter that works for any image format pair
pub fn convert_any_image_format(input_path: &str, output_path: &str, target_format: &str) -> Result<(), ConversionError> {
    let input_path = Path::new(input_path);
    
    if !input_path.exists() {
        return Err(ConversionError::FileNotFound(input_path.to_string_lossy().to_string()));
    }

    // Auto-detect input format and decode
    let img = ImageReader::open(input_path)
        .map_err(|e| ConversionError::ImageError(format!("Failed to read image: {}", e)))?
        .decode()
        .map_err(|e| ConversionError::ImageError(format!("Failed to decode image: {}", e)))?;

    let output_path = Path::new(output_path);
    
    // Save with specified output format
    match target_format.to_lowercase().as_str() {
        "jpg" | "jpeg" => {
            img.save_with_format(output_path, image::ImageFormat::Jpeg)
                .map_err(|e| ConversionError::ImageError(format!("Failed to save JPEG: {}", e)))?;
        },
        "png" => {
            img.save_with_format(output_path, image::ImageFormat::Png)
                .map_err(|e| ConversionError::ImageError(format!("Failed to save PNG: {}", e)))?;
        },
        "webp" => {
            img.save_with_format(output_path, image::ImageFormat::WebP)
                .map_err(|e| ConversionError::ImageError(format!("Failed to save WebP: {}", e)))?;
        },
        "bmp" => {
            img.save_with_format(output_path, image::ImageFormat::Bmp)
                .map_err(|e| ConversionError::ImageError(format!("Failed to save BMP: {}", e)))?;
        },
        "gif" => {
            img.save_with_format(output_path, image::ImageFormat::Gif)
                .map_err(|e| ConversionError::ImageError(format!("Failed to save GIF: {}", e)))?;
        },
        "tiff" | "tif" => {
            img.save_with_format(output_path, image::ImageFormat::Tiff)
                .map_err(|e| ConversionError::ImageError(format!("Failed to save TIFF: {}", e)))?;
        },
        "ico" => {
            img.save_with_format(output_path, image::ImageFormat::Ico)
                .map_err(|e| ConversionError::ImageError(format!("Failed to save ICO: {}", e)))?;
        },
        "ppm" | "pgm" | "pbm" => {
            img.save_with_format(output_path, image::ImageFormat::Pnm)
                .map_err(|e| ConversionError::ImageError(format!("Failed to save PNM: {}", e)))?;
        },
        _ => return Err(ConversionError::UnsupportedFormat(target_format.to_string())),
    }

    Ok(())
}
