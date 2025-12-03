# Data Model: Appearance Settings Editor

**Feature**: 001-appearance-settings  
**Date**: 2024-11-28

## Overview

This document defines the data entities for the appearance settings feature. These models are used in both the Rust backend (serde structs) and TypeScript frontend (interfaces).

---

## Core Entities

### 1. ColorScheme

Represents the complete color configuration for WezTerm.

```typescript
interface ColorScheme {
  // Core colors
  foreground: string;        // Hex color, e.g., "#cdd6f4"
  background: string;        // Hex color
  cursor_bg: string;         // Cursor background
  cursor_border: string;     // Cursor border
  cursor_fg: string;         // Text color when cursor is over it
  selection_bg: string;      // Selection background
  selection_fg: string;      // Selection text color

  // ANSI palette (16 colors)
  ansi: [string, string, string, string, string, string, string, string];  // 0-7
  brights: [string, string, string, string, string, string, string, string]; // 8-15

  // Tab bar colors
  tab_bar: TabBarColors;

  // Optional
  visual_bell?: string;      // Color for visual bell
  scrollbar_thumb?: string;  // Scrollbar color
  split?: string;            // Pane split color
}

interface TabBarColors {
  background: string;
  active_tab: TabColors;
  inactive_tab: TabColors;
  inactive_tab_hover: TabColors;
  new_tab: TabColors;
  new_tab_hover: TabColors;
}

interface TabColors {
  bg_color: string;
  fg_color: string;
  italic?: boolean;
}
```

**Rust equivalent:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub foreground: String,
    pub background: String,
    pub cursor_bg: String,
    pub cursor_border: String,
    pub cursor_fg: String,
    pub selection_bg: String,
    pub selection_fg: String,
    pub ansi: [String; 8],
    pub brights: [String; 8],
    pub tab_bar: TabBarColors,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_bell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollbar_thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<String>,
}
```

**Validation Rules:**
- All color strings must be valid hex format: `#RRGGBB` or `#RGB`
- Arrays must have exactly 8 elements
- Tab bar colors are required

**Output File:** `colors/custom.lua`

---

### 2. FontConfig

Represents font settings for the terminal.

```typescript
interface FontConfig {
  family: string;            // Font family name, e.g., "JetBrainsMono Nerd Font"
  size: number;              // Font size in points, e.g., 12
  weight?: FontWeight;       // Optional weight override
  freetype_load_target?: FreetypeTarget;
  freetype_render_target?: FreetypeTarget;
}

type FontWeight = 'Thin' | 'ExtraLight' | 'Light' | 'Regular' | 'Medium' | 'DemiBold' | 'Bold' | 'ExtraBold' | 'Black';
type FreetypeTarget = 'Normal' | 'Light' | 'Mono' | 'HorizontalLcd';
```

**Rust equivalent:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub family: String,
    pub size: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freetype_load_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freetype_render_target: Option<String>,
}
```

**Validation Rules:**
- `family` must be non-empty string
- `size` must be between 6.0 and 72.0
- `weight` must be one of the valid FontWeight values

**Output File:** `config/fonts.lua`

---

### 3. WindowConfig

Represents window appearance settings.

```typescript
interface WindowConfig {
  // Padding
  window_padding: Padding;

  // Opacity
  window_background_opacity: number;  // 0.0 to 1.0

  // Decorations
  window_decorations: WindowDecorations;

  // Tab bar
  enable_tab_bar: boolean;
  hide_tab_bar_if_only_one_tab: boolean;
  use_fancy_tab_bar: boolean;
  tab_max_width: number;
  show_tab_index_in_tab_bar: boolean;

  // Pane
  inactive_pane_hsb: HSB;

  // Close behavior
  window_close_confirmation: CloseConfirmation;
}

interface Padding {
  left: number;
  right: number;
  top: number;
  bottom: number;
}

interface HSB {
  hue: number;         // 0.0 to 1.0
  saturation: number;  // 0.0 to 1.0
  brightness: number;  // 0.0 to 1.0
}

type WindowDecorations = 'FULL' | 'RESIZE' | 'NONE' | 'TITLE' | 'INTEGRATED_BUTTONS|RESIZE';
type CloseConfirmation = 'AlwaysPrompt' | 'NeverPrompt';
```

**Rust equivalent:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub window_padding: Padding,
    pub window_background_opacity: f32,
    pub window_decorations: String,
    pub enable_tab_bar: bool,
    pub hide_tab_bar_if_only_one_tab: bool,
    pub use_fancy_tab_bar: bool,
    pub tab_max_width: u32,
    pub show_tab_index_in_tab_bar: bool,
    pub inactive_pane_hsb: HSB,
    pub window_close_confirmation: String,
}

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
```

**Validation Rules:**
- `window_background_opacity` must be between 0.0 and 1.0
- `tab_max_width` must be between 1 and 100
- Padding values must be non-negative

**Output File:** `config/appearance.lua`

---

### 4. CursorConfig

Represents cursor appearance settings.

```typescript
interface CursorConfig {
  default_cursor_style: CursorStyle;
  cursor_blink_rate: number;           // milliseconds
  cursor_blink_ease_in: EaseFunction;
  cursor_blink_ease_out: EaseFunction;
  animation_fps: number;
}

type CursorStyle = 'SteadyBlock' | 'BlinkingBlock' | 'SteadyUnderline' | 'BlinkingUnderline' | 'SteadyBar' | 'BlinkingBar';
type EaseFunction = 'Linear' | 'EaseIn' | 'EaseOut' | 'EaseInOut' | 'Constant';
```

