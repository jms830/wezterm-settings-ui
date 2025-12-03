// TypeScript interfaces for WezTerm configuration
// Generated from data-model.md

// ============================================================================
// Color Types
// ============================================================================

export interface TabColors {
  bg_color: string;
  fg_color: string;
  italic?: boolean;
}

export interface TabBarColors {
  background: string;
  active_tab: TabColors;
  inactive_tab: TabColors;
  inactive_tab_hover: TabColors;
  new_tab: TabColors;
  new_tab_hover: TabColors;
}

export interface ColorScheme {
  // Core colors
  foreground: string;
  background: string;
  cursor_bg: string;
  cursor_border: string;
  cursor_fg: string;
  selection_bg: string;
  selection_fg: string;

  // ANSI palette (16 colors)
  ansi: [string, string, string, string, string, string, string, string];
  brights: [string, string, string, string, string, string, string, string];

  // Tab bar colors
  tab_bar: TabBarColors;

  // Optional
  visual_bell?: string;
  scrollbar_thumb?: string;
  split?: string;
}

// ============================================================================
// Font Types
// ============================================================================

export type FontWeight =
  | 'Thin'
  | 'ExtraLight'
  | 'Light'
  | 'Regular'
  | 'Medium'
  | 'DemiBold'
  | 'Bold'
  | 'ExtraBold'
  | 'Black';

export type FreetypeTarget = 'Normal' | 'Light' | 'Mono' | 'HorizontalLcd';

export interface FontConfig {
  family: string;
  size: number;
  weight?: FontWeight;
  freetype_load_target?: FreetypeTarget;
  freetype_render_target?: FreetypeTarget;
}

// ============================================================================
// Window Types
// ============================================================================

export interface Padding {
  left: number;
  right: number;
  top: number;
  bottom: number;
}

export interface HSB {
  hue: number;
  saturation: number;
  brightness: number;
}

export type WindowDecorations =
  | 'FULL'
  | 'RESIZE'
  | 'NONE'
  | 'TITLE'
  | 'INTEGRATED_BUTTONS|RESIZE';

export type CloseConfirmation = 'AlwaysPrompt' | 'NeverPrompt';

export interface WindowConfig {
  window_padding: Padding;
  window_background_opacity: number;
  window_decorations: WindowDecorations;
  enable_tab_bar: boolean;
  hide_tab_bar_if_only_one_tab: boolean;
  use_fancy_tab_bar: boolean;
  tab_max_width: number;
  show_tab_index_in_tab_bar: boolean;
  inactive_pane_hsb: HSB;
  window_close_confirmation: CloseConfirmation;
}

// ============================================================================
// Cursor Types
// ============================================================================

export type CursorStyle =
  | 'SteadyBlock'
  | 'BlinkingBlock'
  | 'SteadyUnderline'
  | 'BlinkingUnderline'
  | 'SteadyBar'
  | 'BlinkingBar';

export type EaseFunction =
  | 'Linear'
  | 'EaseIn'
  | 'EaseOut'
  | 'EaseInOut'
  | 'Constant';

export interface CursorConfig {
  default_cursor_style: CursorStyle;
  cursor_blink_rate: number;
  cursor_blink_ease_in: EaseFunction;
  cursor_blink_ease_out: EaseFunction;
  animation_fps: number;
}

// ============================================================================
// Backdrop Types
// ============================================================================

export interface BackdropConfig {
  enabled: boolean;
  images_dir: string;
  images: string[];
  current_index: number;
  focus_color: string;
  overlay_opacity: number;
  random_on_start: boolean;
}

// ============================================================================
// Command Palette Types
// ============================================================================

export interface CommandPaletteConfig {
  fg_color: string;
  bg_color: string;
  font_size: number;
}

// ============================================================================
// Visual Bell Types
// ============================================================================

export interface VisualBellConfig {
  fade_in_duration_ms: number;
  fade_out_duration_ms: number;
  fade_in_function: EaseFunction;
  fade_out_function: EaseFunction;
  target: string;
}

// ============================================================================
// General Types
// ============================================================================

export type ExitBehavior = 'Close' | 'CloseOnCleanExit' | 'Hold';
export type AudibleBell = 'SystemBeep' | 'Disabled';

export interface GeneralConfig {
  automatically_reload_config: boolean;
  scrollback_lines: number;
  initial_rows: number;
  initial_cols: number;
  exit_behavior: ExitBehavior;
  audible_bell: AudibleBell;
  enable_scroll_bar: boolean;
  switch_to_last_active_tab_when_closing_tab: boolean;
  adjust_window_size_when_changing_font_size: boolean;
}

// ============================================================================
// GPU Types
// ============================================================================

export type FrontEnd = 'WebGpu' | 'OpenGL' | 'Software';
export type PowerPreference = 'LowPower' | 'HighPerformance';

export interface GPUConfig {
  front_end: FrontEnd;
  webgpu_power_preference: PowerPreference;
  max_fps: number;
}

// ============================================================================
// Composite Types
// ============================================================================

export interface AppearanceConfig {
  /** Use a built-in WezTerm color scheme by name (e.g., "Catppuccin Mocha")
   * When set, overrides the custom `colors` settings */
  color_scheme?: string;
  colors: ColorScheme;
  fonts: FontConfig;
  window: WindowConfig;
  cursor: CursorConfig;
  backdrop: BackdropConfig;
  gpu: GPUConfig;
  general: GeneralConfig;
  command_palette: CommandPaletteConfig;
  visual_bell: VisualBellConfig;
}

