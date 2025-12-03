// Lua parser - extracts WezTerm configuration values from Lua files
//
// WezTerm configs can be complex Lua scripts. This parser uses regex patterns
// to extract common configuration values. It's not a full Lua parser, but
// handles the most common config patterns.

use crate::models::{
    AppearanceConfig, CloseConfirmation, ColorScheme, CursorConfig, CursorStyle,
    EaseFunction, FontConfig, FontWeight, FreetypeTarget, FrontEnd, GPUConfig, PowerPreference,
    TabBarColors, WindowConfig, WindowDecorations, HSB, Padding,
};
use regex::Regex;
use std::path::Path;

/// Result of parsing a WezTerm config file
#[derive(Debug)]
pub struct ParseResult {
    pub config: AppearanceConfig,
    pub raw_content: String,
    pub parse_errors: Vec<String>,
}

/// Parse a WezTerm Lua config file and extract configuration values
pub fn parse_wezterm_config(path: &Path) -> Result<ParseResult, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    parse_lua_content(&content)
}

/// Parse Lua content string and extract configuration values
pub fn parse_lua_content(content: &str) -> Result<ParseResult, String> {
    let mut config = AppearanceConfig::default();
    let mut errors = Vec::new();

    // Parse each section
    if let Err(e) = parse_colors(content, &mut config.colors) {
        errors.push(format!("Colors: {}", e));
    }
    if let Err(e) = parse_fonts(content, &mut config.fonts) {
        errors.push(format!("Fonts: {}", e));
    }
    if let Err(e) = parse_window(content, &mut config.window) {
        errors.push(format!("Window: {}", e));
    }
    if let Err(e) = parse_cursor(content, &mut config.cursor) {
        errors.push(format!("Cursor: {}", e));
    }
    if let Err(e) = parse_gpu(content, &mut config.gpu) {
        errors.push(format!("GPU: {}", e));
    }

    Ok(ParseResult {
        config,
        raw_content: content.to_string(),
        parse_errors: errors,
    })
}

// ============================================================================
// Color Parsing
// ============================================================================

fn parse_colors(content: &str, colors: &mut ColorScheme) -> Result<(), String> {
    // Parse simple color assignments like: config.foreground = "#cdd6f4"
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?foreground\s*=\s*["']([^"']+)["']"#) {
        colors.foreground = val;
    }
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?background\s*=\s*["']([^"']+)["']"#) {
        colors.background = val;
    }
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?cursor_bg\s*=\s*["']([^"']+)["']"#) {
        colors.cursor_bg = val;
    }
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?cursor_border\s*=\s*["']([^"']+)["']"#) {
        colors.cursor_border = val;
    }
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?cursor_fg\s*=\s*["']([^"']+)["']"#) {
        colors.cursor_fg = val;
    }
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?selection_bg\s*=\s*["']([^"']+)["']"#) {
        colors.selection_bg = val;
    }
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?selection_fg\s*=\s*["']([^"']+)["']"#) {
        colors.selection_fg = val;
    }

    // Parse ANSI colors array
    if let Some(ansi) = extract_color_array(content, "ansi") {
        if ansi.len() == 8 {
            colors.ansi = ansi.try_into().unwrap_or(colors.ansi.clone());
        }
    }

    // Parse bright colors array
    if let Some(brights) = extract_color_array(content, "brights") {
        if brights.len() == 8 {
            colors.brights = brights.try_into().unwrap_or(colors.brights.clone());
        }
    }

    // Parse tab bar colors from colors block
    parse_tab_bar_colors(content, &mut colors.tab_bar);

    Ok(())
}

fn parse_tab_bar_colors(content: &str, tab_bar: &mut TabBarColors) {
    // Look for tab_bar block within colors
    if let Some(val) = extract_nested_string(content, &["colors", "tab_bar", "background"]) {
        tab_bar.background = val;
    }

    // Parse active_tab
    if let Some(bg) = extract_nested_string(content, &["colors", "tab_bar", "active_tab", "bg_color"]) {
        tab_bar.active_tab.bg_color = bg;
    }
    if let Some(fg) = extract_nested_string(content, &["colors", "tab_bar", "active_tab", "fg_color"]) {
        tab_bar.active_tab.fg_color = fg;
    }

    // Parse inactive_tab
    if let Some(bg) = extract_nested_string(content, &["colors", "tab_bar", "inactive_tab", "bg_color"]) {
        tab_bar.inactive_tab.bg_color = bg;
    }
    if let Some(fg) = extract_nested_string(content, &["colors", "tab_bar", "inactive_tab", "fg_color"]) {
        tab_bar.inactive_tab.fg_color = fg;
    }
}