**Rust equivalent:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorConfig {
    pub default_cursor_style: String,
    pub cursor_blink_rate: u32,
    pub cursor_blink_ease_in: String,
    pub cursor_blink_ease_out: String,
    pub animation_fps: u32,
}
```

**Validation Rules:**
- `cursor_blink_rate` must be between 0 and 5000
- `animation_fps` must be between 1 and 240

**Output File:** `config/appearance.lua` (merged with WindowConfig)

---

### 5. BackdropConfig

Represents background image configuration.

```typescript
interface BackdropConfig {
  enabled: boolean;
  images_dir: string;           // Path to backdrops folder
  images: string[];             // List of image filenames
  current_index: number;        // Currently selected image
  focus_color: string;          // Color when focus mode enabled
  overlay_opacity: number;      // 0.0 to 1.0
  random_on_start: boolean;
}
```

**Rust equivalent:**
```rust
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
```

**Validation Rules:**
- `overlay_opacity` must be between 0.0 and 1.0
- `focus_color` must be valid hex color
- `images_dir` must be valid path

**Output Files:** `utils/backdrops.lua` configuration, images in `backdrops/` folder

---

### 6. GPUConfig

Represents GPU and rendering settings.

```typescript
interface GPUConfig {
  front_end: FrontEnd;
  webgpu_power_preference: PowerPreference;
  max_fps: number;
}

type FrontEnd = 'WebGpu' | 'OpenGL' | 'Software';
type PowerPreference = 'LowPower' | 'HighPerformance';
```

**Rust equivalent:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUConfig {
    pub front_end: String,
    pub webgpu_power_preference: String,
    pub max_fps: u32,
}
```

**Validation Rules:**
- `max_fps` must be between 1 and 240

**Output File:** `config/appearance.lua`

---

## Composite Types

### AppearanceConfig (Complete)

The full appearance configuration combining all settings:

```typescript
interface AppearanceConfig {
  colors: ColorScheme;
  fonts: FontConfig;
  window: WindowConfig;
  cursor: CursorConfig;
  backdrop: BackdropConfig;
  gpu: GPUConfig;
}
```

**Rust equivalent:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub colors: ColorScheme,
    pub fonts: FontConfig,
    pub window: WindowConfig,
    pub cursor: CursorConfig,
    pub backdrop: BackdropConfig,
    pub gpu: GPUConfig,
}
```

---

## Default Values

### Default ColorScheme (Catppuccin Mocha variant)
```json
{
  "foreground": "#cdd6f4",
  "background": "#1f1f28",
  "cursor_bg": "#f5e0dc",
  "cursor_border": "#f5e0dc",
  "cursor_fg": "#11111b",
  "selection_bg": "#585b70",
  "selection_fg": "#cdd6f4",
  "ansi": ["#0C0C0C", "#C50F1F", "#13A10E", "#C19C00", "#0037DA", "#881798", "#3A96DD", "#CCCCCC"],
  "brights": ["#767676", "#E74856", "#16C60C", "#F9F1A5", "#3B78FF", "#B4009E", "#61D6D6", "#F2F2F2"]
}
```

### Default FontConfig
```json
{
  "family": "JetBrainsMono Nerd Font",
  "size": 12,
  "freetype_load_target": "Normal",
  "freetype_render_target": "Normal"
}
```

### Default WindowConfig
```json
{
  "window_padding": { "left": 0, "right": 0, "top": 10, "bottom": 7.5 },
  "window_background_opacity": 1.0,
  "window_decorations": "INTEGRATED_BUTTONS|RESIZE",
  "enable_tab_bar": true,
  "hide_tab_bar_if_only_one_tab": false,
  "use_fancy_tab_bar": true,
  "tab_max_width": 25,
  "show_tab_index_in_tab_bar": false,
  "inactive_pane_hsb": { "hue": 1.0, "saturation": 1.0, "brightness": 1.0 },
  "window_close_confirmation": "NeverPrompt"
}
```

### Default CursorConfig
```json
{
  "default_cursor_style": "BlinkingBlock",
  "cursor_blink_rate": 650,
  "cursor_blink_ease_in": "EaseOut",
  "cursor_blink_ease_out": "EaseOut",
  "animation_fps": 120
}
```

---

## File Output Mapping

| Entity | Output File |
|--------|-------------|
| ColorScheme | `colors/custom.lua` |
| FontConfig | `config/fonts.lua` |
| WindowConfig + CursorConfig + GPUConfig | `config/appearance.lua` |
| BackdropConfig | `utils/backdrops.lua` (init) + `backdrops/*.{jpg,png}` |

---

## State Transitions

### Config Lifecycle

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Default   │────▶│   Editing   │────▶│    Saved    │
│   Values    │     │  (in GUI)   │     │  (on disk)  │
└─────────────┘     └─────────────┘     └─────────────┘
       ▲                   │                   │
       │                   │ Cancel            │ Load
       │                   ▼                   │
       │            ┌─────────────┐            │
       └────────────│  Discarded  │◀───────────┘
                    └─────────────┘
```

### Backup Strategy

Before any save operation:
1. Check if target file exists
2. If exists, copy to `{filename}.bak.{timestamp}`
3. Write new content
4. Keep last 3 backups per file
