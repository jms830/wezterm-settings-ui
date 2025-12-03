// Rust config structs for WezTerm configuration
// Mirrors TypeScript interfaces in src/types/config.ts

use serde::{Deserialize, Serialize};

// ============================================================================
// Color Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabColors {
    pub bg_color: String,
    pub fg_color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabBarColors {
    pub background: String,
    pub active_tab: TabColors,
    pub inactive_tab: TabColors,
    pub inactive_tab_hover: TabColors,
    pub new_tab: TabColors,
    pub new_tab_hover: TabColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    // Core colors
    pub foreground: String,
    pub background: String,
    pub cursor_bg: String,
    pub cursor_border: String,
    pub cursor_fg: String,
    pub selection_bg: String,
    pub selection_fg: String,

    // ANSI palette (16 colors split into 8 + 8)
    pub ansi: [String; 8],
    pub brights: [String; 8],

    // Tab bar colors
    pub tab_bar: TabBarColors,

    // Optional colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_bell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollbar_thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<String>,
}

// ============================================================================
// Font Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    DemiBold,
    Bold,
    ExtraBold,
    Black,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FreetypeTarget {
    Normal,
    Light,
    Mono,
    HorizontalLcd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub family: String,
    pub size: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<FontWeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freetype_load_target: Option<FreetypeTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freetype_render_target: Option<FreetypeTarget>,
}

// ============================================================================
// Window Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HSB {
    pub hue: f32,
    pub saturation: f32,
    pub brightness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowDecorations {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "RESIZE")]
    Resize,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "TITLE")]
    Title,
    #[serde(rename = "INTEGRATED_BUTTONS|RESIZE")]
    IntegratedButtonsResize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloseConfirmation {
    AlwaysPrompt,
    NeverPrompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub window_padding: Padding,
    pub window_background_opacity: f32,
    pub window_decorations: WindowDecorations,
    pub enable_tab_bar: bool,
    pub hide_tab_bar_if_only_one_tab: bool,
    pub use_fancy_tab_bar: bool,
    pub tab_max_width: u32,
    pub show_tab_index_in_tab_bar: bool,
    pub inactive_pane_hsb: HSB,
    pub window_close_confirmation: CloseConfirmation,
}

// ============================================================================
// Cursor Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CursorStyle {
    SteadyBlock,
    BlinkingBlock,
    SteadyUnderline,
    BlinkingUnderline,
    SteadyBar,
    BlinkingBar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EaseFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Constant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorConfig {
    pub default_cursor_style: CursorStyle,
    pub cursor_blink_rate: u32,
    pub cursor_blink_ease_in: EaseFunction,
    pub cursor_blink_ease_out: EaseFunction,
    pub animation_fps: u32,
}

// ============================================================================
// Backdrop Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackdropConfig {
    pub enabled: bool,
    pub images_dir: String,
    pub images: Vec<String>,
    pub current_index: usize,
    pub focus_color: String,
    pub overlay_opacity: f32,
    pub random_on_start: bool,
}

// ============================================================================
// Command Palette Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPaletteConfig {
    pub fg_color: String,
    pub bg_color: String,
    pub font_size: f32,
}

// ============================================================================
// Visual Bell Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualBellConfig {
    pub fade_in_duration_ms: u32,
    pub fade_out_duration_ms: u32,
    pub fade_in_function: EaseFunction,
    pub fade_out_function: EaseFunction,
    pub target: String, // VisualBellTarget enum as string
}

// ============================================================================
// General Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExitBehavior {
    Close,
    CloseOnCleanExit,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudibleBell {
    SystemBeep,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub automatically_reload_config: bool,
    pub scrollback_lines: u32,
    pub initial_rows: u32,
    pub initial_cols: u32,
    pub exit_behavior: ExitBehavior,
    pub audible_bell: AudibleBell,
    pub enable_scroll_bar: bool,
    pub switch_to_last_active_tab_when_closing_tab: bool,
    pub adjust_window_size_when_changing_font_size: bool,
}

