// System commands - config path detection and system information

use crate::config::{get_wezterm_config_dir, get_wezterm_config_file};
use crate::models::{Platform, SystemInfo};

/// Check if WezTerm is installed by looking for the executable
fn is_wezterm_installed() -> bool {
    // Check if wezterm is in PATH
    if let Ok(output) = std::process::Command::new("which")
        .arg("wezterm")
        .output()
    {
        if output.status.success() {
            return true;
        }
    }

    // Fallback for Windows
    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("where")
            .arg("wezterm")
            .output()
        {
            if output.status.success() {
                return true;
            }
        }
    }

    false
}

/// Get the current platform
fn get_platform() -> Platform {
    #[cfg(target_os = "windows")]
    return Platform::Windows;

    #[cfg(target_os = "macos")]
    return Platform::Macos;

    #[cfg(target_os = "linux")]
    return Platform::Linux;

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return Platform::Linux; // Default fallback
}

/// Tauri command: Get the WezTerm config directory path
#[tauri::command]
pub fn get_config_path() -> Result<String, String> {
    get_wezterm_config_dir()
        .map(|p| p.to_string_lossy().to_string())
}

/// Tauri command: Ensures the config file exists, creating a default one if not.
/// Returns the path to the config file.
#[tauri::command]
pub fn ensure_config_exists() -> Result<String, String> {
    let config_file = get_wezterm_config_file()?;

    if !config_file.exists() {
        // Create default config
        let default_config = r#"-- WezTerm configuration
local wezterm = require 'wezterm'
local config = wezterm.config_builder()

return config
"#;
        if let Some(parent) = config_file.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| e.to_string())?;
        }
        std::fs::write(&config_file, default_config)
            .map_err(|e| e.to_string())?;
    }

    Ok(config_file.to_string_lossy().to_string())
}


/// Tauri command: Get system information
#[tauri::command]
pub fn get_system_info() -> Result<SystemInfo, String> {
    let config_dir = get_wezterm_config_dir()?;

    let config_exists = config_dir.exists() && config_dir.join("wezterm.lua").exists();

    Ok(SystemInfo {
        platform: get_platform(),
        config_dir: config_dir.to_string_lossy().to_string(),
        config_exists,
        wezterm_installed: is_wezterm_installed(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wezterm_config_dir() {
        // Should return Some path (won't necessarily exist in test env)
        let result = get_wezterm_config_dir();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_platform() {
        let platform = get_platform();
        // Should match the compile target
        #[cfg(target_os = "linux")]
        assert!(matches!(platform, Platform::Linux));

        #[cfg(target_os = "macos")]
        assert!(matches!(platform, Platform::Macos));

        #[cfg(target_os = "windows")]
        assert!(matches!(platform, Platform::Windows));
    }
}
