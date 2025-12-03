// Config loading and saving

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::lua::parser::parse_wezterm_config;

/// Finds the WezTerm configuration directory, matching WezTerm's own search order.
/// Returns the first existing directory, or creates the default if none exist.
pub fn get_wezterm_config_dir() -> Result<PathBuf, String> {
    // Check for explicit override via environment
    if let Ok(config_file) = std::env::var("WEZTERM_CONFIG_FILE") {
        let path = PathBuf::from(config_file);
        if let Some(parent) = path.parent() {
            if parent.exists() {
                return Ok(parent.to_path_buf());
            }
        }
    }

    // Get potential config directories in WezTerm's search order
    let candidates = get_config_candidates();

    // Return first existing directory
    for candidate in &candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    // None exist - create the default one
    let default_dir = get_default_config_dir()?;
    fs::create_dir_all(&default_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    Ok(default_dir)
}

/// Returns candidate config directories in WezTerm's search order
fn get_config_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    // 1. XDG_CONFIG_HOME/wezterm (Linux/macOS)
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        candidates.push(PathBuf::from(xdg).join("wezterm"));
    }

    // 2. ~/.config/wezterm (standard location)
    if let Some(home) = dirs::home_dir() {
        candidates.push(home.join(".config").join("wezterm"));
    }

    // 3. Windows alternative: %USERPROFILE%/.wezterm
    #[cfg(windows)]
    if let Some(home) = dirs::home_dir() {
        candidates.push(home.join(".wezterm"));
    }

    candidates
}

/// Returns the default config directory to create
fn get_default_config_dir() -> Result<PathBuf, String> {
    // Prefer XDG on Linux
    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            return Ok(PathBuf::from(xdg).join("wezterm"));
        }
    }

    // Fall back to ~/.config/wezterm
    dirs::home_dir()
        .map(|h| h.join(".config").join("wezterm"))
        .ok_or_else(|| "Could not determine home directory".to_string())
}

/// Gets the path to the main wezterm.lua config file
pub fn get_wezterm_config_file() -> Result<PathBuf, String> {
    // Check WEZTERM_CONFIG_FILE first
    if let Ok(config_file) = std::env::var("WEZTERM_CONFIG_FILE") {
        let path = PathBuf::from(&config_file);
        if path.exists() {
            return Ok(path);
        }
    }

    let config_dir = get_wezterm_config_dir()?;
    let config_file = config_dir.join("wezterm.lua");

    // Also check for ~/.wezterm.lua (WezTerm's simple config location)
    if !config_file.exists() {
        if let Some(home) = dirs::home_dir() {
            let simple_config = home.join(".wezterm.lua");
            if simple_config.exists() {
                return Ok(simple_config);
            }
        }
    }

    Ok(config_file)
}

use crate::models::AppearanceConfig;

/// Load config from disk, parsing existing wezterm.lua if it exists
pub fn load_config(_config_dir: Option<&str>) -> Result<AppearanceConfig> {
    // Try to find and parse existing config
    match get_wezterm_config_file() {
        Ok(config_path) if config_path.exists() => {
            match parse_wezterm_config(&config_path) {
                Ok(result) => {
                    if !result.parse_errors.is_empty() {
                        // Log parse errors but still return partial config
                        for err in &result.parse_errors {
                            eprintln!("Config parse warning: {}", err);
                        }
                    }
                    Ok(result.config)
                }
                Err(e) => {
                    eprintln!("Failed to parse config, using defaults: {}", e);
                    Ok(AppearanceConfig::default())
                }
            }
        }
        _ => {
            // No config file exists, return defaults
            Ok(AppearanceConfig::default())
        }
    }
}

/// Load config and return additional metadata about the parse
pub fn load_config_with_metadata(_config_dir: Option<&str>) -> Result<ConfigLoadResult> {
    match get_wezterm_config_file() {
        Ok(config_path) => {
            let config_exists = config_path.exists();
            if config_exists {
                match parse_wezterm_config(&config_path) {
                    Ok(result) => Ok(ConfigLoadResult {
                        config: result.config,
                        config_path: config_path.to_string_lossy().to_string(),
                        config_exists: true,
                        raw_content: Some(result.raw_content),
                        parse_errors: result.parse_errors,
                    }),
                    Err(e) => Ok(ConfigLoadResult {
                        config: AppearanceConfig::default(),
                        config_path: config_path.to_string_lossy().to_string(),
                        config_exists: true,
                        raw_content: None,
                        parse_errors: vec![e],
                    }),
                }
            } else {
                Ok(ConfigLoadResult {
                    config: AppearanceConfig::default(),
                    config_path: config_path.to_string_lossy().to_string(),
                    config_exists: false,
                    raw_content: None,
                    parse_errors: vec![],
                })
            }
        }
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

/// Result of loading config with metadata
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConfigLoadResult {
    pub config: AppearanceConfig,
    pub config_path: String,
    pub config_exists: bool,
    pub raw_content: Option<String>,
    pub parse_errors: Vec<String>,
}

/// Save config to disk
pub fn save_config(config: &AppearanceConfig, _config_dir: Option<&str>) -> Result<()> {
    let dir = get_wezterm_config_dir().map_err(|e| anyhow::anyhow!(e))?;

    // Ensure directories exist
    std::fs::create_dir_all(&dir).context("Failed to create config directory")?;

    // TODO: Implement Lua generation to write config files.
    // For now, this function is a placeholder.
    println!("Saving config to: {:?}", dir);
    println!("{:#?}", config);

    Ok(())
}
