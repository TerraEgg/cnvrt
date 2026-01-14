use crate::converters;
use crate::models::{ConversionResult, ConversionError};
use std::io::Write;
use std::path::{Path, PathBuf};

#[tauri::command]
pub async fn convert_image(
    input_data: Vec<u8>,
    output_path: String,
    from_format: String,
    to_format: String,
    _keep_transparency: bool,
) -> Result<ConversionResult, String> {
    let temp_dir = std::env::temp_dir();
    let temp_input = temp_dir.join(format!("cnvrt_input_{}.{}", uuid::Uuid::new_v4(), from_format.to_lowercase()));
    
    match std::fs::File::create(&temp_input) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(&input_data) {
                return Ok(ConversionResult {
                    id: uuid::Uuid::new_v4().to_string(),
                    success: false,
                    message: format!("Failed to write input file: {}", e),
                    output_path: None,
                });
            }
        }
        Err(e) => {
            return Ok(ConversionResult {
                id: uuid::Uuid::new_v4().to_string(),
                success: false,
                message: format!("Failed to create temp file: {}", e),
                output_path: None,
            });
        }
    }

    let temp_input_str = temp_input.to_string_lossy().to_string();
    
    let final_output_path = if output_path.is_empty() || output_path == "." {
        let downloads = dirs::download_dir()
            .unwrap_or_else(|| PathBuf::from(&output_path))
            .join(Path::new(&output_path).file_name().unwrap_or_default());
        downloads.to_string_lossy().to_string()
    } else {
        output_path.clone()
    };
    
    let to_format_lower = to_format.to_lowercase();
    
    let result = if to_format_lower == "gif" {
        converters::utils::convert_any_image_format(&temp_input_str, &final_output_path, &to_format)
    } else {
        match from_format.to_lowercase().as_str() {
            "png" => converters::png::convert_png(&temp_input_str, &final_output_path, &to_format),
            "jpg" | "jpeg" | "jfif" => converters::jpg::convert_jpg(&temp_input_str, &final_output_path, &to_format),
            "webp" => converters::webp::convert_webp(&temp_input_str, &final_output_path, &to_format),
            "bmp" => converters::bmp::convert_bmp(&temp_input_str, &final_output_path, &to_format),
            "gif" => converters::gif::convert_gif(&temp_input_str, &final_output_path, &to_format),
            "tiff" | "tif" => converters::tiff::convert_tiff(&temp_input_str, &final_output_path, &to_format),
            "ico" => converters::ico::convert_ico(&temp_input_str, &final_output_path, &to_format),
            "avif" => converters::avif::convert_avif(&temp_input_str, &final_output_path, &to_format),
            "heic" | "heif" => converters::heic::convert_heic(&temp_input_str, &final_output_path, &to_format),
            "ppm" | "pgm" | "pbm" => converters::ppm::convert_ppm(&temp_input_str, &final_output_path, &to_format),
            "tga" => converters::tga::convert_tga(&temp_input_str, &final_output_path, &to_format),
            "dds" => converters::dds::convert_dds(&temp_input_str, &final_output_path, &to_format),
            "apng" => converters::apng::convert_apng(&temp_input_str, &final_output_path, &to_format),
            "cur" => converters::cur::convert_cur(&temp_input_str, &final_output_path, &to_format),
            "exr" => converters::exr::convert_exr(&temp_input_str, &final_output_path, &to_format),
            "svg" => converters::svg::convert_svg(&temp_input_str, &final_output_path, &to_format),
            "pdf" => converters::pdf::convert_pdf(&temp_input_str, &final_output_path, &to_format),
            "psd" | "psb" => converters::psd::convert_psd(&temp_input_str, &final_output_path, &to_format),
            "fits" => converters::fits::convert_fits(&temp_input_str, &final_output_path, &to_format),
            "dcm" => converters::dcm::convert_dcm(&temp_input_str, &final_output_path, &to_format),
            "pcx" => converters::pcx::convert_pcx(&temp_input_str, &final_output_path, &to_format),
            "mp4" | "m4v" | "mkv" | "mov" | "webm" | "avi" | "flv" | "mpg" | "mpeg" | "ts" | "m2ts" | "mts" | "ogv" | "ogg" => {
                converters::video::transcoder::convert_video(&temp_input_str, &final_output_path, &to_format)
            }
            _ => Err(ConversionError::UnsupportedFormat(from_format.clone())),
        }
    };

    match result {
        Ok(_) => Ok(ConversionResult {
            id: uuid::Uuid::new_v4().to_string(),
            success: true,
            message: format!("Successfully converted {} to {}", from_format, to_format),
            output_path: Some(final_output_path),
        }),
        Err(e) => Ok(ConversionResult {
            id: uuid::Uuid::new_v4().to_string(),
            success: false,
            message: e.to_string(),
            output_path: None,
        }),
    }
}

