use image::ImageReader;
use std::path::Path;
use crate::models::ConversionError;

pub fn convert_pcx(input_path: &str, output_path: &str, target_format: &str) -> Result<(), ConversionError> {
    let input_path = Path::new(input_path);
    
    if !input_path.exists() {
        return Err(ConversionError::FileNotFound(input_path.to_string_lossy().to_string()));
    }

    let img = ImageReader::open(input_path)
        .map_err(|e| ConversionError::ImageError(e.to_string()))?
        .decode()
        .map_err(|e| ConversionError::ImageError(e.to_string()))?;

    let output_path = Path::new(output_path);
    
    match target_format.to_lowercase().as_str() {
        "png" => {
            img.save_with_format(output_path, image::ImageFormat::Png)
                .map_err(|e| ConversionError::ImageError(e.to_string()))?;
        },
        "jpg" | "jpeg" => {
            img.save_with_format(output_path, image::ImageFormat::Jpeg)
                .map_err(|e| ConversionError::ImageError(e.to_string()))?;
        },
        "bmp" => {
            img.save_with_format(output_path, image::ImageFormat::Bmp)
                .map_err(|e| ConversionError::ImageError(e.to_string()))?;
        },
        "tiff" | "tif" => {
            img.save_with_format(output_path, image::ImageFormat::Tiff)
                .map_err(|e| ConversionError::ImageError(e.to_string()))?;
        },
        "webp" => {
            img.save_with_format(output_path, image::ImageFormat::WebP)
                .map_err(|e| ConversionError::ImageError(e.to_string()))?;
        },
        "gif" => {
            img.save_with_format(output_path, image::ImageFormat::Gif)
                .map_err(|e| ConversionError::ImageError(e.to_string()))?;
        },        "ppm" | "pgm" | "pbm" => {
            img.save_with_format(output_path, image::ImageFormat::Pnm)
                .map_err(|e| ConversionError::ImageError(e.to_string()))?
        },        _ => return Err(ConversionError::UnsupportedFormat(target_format.to_string())),
    }

    Ok(())
}
