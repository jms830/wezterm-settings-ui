# Feature Specification: Appearance Settings Editor

**Feature Branch**: `001-appearance-settings`  
**Created**: 2024-11-28  
**Status**: Draft  
**Input**: Visual appearance settings editor for colors, fonts, backgrounds, and window styling with modular Lua config output

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Configure Color Scheme (Priority: P1)

A user wants to customize their terminal colors without manually editing Lua files. They open the Settings GUI, navigate to Colors, and use color pickers to adjust foreground, background, cursor, and ANSI colors. Changes are previewed live and saved to `colors/custom.lua`.

**Why this priority**: Colors are the most visually impactful setting and validate the core value proposition - GUI editing of Lua config.

**Independent Test**: Can be fully tested by changing colors in GUI and verifying the generated `colors/custom.lua` loads correctly in WezTerm.

**Acceptance Scenarios**:

1. **Given** the GUI is open, **When** user clicks on "Background Color" and selects #1a1b26, **Then** the color picker shows the selection AND preview updates
2. **Given** user has modified colors, **When** user clicks "Save", **Then** `colors/custom.lua` is generated with valid Lua syntax
3. **Given** a generated `colors/custom.lua` exists, **When** WezTerm loads, **Then** colors are applied without errors

---

### User Story 2 - Configure Font Settings (Priority: P1)

A user wants to change their terminal font family and size. They select from available system fonts, adjust size, and configure font rendering options. Changes save to `config/fonts.lua`.

**Why this priority**: Fonts are essential for terminal usability and frequently adjusted.

**Independent Test**: Can be tested by selecting a font, saving, and verifying WezTerm displays the new font.

**Acceptance Scenarios**:

1. **Given** the GUI is open, **When** user navigates to Fonts section, **Then** a list of available monospace fonts is displayed
2. **Given** user selects "JetBrainsMono Nerd Font" size 12, **When** user clicks Save, **Then** `config/fonts.lua` contains `font = wezterm.font({ family = 'JetBrainsMono Nerd Font' })` and `font_size = 12`
3. **Given** font is configured, **When** WezTerm reloads, **Then** new font is applied

---

### User Story 3 - Configure Window Appearance (Priority: P2)

A user wants to adjust window decorations, padding, opacity, and tab bar style. They use sliders and toggles to configure these settings. Changes save to `config/appearance.lua`.

**Why this priority**: Window settings are important but less frequently changed than colors/fonts.

**Independent Test**: Can be tested by adjusting window opacity, saving, and verifying WezTerm shows the change.

**Acceptance Scenarios**:

1. **Given** the GUI is open, **When** user adjusts window_background_opacity slider to 0.9, **Then** preview shows semi-transparent effect
2. **Given** user configures padding (left: 10, right: 10, top: 10, bottom: 10), **When** saved, **Then** `config/appearance.lua` contains correct `window_padding` table
3. **Given** user toggles "Hide tab bar if only one tab", **When** saved, **Then** `hide_tab_bar_if_only_one_tab = true` in config

---

### User Story 4 - Configure Background Images (Priority: P2)

A user wants to set a background image for their terminal. They browse/select images, configure opacity overlay, and optionally enable random cycling. This integrates with the `backdrops` utility pattern.

**Why this priority**: Background images are a popular feature in Kevin's config but more complex to implement.

**Independent Test**: Can be tested by selecting an image, saving, and verifying it appears in WezTerm.

**Acceptance Scenarios**:

1. **Given** the GUI is open, **When** user clicks "Add Background Image", **Then** file picker opens for image selection
2. **Given** user selects an image, **When** saved, **Then** image is copied to `backdrops/` folder AND backdrop config is updated
3. **Given** user enables "Random backdrop on start", **When** WezTerm starts, **Then** a random image from `backdrops/` is shown

---

### User Story 5 - Configure Cursor Appearance (Priority: P3)

A user wants to customize cursor style (block, bar, underline), blink rate, and colors. Changes save to `config/appearance.lua`.