fn extract_color_array(content: &str, name: &str) -> Option<Vec<String>> {
    // Match patterns like: ansi = { "#000", "#111", ... }
    let pattern = format!(r#"{}\s*=\s*\{{\s*([^}}]+)\s*\}}"#, name);
    let re = Regex::new(&pattern).ok()?;
    let caps = re.captures(content)?;
    let array_content = caps.get(1)?.as_str();

    // Extract quoted strings from array
    let string_re = Regex::new(r#"["']([^"']+)["']"#).ok()?;
    let colors: Vec<String> = string_re
        .captures_iter(array_content)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect();

    if colors.is_empty() {
        None
    } else {
        Some(colors)
    }
}

// ============================================================================
// Font Parsing
// ============================================================================

fn parse_fonts(content: &str, fonts: &mut FontConfig) -> Result<(), String> {
    // Parse font_size = 12
    if let Some(val) = extract_number_value(content, r#"(?:config\.)?font_size\s*=\s*(\d+(?:\.\d+)?)"#) {
        fonts.size = val;
    }

    // Parse font = wezterm.font("JetBrainsMono Nerd Font")
    if let Some(val) = extract_string_value(content, r#"wezterm\.font\s*\(\s*["']([^"']+)["']"#) {
        fonts.family = val;
    }
    // Also try: font = wezterm.font { family = "..." }
    if let Some(val) = extract_string_value(content, r#"wezterm\.font\s*\{\s*family\s*=\s*["']([^"']+)["']"#) {
        fonts.family = val;
    }

    // Parse font weight
    if let Some(val) = extract_string_value(content, r#"(?:weight|font_weight)\s*=\s*["']([^"']+)["']"#) {
        fonts.weight = parse_font_weight(&val);
    }

    // Parse freetype targets
    if let Some(val) = extract_string_value(content, r#"freetype_load_target\s*=\s*["']([^"']+)["']"#) {
        fonts.freetype_load_target = parse_freetype_target(&val);
    }
    if let Some(val) = extract_string_value(content, r#"freetype_render_target\s*=\s*["']([^"']+)["']"#) {
        fonts.freetype_render_target = parse_freetype_target(&val);
    }

    Ok(())
}

fn parse_font_weight(s: &str) -> Option<FontWeight> {
    match s.to_lowercase().as_str() {
        "thin" => Some(FontWeight::Thin),
        "extralight" | "extra-light" => Some(FontWeight::ExtraLight),
        "light" => Some(FontWeight::Light),
        "regular" | "normal" => Some(FontWeight::Regular),
        "medium" => Some(FontWeight::Medium),
        "demibold" | "demi-bold" | "semibold" | "semi-bold" => Some(FontWeight::DemiBold),
        "bold" => Some(FontWeight::Bold),
        "extrabold" | "extra-bold" => Some(FontWeight::ExtraBold),
        "black" | "heavy" => Some(FontWeight::Black),
        _ => None,
    }
}

fn parse_freetype_target(s: &str) -> Option<FreetypeTarget> {
    match s.to_lowercase().as_str() {
        "normal" => Some(FreetypeTarget::Normal),
        "light" => Some(FreetypeTarget::Light),
        "mono" => Some(FreetypeTarget::Mono),
        "horizontallcd" | "horizontal_lcd" => Some(FreetypeTarget::HorizontalLcd),
        _ => None,
    }
}

// ============================================================================
// Window Parsing
// ============================================================================

fn parse_window(content: &str, window: &mut WindowConfig) -> Result<(), String> {
    // Parse window_background_opacity
    if let Some(val) = extract_number_value(content, r#"(?:config\.)?window_background_opacity\s*=\s*(\d+(?:\.\d+)?)"#) {
        window.window_background_opacity = val;
    }

    // Parse window_decorations
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?window_decorations\s*=\s*["']([^"']+)["']"#) {
        window.window_decorations = parse_window_decorations(&val);
    }

    // Parse tab bar settings
    if let Some(val) = extract_bool_value(content, r#"(?:config\.)?enable_tab_bar\s*=\s*(true|false)"#) {
        window.enable_tab_bar = val;
    }
    if let Some(val) = extract_bool_value(content, r#"(?:config\.)?hide_tab_bar_if_only_one_tab\s*=\s*(true|false)"#) {
        window.hide_tab_bar_if_only_one_tab = val;
    }
    if let Some(val) = extract_bool_value(content, r#"(?:config\.)?use_fancy_tab_bar\s*=\s*(true|false)"#) {
        window.use_fancy_tab_bar = val;
    }
    if let Some(val) = extract_number_value(content, r#"(?:config\.)?tab_max_width\s*=\s*(\d+)"#) {
        window.tab_max_width = val as u32;
    }
    if let Some(val) = extract_bool_value(content, r#"(?:config\.)?show_tab_index_in_tab_bar\s*=\s*(true|false)"#) {
        window.show_tab_index_in_tab_bar = val;
    }

    // Parse window_padding
    parse_window_padding(content, &mut window.window_padding);

    // Parse inactive_pane_hsb
    parse_hsb(content, &mut window.inactive_pane_hsb);

    // Parse window_close_confirmation
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?window_close_confirmation\s*=\s*["']([^"']+)["']"#) {
        window.window_close_confirmation = parse_close_confirmation(&val);
    }

    Ok(())
}