// ============================================================================
// Key Bindings / Custom Commands Types
// ============================================================================

/// A single keybinding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub enabled: bool,
    pub key: String,
    pub mods: String,  // e.g., "CTRL|SHIFT", "ALT", "SUPER", "NONE"
}

impl KeyBinding {
    pub fn new(key: &str, mods: &str) -> Self {
        Self {
            enabled: true,
            key: key.to_string(),
            mods: mods.to_string(),
        }
    }
    
    pub fn disabled(key: &str, mods: &str) -> Self {
        Self {
            enabled: false,
            key: key.to_string(),
            mods: mods.to_string(),
        }
    }
}

/// Misc/utility keybindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscBindings {
    pub copy_mode: KeyBinding,           // F1 - ActivateCopyMode
    pub command_palette: KeyBinding,     // F2 - ActivateCommandPalette
    pub command_palette_alt: KeyBinding, // Ctrl+Shift+P - ActivateCommandPalette
    pub show_launcher: KeyBinding,       // F3 - ShowLauncher
    pub show_tab_launcher: KeyBinding,   // F4 - ShowLauncherArgs FUZZY|TABS
    pub show_workspace_launcher: KeyBinding, // F5 - ShowLauncherArgs FUZZY|WORKSPACES
    pub toggle_fullscreen: KeyBinding,   // F11 - ToggleFullScreen
    pub show_debug_overlay: KeyBinding,  // F12 - ShowDebugOverlay
    pub search: KeyBinding,              // Super+f - Search
    pub quick_select_url: KeyBinding,    // Super_Rev+u - QuickSelectArgs for URLs
}

/// Copy/paste keybindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyPasteBindings {
    pub copy: KeyBinding,          // Ctrl+Shift+c - CopyTo Clipboard
    pub paste: KeyBinding,         // Ctrl+Shift+v - PasteFrom Clipboard
    pub copy_simple: KeyBinding,   // Ctrl+c - CopyTo Clipboard (Windows style)
    pub paste_simple: KeyBinding,  // Ctrl+v - PasteFrom Clipboard (Windows style)
}

/// Tab-related keybindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabBindings {
    pub spawn_tab: KeyBinding,           // Super+t - SpawnTab DefaultDomain
    pub spawn_tab_wsl: KeyBinding,       // Super_Rev+t - SpawnTab WSL
    pub close_tab: KeyBinding,           // Super_Rev+w - CloseCurrentTab
    pub next_tab: KeyBinding,            // Super+] - ActivateTabRelative(1)
    pub prev_tab: KeyBinding,            // Super+[ - ActivateTabRelative(-1)
    pub move_tab_forward: KeyBinding,    // Super_Rev+] - MoveTabRelative(1)
    pub move_tab_back: KeyBinding,       // Super_Rev+[ - MoveTabRelative(-1)
    pub rename_tab: KeyBinding,          // Super_Rev+r - PromptInputLine rename
    pub manual_update_title: KeyBinding, // Super+0 - EmitEvent tabs.manual-update-tab-title
    pub reset_title: KeyBinding,         // Super_Rev+0 - EmitEvent tabs.reset-tab-title
    pub toggle_tab_bar: KeyBinding,      // Super+9 - EmitEvent tabs.toggle-tab-bar
}

/// Window-related keybindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowBindings {
    pub spawn_window: KeyBinding,   // Super+n - SpawnWindow
    pub shrink_window: KeyBinding,  // Super+- - Shrink window by 50px
    pub grow_window: KeyBinding,    // Super+= - Grow window by 50px
    pub maximize_window: KeyBinding, // Super_Rev+Enter - Maximize
}

