mod models;
pub mod converters;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::convert_image,
            commands::convert_from_path,
            commands::get_supported_formats,
            commands::is_supported_format,
            commands::select_folder,
            commands::get_downloads_folder,
            commands::pick_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