// ============================================================================
// Command Response Types
// ============================================================================

export interface SaveResult {
  success: boolean;
  files_written: string[];
  backups_created: string[];
  config_dir: string;
}

export interface ValidationError {
  field: string;
  message: string;
  value: string;
}

export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

export interface SystemInfo {
  platform: 'windows' | 'macos' | 'linux';
  config_dir: string;
  config_exists: boolean;
  wezterm_installed: boolean;
}

export interface ImageInfo {
  filename: string;
  path: string;
  thumbnail?: string;
}

// ============================================================================
// Default Values
// ============================================================================

export const DEFAULT_COLOR_SCHEME: ColorScheme = {
  foreground: '#cdd6f4',
  background: '#1f1f28',
  cursor_bg: '#f5e0dc',
  cursor_border: '#f5e0dc',
  cursor_fg: '#11111b',
  selection_bg: '#585b70',
  selection_fg: '#cdd6f4',
  ansi: [
    '#0C0C0C',
    '#C50F1F',
    '#13A10E',
    '#C19C00',
    '#0037DA',
    '#881798',
    '#3A96DD',
    '#CCCCCC',
  ],
  brights: [
    '#767676',
    '#E74856',
    '#16C60C',
    '#F9F1A5',
    '#3B78FF',
    '#B4009E',
    '#61D6D6',
    '#F2F2F2',
  ],
  tab_bar: {
    background: 'rgba(0, 0, 0, 0.4)',
    active_tab: { bg_color: '#585b70', fg_color: '#cdd6f4' },
    inactive_tab: { bg_color: '#313244', fg_color: '#bac2de' },
    inactive_tab_hover: { bg_color: '#313244', fg_color: '#cdd6f4' },
    new_tab: { bg_color: '#1f1f28', fg_color: '#cdd6f4' },
    new_tab_hover: { bg_color: '#181825', fg_color: '#cdd6f4', italic: true },
  },
};

export const DEFAULT_FONT_CONFIG: FontConfig = {
  family: 'JetBrainsMono Nerd Font',
  size: 12,
  freetype_load_target: 'Normal',
  freetype_render_target: 'Normal',
};

export const DEFAULT_WINDOW_CONFIG: WindowConfig = {
  window_padding: { left: 0, right: 0, top: 10, bottom: 7.5 },
  window_background_opacity: 1.0,
  window_decorations: 'INTEGRATED_BUTTONS|RESIZE',
  enable_tab_bar: true,
  hide_tab_bar_if_only_one_tab: false,
  use_fancy_tab_bar: true,
  tab_max_width: 25,
  show_tab_index_in_tab_bar: false,
  inactive_pane_hsb: { hue: 1.0, saturation: 1.0, brightness: 1.0 },
  window_close_confirmation: 'NeverPrompt',
};

export const DEFAULT_CURSOR_CONFIG: CursorConfig = {
  default_cursor_style: 'BlinkingBlock',
  cursor_blink_rate: 650,
  cursor_blink_ease_in: 'EaseOut',
  cursor_blink_ease_out: 'EaseOut',
  animation_fps: 120,
};

export const DEFAULT_BACKDROP_CONFIG: BackdropConfig = {
  enabled: false,
  images_dir: '',
  images: [],
  current_index: 0,
  focus_color: '#1f1f28',
  overlay_opacity: 0.96,
  random_on_start: false,
};

export const DEFAULT_GPU_CONFIG: GPUConfig = {
  front_end: 'WebGpu',
  webgpu_power_preference: 'HighPerformance',
  max_fps: 120,
};

export const DEFAULT_GENERAL_CONFIG: GeneralConfig = {
  automatically_reload_config: true,
  scrollback_lines: 3500,
  initial_rows: 24,
  initial_cols: 80,
  exit_behavior: 'CloseOnCleanExit',
  audible_bell: 'Disabled',
  enable_scroll_bar: false,
  switch_to_last_active_tab_when_closing_tab: true,
  adjust_window_size_when_changing_font_size: true,
};

export const DEFAULT_COMMAND_PALETTE_CONFIG: CommandPaletteConfig = {
  fg_color: '#cdd6f4',
  bg_color: '#1e1e2e',
  font_size: 14,
};

export const DEFAULT_VISUAL_BELL_CONFIG: VisualBellConfig = {
  fade_in_duration_ms: 75,
  fade_out_duration_ms: 150,
  fade_in_function: 'EaseIn',
  fade_out_function: 'EaseOut',
  target: 'BackgroundColor',
};

export const DEFAULT_APPEARANCE_CONFIG: AppearanceConfig = {
  colors: DEFAULT_COLOR_SCHEME,
  fonts: DEFAULT_FONT_CONFIG,
  window: DEFAULT_WINDOW_CONFIG,
  cursor: DEFAULT_CURSOR_CONFIG,
  backdrop: DEFAULT_BACKDROP_CONFIG,
  gpu: DEFAULT_GPU_CONFIG,
  general: DEFAULT_GENERAL_CONFIG,
  command_palette: DEFAULT_COMMAND_PALETTE_CONFIG,
  visual_bell: DEFAULT_VISUAL_BELL_CONFIG,
};