fn parse_window_padding(content: &str, padding: &mut Padding) {
    // Look for window_padding = { left = 0, right = 0, top = 10, bottom = 7.5 }
    if let Some(val) = extract_nested_number(content, &["window_padding", "left"]) {
        padding.left = val;
    }
    if let Some(val) = extract_nested_number(content, &["window_padding", "right"]) {
        padding.right = val;
    }
    if let Some(val) = extract_nested_number(content, &["window_padding", "top"]) {
        padding.top = val;
    }
    if let Some(val) = extract_nested_number(content, &["window_padding", "bottom"]) {
        padding.bottom = val;
    }
}

fn parse_hsb(content: &str, hsb: &mut HSB) {
    if let Some(val) = extract_nested_number(content, &["inactive_pane_hsb", "hue"]) {
        hsb.hue = val;
    }
    if let Some(val) = extract_nested_number(content, &["inactive_pane_hsb", "saturation"]) {
        hsb.saturation = val;
    }
    if let Some(val) = extract_nested_number(content, &["inactive_pane_hsb", "brightness"]) {
        hsb.brightness = val;
    }
}

fn parse_window_decorations(s: &str) -> WindowDecorations {
    match s.to_uppercase().as_str() {
        "FULL" => WindowDecorations::Full,
        "RESIZE" => WindowDecorations::Resize,
        "NONE" => WindowDecorations::None,
        "TITLE" => WindowDecorations::Title,
        "INTEGRATED_BUTTONS|RESIZE" => WindowDecorations::IntegratedButtonsResize,
        _ => WindowDecorations::Full,
    }
}

fn parse_close_confirmation(s: &str) -> CloseConfirmation {
    match s {
        "AlwaysPrompt" => CloseConfirmation::AlwaysPrompt,
        "NeverPrompt" => CloseConfirmation::NeverPrompt,
        _ => CloseConfirmation::NeverPrompt,
    }
}

// ============================================================================
// Cursor Parsing
// ============================================================================

fn parse_cursor(content: &str, cursor: &mut CursorConfig) -> Result<(), String> {
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?default_cursor_style\s*=\s*["']([^"']+)["']"#) {
        cursor.default_cursor_style = parse_cursor_style(&val);
    }
    if let Some(val) = extract_number_value(content, r#"(?:config\.)?cursor_blink_rate\s*=\s*(\d+)"#) {
        cursor.cursor_blink_rate = val as u32;
    }
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?cursor_blink_ease_in\s*=\s*["']([^"']+)["']"#) {
        cursor.cursor_blink_ease_in = parse_ease_function(&val);
    }
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?cursor_blink_ease_out\s*=\s*["']([^"']+)["']"#) {
        cursor.cursor_blink_ease_out = parse_ease_function(&val);
    }
    if let Some(val) = extract_number_value(content, r#"(?:config\.)?animation_fps\s*=\s*(\d+)"#) {
        cursor.animation_fps = val as u32;
    }

    Ok(())
}

fn parse_cursor_style(s: &str) -> CursorStyle {
    match s {
        "SteadyBlock" => CursorStyle::SteadyBlock,
        "BlinkingBlock" => CursorStyle::BlinkingBlock,
        "SteadyUnderline" => CursorStyle::SteadyUnderline,
        "BlinkingUnderline" => CursorStyle::BlinkingUnderline,
        "SteadyBar" => CursorStyle::SteadyBar,
        "BlinkingBar" => CursorStyle::BlinkingBar,
        _ => CursorStyle::BlinkingBlock,
    }
}