/// Pane-related keybindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaneBindings {
    pub split_vertical: KeyBinding,   // Super+\ - SplitVertical
    pub split_horizontal: KeyBinding, // Super_Rev+\ - SplitHorizontal
    pub toggle_zoom: KeyBinding,      // Super+Enter - TogglePaneZoomState
    pub close_pane: KeyBinding,       // Super+w - CloseCurrentPane
    pub nav_up: KeyBinding,           // Super_Rev+k - ActivatePaneDirection Up
    pub nav_down: KeyBinding,         // Super_Rev+j - ActivatePaneDirection Down
    pub nav_left: KeyBinding,         // Super_Rev+h - ActivatePaneDirection Left
    pub nav_right: KeyBinding,        // Super_Rev+l - ActivatePaneDirection Right
    pub swap_pane: KeyBinding,        // Super_Rev+p - PaneSelect swap
    pub scroll_up: KeyBinding,        // Super+u - ScrollByLine(-5)
    pub scroll_down: KeyBinding,      // Super+d - ScrollByLine(5)
    pub page_up: KeyBinding,          // PageUp - ScrollByPage(-0.75)
    pub page_down: KeyBinding,        // PageDown - ScrollByPage(0.75)
}

/// Backdrop/background keybindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackdropBindings {
    pub random: KeyBinding,       // Super+/ - Random backdrop
    pub cycle_back: KeyBinding,   // Super+, - Cycle backdrop back
    pub cycle_forward: KeyBinding, // Super+. - Cycle backdrop forward
    pub select: KeyBinding,       // Super_Rev+/ - InputSelector for backdrop
    pub toggle_focus: KeyBinding, // Super+b - Toggle focus mode
}

/// Cursor movement keybindings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorBindings {
    pub home: KeyBinding,      // Super+LeftArrow - Send Home
    pub end: KeyBinding,       // Super+RightArrow - Send End
    pub delete_line: KeyBinding, // Super+Backspace - Delete line
    pub newline: KeyBinding,   // Shift+Enter - Send newline without execute
}

/// Key table bindings (Leader key activated modes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyTableBindings {
    pub resize_font_mode: KeyBinding, // Leader+f - Activate resize_font key table
    pub resize_pane_mode: KeyBinding, // Leader+p - Activate resize_pane key table
}

/// Leader key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderKeyConfig {
    pub enabled: bool,
    pub key: String,
    pub mods: String,
    pub timeout_ms: u32,
}

/// Mouse binding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseBindings {
    pub ctrl_click_open_link: bool,     // Ctrl+Click opens link
    pub right_click_command_palette: bool, // Right-click opens command palette
}

/// Custom commands for command palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomCommands {
    pub settings_tui: bool,  // Add Settings-TUI to command palette
    pub rename_tab: bool,    // Add Rename Tab to command palette (redundant with keybinding but appears in palette)
}

/// Complete keybindings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindingsConfig {
    pub disable_defaults: bool,
    pub leader: LeaderKeyConfig,
    pub misc: MiscBindings,
    pub copy_paste: CopyPasteBindings,
    pub tabs: TabBindings,
    pub windows: WindowBindings,
    pub panes: PaneBindings,
    pub backdrops: BackdropBindings,
    pub cursor: CursorBindings,
    pub key_tables: KeyTableBindings,
    pub mouse: MouseBindings,
    pub custom_commands: CustomCommands,
}

// ============================================================================
// GPU Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrontEnd {
    WebGpu,
    OpenGL,
    Software,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PowerPreference {
    LowPower,
    HighPerformance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUConfig {
    pub front_end: FrontEnd,
    pub webgpu_power_preference: PowerPreference,
    pub max_fps: u32,
}

// ============================================================================
// Composite Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    /// Use a built-in WezTerm color scheme by name (e.g., "Catppuccin Mocha")
    /// When set, overrides the custom `colors` settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_scheme: Option<String>,
    pub colors: ColorScheme,
    pub fonts: FontConfig,
    pub window: WindowConfig,
    pub cursor: CursorConfig,
    pub backdrop: BackdropConfig,
    pub gpu: GPUConfig,
    pub general: GeneralConfig,
    pub command_palette: CommandPaletteConfig,
    pub visual_bell: VisualBellConfig,
    pub keybindings: KeyBindingsConfig,
}

