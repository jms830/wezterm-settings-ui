// WezTerm Settings GUI - Tauri backend

pub mod commands;
pub mod config;
pub mod lua;
pub mod models;

use commands::{
    ensure_config_exists, get_builtin_color_schemes, get_config_path, get_default_config,
    get_system_info, list_backdrop_images, load_wezterm_config, save_wezterm_config,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // System commands
            get_config_path,
            get_system_info,
            ensure_config_exists,
            // Config commands
            load_wezterm_config,
            save_wezterm_config,
            get_default_config,
            // Color scheme commands
            get_builtin_color_schemes,
            // Backdrop commands
            list_backdrop_images,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
