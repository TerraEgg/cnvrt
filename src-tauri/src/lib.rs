mod models;
pub mod converters;
mod commands;

use std::sync::Mutex;
use tauri::{State, Manager, Listener};

pub struct SharedState {
    pub initial_file: Mutex<Option<String>>,
}

#[tauri::command]
fn get_initial_file(state: State<SharedState>) -> Option<String> {
    state.initial_file.lock().unwrap().take()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let shared_state = SharedState {
        initial_file: Mutex::new(None),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(shared_state)
        .invoke_handler(tauri::generate_handler![
            commands::convert_image,
            commands::convert_from_path,
            commands::get_supported_formats,
            commands::is_supported_format,
            commands::select_folder,
            commands::get_downloads_folder,
            commands::pick_files,
            get_initial_file,
        ])
        .setup(|app| {
            let state = app.state::<SharedState>();
            let mut args = std::env::args();
            
            while let Some(arg) = args.next() {
                if arg == "--file" {
                    if let Some(file_path) = args.next() {
                        *state.initial_file.lock().unwrap() = Some(file_path);
                    }
                }
            }

            #[cfg(any(windows, target_os = "linux"))]
            {
                app.listen("tauri://deep-link", |_event| {});
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