#[tauri::command]
pub fn get_supported_formats(format: String) -> Vec<String> {
    crate::converters::utils::get_supported_formats(&format)
}

#[tauri::command]
pub fn is_supported_format(format: String) -> bool {
    crate::converters::utils::is_image_format(&format) || crate::converters::utils::is_video_format(&format)
}

#[tauri::command]
pub fn select_folder() -> Option<String> {
    rfd::FileDialog::new()
        .pick_folder()
        .map(|path| path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn get_downloads_folder() -> Option<String> {
    dirs::download_dir()
        .map(|path| path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn pick_files() -> Option<Vec<String>> {
    rfd::FileDialog::new()
        .add_filter("All Supported", &[
            "png", "jpg", "jpeg", "webp", "bmp", "gif", "tiff", "tif", "ico", "avif", "heic", "heif",
            "mp4", "mkv", "mov", "webm", "avi", "flv", "mpg", "mpeg", "ts", "m2ts", "mts", "ogv", "ogg"
        ])
        .add_filter("Images", &["png", "jpg", "jpeg", "webp", "bmp", "gif", "tiff", "tif", "ico", "avif", "heic", "heif"])
        .add_filter("Videos", &["mp4", "mkv", "mov", "webm", "avi", "flv", "mpg", "mpeg", "ts", "m2ts", "mts", "ogv", "ogg"])
        .add_filter("All Files", &["*"])
        .pick_files()
        .map(|paths| paths.iter().map(|p| p.to_string_lossy().to_string()).collect())
}

#[tauri::command]
pub async fn convert_from_path(
    input_path: String,
    output_path: String,
    from_format: String,
    to_format: String,
) -> Result<ConversionResult, String> {
    let target_lower = to_format.to_lowercase();
    let from_lower = from_format.to_lowercase();
    
    let result = if target_lower == "gif" {
        crate::converters::utils::convert_any_image_format(&input_path, &output_path, &to_format)
    } else {
        match from_lower.as_str() {
            // Image formats
            "png" => crate::converters::png::convert_png(&input_path, &output_path, &to_format),
            "jpg" | "jpeg" | "jfif" => crate::converters::jpg::convert_jpg(&input_path, &output_path, &to_format),
            "webp" => crate::converters::webp::convert_webp(&input_path, &output_path, &to_format),
            "bmp" => crate::converters::bmp::convert_bmp(&input_path, &output_path, &to_format),
            "gif" => crate::converters::gif::convert_gif(&input_path, &output_path, &to_format),
            "tiff" | "tif" => crate::converters::tiff::convert_tiff(&input_path, &output_path, &to_format),
            "ico" => crate::converters::ico::convert_ico(&input_path, &output_path, &to_format),
            "avif" => crate::converters::avif::convert_avif(&input_path, &output_path, &to_format),
            "heic" | "heif" => crate::converters::heic::convert_heic(&input_path, &output_path, &to_format),
            "ppm" | "pgm" | "pbm" => crate::converters::ppm::convert_ppm(&input_path, &output_path, &to_format),
            "tga" => crate::converters::tga::convert_tga(&input_path, &output_path, &to_format),
            "dds" => crate::converters::dds::convert_dds(&input_path, &output_path, &to_format),
            "apng" => crate::converters::apng::convert_apng(&input_path, &output_path, &to_format),
            "cur" => crate::converters::cur::convert_cur(&input_path, &output_path, &to_format),
            "exr" => crate::converters::exr::convert_exr(&input_path, &output_path, &to_format),
            "svg" => crate::converters::svg::convert_svg(&input_path, &output_path, &to_format),
            "pdf" => crate::converters::pdf::convert_pdf(&input_path, &output_path, &to_format),
            "psd" | "psb" => crate::converters::psd::convert_psd(&input_path, &output_path, &to_format),
            "fits" => crate::converters::fits::convert_fits(&input_path, &output_path, &to_format),
            "dcm" => crate::converters::dcm::convert_dcm(&input_path, &output_path, &to_format),
            "pcx" => crate::converters::pcx::convert_pcx(&input_path, &output_path, &to_format),
            // Video formats
            "mp4" | "m4v" | "mkv" | "mov" | "webm" | "avi" | "flv" | "mpg" | "mpeg" | "ts" | "m2ts" | "mts" | "ogv" | "ogg" => {
                crate::converters::video::transcoder::convert_video(&input_path, &output_path, &to_format)
            }
            _ => Err(ConversionError::UnsupportedFormat(from_format.clone())),
        }
    };

    match result {
        Ok(_) => Ok(ConversionResult {
            id: uuid::Uuid::new_v4().to_string(),
            success: true,
            message: format!("Successfully converted {} to {}", from_format, to_format),
            output_path: Some(output_path),
        }),
        Err(e) => Ok(ConversionResult {
            id: uuid::Uuid::new_v4().to_string(),
            success: false,
            message: format!("Conversion error: {}", e),
            output_path: None,
        }),
    }
}