// ============================================================================
// Command Response Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveResult {
    pub success: bool,
    pub files_written: Vec<String>,
    pub backups_created: Vec<String>,
    pub config_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Windows,
    Macos,
    Linux,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: Platform,
    pub config_dir: String,
    pub config_exists: bool,
    pub wezterm_installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub filename: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}

// ============================================================================
// Default Implementations
// ============================================================================

impl Default for TabColors {
    fn default() -> Self {
        Self {
            bg_color: "#313244".to_string(),
            fg_color: "#cdd6f4".to_string(),
            italic: None,
        }
    }
}

impl Default for TabBarColors {
    fn default() -> Self {
        Self {
            background: "rgba(0, 0, 0, 0.4)".to_string(),
            active_tab: TabColors {
                bg_color: "#585b70".to_string(),
                fg_color: "#cdd6f4".to_string(),
                italic: None,
            },
            inactive_tab: TabColors {
                bg_color: "#313244".to_string(),
                fg_color: "#bac2de".to_string(),
                italic: None,
            },
            inactive_tab_hover: TabColors {
                bg_color: "#313244".to_string(),
                fg_color: "#cdd6f4".to_string(),
                italic: None,
            },
            new_tab: TabColors {
                bg_color: "#1f1f28".to_string(),
                fg_color: "#cdd6f4".to_string(),
                italic: None,
            },
            new_tab_hover: TabColors {
                bg_color: "#181825".to_string(),
                fg_color: "#cdd6f4".to_string(),
                italic: Some(true),
            },
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            foreground: "#cdd6f4".to_string(),
            background: "#1f1f28".to_string(),
            cursor_bg: "#f5e0dc".to_string(),
            cursor_border: "#f5e0dc".to_string(),
            cursor_fg: "#11111b".to_string(),
            selection_bg: "#585b70".to_string(),
            selection_fg: "#cdd6f4".to_string(),
            ansi: [
                "#0C0C0C".to_string(),
                "#C50F1F".to_string(),
                "#13A10E".to_string(),
                "#C19C00".to_string(),
                "#0037DA".to_string(),
                "#881798".to_string(),
                "#3A96DD".to_string(),
                "#CCCCCC".to_string(),
            ],
            brights: [
                "#767676".to_string(),
                "#E74856".to_string(),
                "#16C60C".to_string(),
                "#F9F1A5".to_string(),
                "#3B78FF".to_string(),
                "#B4009E".to_string(),
                "#61D6D6".to_string(),
                "#F2F2F2".to_string(),
            ],
            tab_bar: TabBarColors::default(),
            visual_bell: None,
            scrollbar_thumb: None,
            split: None,
        }
    }
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: "JetBrainsMono Nerd Font".to_string(),
            size: 12.0,
            weight: None,
            freetype_load_target: Some(FreetypeTarget::Normal),
            freetype_render_target: Some(FreetypeTarget::Normal),
        }
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: 10.0,
            bottom: 7.5,
        }
    }
}

impl Default for HSB {
    fn default() -> Self {
        Self {
            hue: 1.0,
            saturation: 1.0,
            brightness: 1.0,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            window_padding: Padding::default(),
            window_background_opacity: 1.0,
            window_decorations: WindowDecorations::IntegratedButtonsResize,
            enable_tab_bar: true,
            hide_tab_bar_if_only_one_tab: false,
            use_fancy_tab_bar: true,
            tab_max_width: 25,
            show_tab_index_in_tab_bar: false,
            inactive_pane_hsb: HSB::default(),
            window_close_confirmation: CloseConfirmation::NeverPrompt,
        }
    }
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            default_cursor_style: CursorStyle::BlinkingBlock,
            cursor_blink_rate: 650,
            cursor_blink_ease_in: EaseFunction::EaseOut,
            cursor_blink_ease_out: EaseFunction::EaseOut,
            animation_fps: 120,
        }
    }
}