**Why this priority**: Cursor settings are nice-to-have but not critical for MVP.

**Independent Test**: Can be tested by changing cursor style and verifying in WezTerm.

**Acceptance Scenarios**:

1. **Given** the GUI is open, **When** user selects "BlinkingBar" cursor style, **Then** `default_cursor_style = 'BlinkingBar'` is saved
2. **Given** user sets blink rate to 500ms, **When** saved, **Then** `cursor_blink_rate = 500` in config

---

### User Story 6 - Import Existing Config (Priority: P3)

A user has an existing WezTerm config and wants to import it into the GUI. The GUI parses their existing Lua files and populates the settings.

**Why this priority**: Important for adoption but complex - requires Lua parsing.

**Independent Test**: Can be tested by pointing to existing config directory and verifying settings populate.

**Acceptance Scenarios**:

1. **Given** user has existing config at `~/.config/wezterm`, **When** user clicks "Import Existing Config", **Then** GUI reads and populates color values from `colors/custom.lua`
2. **Given** config uses non-modular structure, **When** imported, **Then** GUI extracts what it can and warns about unsupported patterns

---

### Edge Cases

- What happens when user's WezTerm config directory doesn't exist? → Create it with default structure
- What happens when selected font isn't installed? → Show warning, allow save anyway
- How does system handle invalid color values? → Validate hex format, show error inline
- What happens when background image path has spaces? → Properly quote in Lua output
- What if user manually edits generated Lua and breaks syntax? → Detect on next load, offer to restore backup

---

## Distribution Goals

### Installation Methods (Future)

The GUI should be easily installable via multiple methods:

1. **Terminal Command**: `wezterm-settings` launches the GUI
2. **WezTerm Plugin**: Installable directly from WezTerm using the plugin system
3. **Package Managers**: Available via cargo, brew, apt, etc.

### CLI Interface

```bash
wezterm-settings              # Open GUI
wezterm-settings --config-dir ~/.config/wezterm  # Specify config location
wezterm-settings colors       # Jump to colors section
wezterm-settings fonts        # Jump to fonts section
wezterm-settings --export     # Export current config to stdout
```

### WezTerm Integration (Future)

Could integrate as a WezTerm plugin that:
- Adds a keybinding (e.g., `CTRL+SHIFT+,`) to open settings
- Adds "Settings" to the launcher menu
- Auto-reloads config after saving

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST generate valid WezTerm Lua configuration files
- **FR-002**: System MUST output modular config structure (separate files for colors, fonts, appearance, etc.)
- **FR-003**: System MUST provide color picker for all color settings (foreground, background, cursor, ANSI 0-15, brights)
- **FR-004**: System MUST list available monospace fonts from the system
- **FR-005**: System MUST allow numeric input for: font_size, window_padding, opacity, cursor_blink_rate
- **FR-006**: System MUST support dropdown selection for enum values (cursor_style, front_end, etc.)
- **FR-007**: System MUST create backup of existing config before overwriting
- **FR-008**: System MUST detect WezTerm config directory location (platform-specific defaults)
- **FR-009**: System MUST validate generated Lua syntax before saving
- **FR-010**: System MUST copy background images to `backdrops/` folder within config directory

### Key Entities

- **ColorScheme**: Foreground, background, cursor colors, ANSI palette (16 colors), brights palette, tab bar colors, selection colors
- **FontConfig**: Font family, font size, font weight, freetype rendering options
- **WindowConfig**: Padding (left/right/top/bottom), opacity, decorations, tab bar visibility/style
- **CursorConfig**: Style (Block/Bar/Underline), blink rate, blink easing
- **BackdropConfig**: Image paths, overlay opacity, focus mode color, random selection enabled

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: User can configure and save a complete color scheme in under 2 minutes
- **SC-002**: Generated Lua files pass `luacheck` without errors
- **SC-003**: Generated config loads in WezTerm without errors or warnings
- **SC-004**: Settings round-trip: save → reload GUI → values match original input
- **SC-005**: User can go from fresh install to customized WezTerm in under 5 minutes