fn parse_ease_function(s: &str) -> EaseFunction {
    match s {
        "Linear" => EaseFunction::Linear,
        "EaseIn" => EaseFunction::EaseIn,
        "EaseOut" => EaseFunction::EaseOut,
        "EaseInOut" => EaseFunction::EaseInOut,
        "Constant" => EaseFunction::Constant,
        _ => EaseFunction::EaseOut,
    }
}

// ============================================================================
// GPU Parsing
// ============================================================================

fn parse_gpu(content: &str, gpu: &mut GPUConfig) -> Result<(), String> {
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?front_end\s*=\s*["']([^"']+)["']"#) {
        gpu.front_end = parse_front_end(&val);
    }
    if let Some(val) = extract_string_value(content, r#"(?:config\.)?webgpu_power_preference\s*=\s*["']([^"']+)["']"#) {
        gpu.webgpu_power_preference = parse_power_preference(&val);
    }
    if let Some(val) = extract_number_value(content, r#"(?:config\.)?max_fps\s*=\s*(\d+)"#) {
        gpu.max_fps = val as u32;
    }

    Ok(())
}

fn parse_front_end(s: &str) -> FrontEnd {
    match s {
        "WebGpu" => FrontEnd::WebGpu,
        "OpenGL" => FrontEnd::OpenGL,
        "Software" => FrontEnd::Software,
        _ => FrontEnd::WebGpu,
    }
}

fn parse_power_preference(s: &str) -> PowerPreference {
    match s {
        "LowPower" => PowerPreference::LowPower,
        "HighPerformance" => PowerPreference::HighPerformance,
        _ => PowerPreference::HighPerformance,
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn extract_string_value(content: &str, pattern: &str) -> Option<String> {
    let re = Regex::new(pattern).ok()?;
    let caps = re.captures(content)?;
    caps.get(1).map(|m| m.as_str().to_string())
}

fn extract_number_value(content: &str, pattern: &str) -> Option<f32> {
    let re = Regex::new(pattern).ok()?;
    let caps = re.captures(content)?;
    caps.get(1)?.as_str().parse().ok()
}

fn extract_bool_value(content: &str, pattern: &str) -> Option<bool> {
    let re = Regex::new(pattern).ok()?;
    let caps = re.captures(content)?;
    match caps.get(1)?.as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn extract_nested_string(content: &str, keys: &[&str]) -> Option<String> {
    // Build a pattern for nested table access
    // e.g., colors.tab_bar.background = "#000"
    let key_path = keys.join(r#"\s*[\.\[\]"']*\s*"#);
    let pattern = format!(r#"{}\s*=\s*["']([^"']+)["']"#, key_path);
    extract_string_value(content, &pattern)
}

fn extract_nested_number(content: &str, keys: &[&str]) -> Option<f32> {
    let key_path = keys.join(r#"\s*[\.\[\]"']*\s*"#);
    let pattern = format!(r#"{}\s*=\s*(\d+(?:\.\d+)?)"#, key_path);
    extract_number_value(content, &pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_config() {
        let content = r#"
            local wezterm = require 'wezterm'
            local config = wezterm.config_builder()
            
            config.font_size = 14
            config.font = wezterm.font("JetBrains Mono")
            config.window_background_opacity = 0.95
            config.enable_tab_bar = false
            
            return config
        "#;

        let result = parse_lua_content(content).unwrap();
        assert_eq!(result.config.fonts.size, 14.0);
        assert_eq!(result.config.fonts.family, "JetBrains Mono");
        assert_eq!(result.config.window.window_background_opacity, 0.95);
        assert!(!result.config.window.enable_tab_bar);
    }

    #[test]
    fn test_parse_color_array() {
        let content = r##"
            ansi = { "#000000", "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#ff00ff", "#00ffff", "#ffffff" }
        "##;

        let colors = extract_color_array(content, "ansi").unwrap();
        assert_eq!(colors.len(), 8);
        assert_eq!(colors[0], "#000000");
        assert_eq!(colors[1], "#ff0000");
    }

    #[test]
    fn test_parse_cursor_style() {
        assert!(matches!(parse_cursor_style("BlinkingBlock"), CursorStyle::BlinkingBlock));
        assert!(matches!(parse_cursor_style("SteadyBar"), CursorStyle::SteadyBar));
    }
}