impl Default for BackdropConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            images_dir: String::new(),
            images: Vec::new(),
            current_index: 0,
            focus_color: "#1f1f28".to_string(),
            overlay_opacity: 0.96,
            random_on_start: false,
        }
    }
}

impl Default for GPUConfig {
    fn default() -> Self {
        Self {
            front_end: FrontEnd::WebGpu,
            webgpu_power_preference: PowerPreference::HighPerformance,
            max_fps: 120,
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            automatically_reload_config: true,
            scrollback_lines: 3500,
            initial_rows: 24,
            initial_cols: 80,
            exit_behavior: ExitBehavior::CloseOnCleanExit,
            audible_bell: AudibleBell::Disabled,
            enable_scroll_bar: false,
            switch_to_last_active_tab_when_closing_tab: true,
            adjust_window_size_when_changing_font_size: true,
        }
    }
}

impl Default for CommandPaletteConfig {
    fn default() -> Self {
        Self {
            fg_color: "#cdd6f4".to_string(),
            bg_color: "#1e1e2e".to_string(),
            font_size: 14.0,
        }
    }
}

impl Default for VisualBellConfig {
    fn default() -> Self {
        Self {
            fade_in_duration_ms: 75,
            fade_out_duration_ms: 150,
            fade_in_function: EaseFunction::EaseIn,
            fade_out_function: EaseFunction::EaseOut,
            target: "BackgroundColor".to_string(),
        }
    }
}

impl Default for LeaderKeyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            key: "Space".to_string(),
            mods: "ALT|CTRL".to_string(),  // SUPER_REV on Windows/Linux
            timeout_ms: 1000,
        }
    }
}

impl Default for MiscBindings {
    fn default() -> Self {
        Self {
            copy_mode: KeyBinding::new("F1", "NONE"),
            command_palette: KeyBinding::new("F2", "NONE"),
            command_palette_alt: KeyBinding::new("p", "CTRL|SHIFT"),
            show_launcher: KeyBinding::new("F3", "NONE"),
            show_tab_launcher: KeyBinding::new("F4", "NONE"),
            show_workspace_launcher: KeyBinding::new("F5", "NONE"),
            toggle_fullscreen: KeyBinding::new("F11", "NONE"),
            show_debug_overlay: KeyBinding::new("F12", "NONE"),
            search: KeyBinding::new("f", "ALT"),  // SUPER on Windows/Linux
            quick_select_url: KeyBinding::new("u", "ALT|CTRL"),  // SUPER_REV
        }
    }
}

impl Default for CopyPasteBindings {
    fn default() -> Self {
        Self {
            copy: KeyBinding::new("c", "CTRL|SHIFT"),
            paste: KeyBinding::new("v", "CTRL|SHIFT"),
            copy_simple: KeyBinding::new("c", "CTRL"),
            paste_simple: KeyBinding::new("v", "CTRL"),
        }
    }
}

impl Default for TabBindings {
    fn default() -> Self {
        Self {
            spawn_tab: KeyBinding::new("t", "ALT"),
            spawn_tab_wsl: KeyBinding::new("t", "ALT|CTRL"),
            close_tab: KeyBinding::new("w", "ALT|CTRL"),
            next_tab: KeyBinding::new("]", "ALT"),
            prev_tab: KeyBinding::new("[", "ALT"),
            move_tab_forward: KeyBinding::new("]", "ALT|CTRL"),
            move_tab_back: KeyBinding::new("[", "ALT|CTRL"),
            rename_tab: KeyBinding::new("r", "ALT|CTRL"),
            manual_update_title: KeyBinding::new("0", "ALT"),
            reset_title: KeyBinding::new("0", "ALT|CTRL"),
            toggle_tab_bar: KeyBinding::new("9", "ALT"),
        }
    }
}

impl Default for WindowBindings {
    fn default() -> Self {
        Self {
            spawn_window: KeyBinding::new("n", "ALT"),
            shrink_window: KeyBinding::new("-", "ALT"),
            grow_window: KeyBinding::new("=", "ALT"),
            maximize_window: KeyBinding::new("Enter", "ALT|CTRL"),
        }
    }
}

impl Default for PaneBindings {
    fn default() -> Self {
        Self {
            split_vertical: KeyBinding::new("\\", "ALT"),
            split_horizontal: KeyBinding::new("\\", "ALT|CTRL"),
            toggle_zoom: KeyBinding::new("Enter", "ALT"),
            close_pane: KeyBinding::new("w", "ALT"),
            nav_up: KeyBinding::new("k", "ALT|CTRL"),
            nav_down: KeyBinding::new("j", "ALT|CTRL"),
            nav_left: KeyBinding::new("h", "ALT|CTRL"),
            nav_right: KeyBinding::new("l", "ALT|CTRL"),
            swap_pane: KeyBinding::new("p", "ALT|CTRL"),
            scroll_up: KeyBinding::new("u", "ALT"),
            scroll_down: KeyBinding::new("d", "ALT"),
            page_up: KeyBinding::new("PageUp", "NONE"),
            page_down: KeyBinding::new("PageDown", "NONE"),
        }
    }
}

impl Default for BackdropBindings {
    fn default() -> Self {
        Self {
            random: KeyBinding::new("/", "ALT"),
            cycle_back: KeyBinding::new(",", "ALT"),
            cycle_forward: KeyBinding::new(".", "ALT"),
            select: KeyBinding::new("/", "ALT|CTRL"),
            toggle_focus: KeyBinding::new("b", "ALT"),
        }
    }
}

impl Default for CursorBindings {
    fn default() -> Self {
        Self {
            home: KeyBinding::new("LeftArrow", "ALT"),
            end: KeyBinding::new("RightArrow", "ALT"),
            delete_line: KeyBinding::new("Backspace", "ALT"),
            newline: KeyBinding::new("Enter", "SHIFT"),
        }
    }
}

impl Default for KeyTableBindings {
    fn default() -> Self {
        Self {
            resize_font_mode: KeyBinding::new("f", "LEADER"),
            resize_pane_mode: KeyBinding::new("p", "LEADER"),
        }
    }
}

impl Default for MouseBindings {
    fn default() -> Self {
        Self {
            ctrl_click_open_link: true,
            right_click_command_palette: true,
        }
    }
}

impl Default for CustomCommands {
    fn default() -> Self {
        Self {
            settings_tui: true,
            rename_tab: true,
        }
    }
}

impl Default for KeyBindingsConfig {
    fn default() -> Self {
        Self {
            disable_defaults: true,
            leader: LeaderKeyConfig::default(),
            misc: MiscBindings::default(),
            copy_paste: CopyPasteBindings::default(),
            tabs: TabBindings::default(),
            windows: WindowBindings::default(),
            panes: PaneBindings::default(),
            backdrops: BackdropBindings::default(),
            cursor: CursorBindings::default(),
            key_tables: KeyTableBindings::default(),
            mouse: MouseBindings::default(),
            custom_commands: CustomCommands::default(),
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            color_scheme: None, // Use custom colors by default
            colors: ColorScheme::default(),
            fonts: FontConfig::default(),
            window: WindowConfig::default(),
            cursor: CursorConfig::default(),
            backdrop: BackdropConfig::default(),
            gpu: GPUConfig::default(),
            general: GeneralConfig::default(),
            command_palette: CommandPaletteConfig::default(),
            visual_bell: VisualBellConfig::default(),
            keybindings: KeyBindingsConfig::default(),
        }
    }
}
