# Feature Specification: TUI Settings Editor

**Feature Branch**: `001-tui-settings`  
**Created**: 2024-11-28  
**Status**: Active  
**Pivot From**: GUI approach (deferred to `specs/gui-frontend-deferred/`)

## Overview

A terminal-based settings editor for WezTerm configuration. Launches with `wezterm-settings` command, provides keyboard-driven navigation through settings panels, and generates modular Lua config files.

## Why TUI Over GUI

1. **Native to workflow**: WezTerm users live in the terminal
2. **Lightweight**: No webkit/webview (~200MB smaller binary)
3. **Fast**: Instant startup, no window initialization
4. **SSH-friendly**: Works over remote connections
5. **Ecosystem fit**: Matches tools like lazygit, btop, yazi

---

## User Stories

### US1 - Navigate Settings (Priority: P0)

User launches `wezterm-settings`, sees a sidebar with categories (Colors, Fonts, Window, Cursor, GPU), and navigates with arrow keys or vim bindings.

**Acceptance**:
1. `wezterm-settings` launches TUI in current terminal
2. Left panel shows settings categories
3. `j/k` or arrows navigate categories
4. `Enter` or `l` opens category
5. `q` or `Esc` quits

---

### US2 - Edit Color Scheme (Priority: P1)

User navigates to Colors, sees all color fields (foreground, background, cursor, ANSI 0-15, brights), edits hex values, sees live preview swatch, saves to `colors/custom.lua`.

**Acceptance**:
1. Colors panel shows all color fields with current values
2. Hex input with validation (#RRGGBB format)
3. Color swatch preview next to each field (using terminal colors)
4. Tab/Shift+Tab navigates between fields
5. `Ctrl+S` saves changes

---

### US3 - Edit Font Settings (Priority: P1)

User navigates to Fonts, selects from system monospace fonts via fuzzy finder, adjusts size with number input, saves to `config/fonts.lua`.

**Acceptance**:
1. Font family field opens fuzzy finder with system monospace fonts
2. Font size is numeric input (8-72 range)
3. Optional: weight selection dropdown
4. Preview shows sample text in selected font (if terminal supports)

---

### US4 - Edit Window Settings (Priority: P2)

User adjusts window opacity (slider/number), padding values, tab bar options, saves to `config/appearance.lua`.

**Acceptance**:
1. Opacity: number input 0.0-1.0 with 0.05 steps
2. Padding: four number inputs (left, right, top, bottom)
3. Tab bar: toggle switches for enable, hide_if_one, fancy
4. Decorations: dropdown selection

---

### US5 - Edit Cursor Settings (Priority: P2)

User selects cursor style from list, adjusts blink rate, saves to config.

**Acceptance**:
1. Cursor style: selection list (SteadyBlock, BlinkingBlock, etc.)
2. Blink rate: number input in ms
3. Animation FPS: number input

---

### US6 - Edit Basic Keybindings (Priority: P2)

User can configure common keybindings for built-in WezTerm actions (spawn tab, activate tab, split panes, copy/paste, etc.) and basic prompts like "Rename Current Tab".

**Acceptance**:
1. List shows common actions (spawn tab, close tab, split panes, activate tabs 1-9, copy/paste)
2. Special case: "Rename Current Tab" generates PromptInputLine with event callback
3. Each binding shows: key combination + action description
4. User can edit key and modifiers (Ctrl, Alt, Shift, Super)
5. Warning displayed: "Custom events and advanced keybindings must be edited manually"
6. Saves to `config/keybindings.lua`

**Limitations** (documented in UI):
- Cannot create custom event handlers
- Cannot edit existing custom events (shows as read-only)
- Action callbacks limited to built-in actions + "Rename Current Tab" pattern

---

### US7 - Save & Backup (Priority: P1)

When user saves, system creates timestamped backup of existing files, generates new Lua files, shows success/error message.

**Acceptance**:
1. `Ctrl+S` triggers save
2. Existing files backed up to `.backup/` with timestamp
3. Success message shows files written
4. Error message if validation fails (invalid hex, etc.)

---

## UI Layout

```
┌─────────────────────────────────────────────────────────────┐
│  WezTerm Settings                              Ctrl+S: Save │
├──────────────┬──────────────────────────────────────────────┤
│              │                                              │
│  > Colors    │  Foreground    #cdd6f4  ██                   │
│    Fonts     │  Background    #1f1f28  ██                   │
│    Window    │  Cursor BG     #f5e0dc  ██                   │
│    Cursor    │  Cursor FG     #11111b  ██                   │
│    Keys      │                                              │
│    GPU       │                                              │
│              │  ANSI Colors                                 │
│              │  [0] #0C0C0C ██  [1] #C50F1F ██              │
│              │  [2] #13A10E ██  [3] #C19C00 ██              │
│              │  ...                                         │
│              │                                              │
├──────────────┴──────────────────────────────────────────────┤
│  j/k: Navigate  Enter: Edit  Tab: Next Field  q: Quit       │
└─────────────────────────────────────────────────────────────┘
```

---

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `h` / `←` | Back / collapse |
| `l` / `→` / `Enter` | Select / expand / edit |
| `Tab` | Next field |
| `Shift+Tab` | Previous field |
| `Ctrl+S` | Save all changes |
| `Ctrl+Z` | Undo last change |
| `q` / `Esc` | Quit (prompt if unsaved) |
| `/` | Search/filter settings |
| `?` | Show help |

---

## Technical Stack

### Dependencies

```toml
[dependencies]
ratatui = "0.29"          # TUI framework
crossterm = "0.28"        # Terminal backend
tui-input = "0.11"        # Text input widget
fuzzy-matcher = "0.3"     # Font fuzzy search

# Reuse from existing
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tera = "1.20"             # Lua generation
fontdb = "0.23"           # Font enumeration
dirs = "6.0"              # Config paths
```

### Project Structure

```
src/
├── main.rs               # Entry point, arg parsing
├── app.rs                # App state, event loop
├── ui/
│   ├── mod.rs
│   ├── layout.rs         # Main layout (sidebar + panel)
│   ├── sidebar.rs        # Category navigation
│   ├── panels/
│   │   ├── colors.rs     # Color settings panel
│   │   ├── fonts.rs      # Font settings panel
│   │   ├── window.rs     # Window settings panel
│   │   ├── cursor.rs     # Cursor settings panel
│   │   └── gpu.rs        # GPU settings panel
│   └── widgets/
│       ├── color_input.rs    # Hex input + swatch
│       ├── number_input.rs   # Numeric input
│       ├── select.rs         # Dropdown/list select
│       └── toggle.rs         # Boolean toggle
├── config/
│   ├── mod.rs
│   ├── models.rs         # Config structs (reuse existing)
│   ├── loader.rs         # Load existing config
│   └── saver.rs          # Save + backup logic
└── lua/
    ├── mod.rs
    ├── generator.rs      # Tera-based generation (reuse existing)
    └── templates/        # Lua templates
```

---

## Reusable from GUI Approach

These files from `src-tauri/src/` can be reused directly:
- `models/config.rs` - All config structs with serde
- `lua/generator.rs` - Tera-based Lua generation
- `commands/system.rs` - Config path detection (remove Tauri annotations)

---

## CLI Interface

```bash
wezterm-settings              # Launch TUI
wezterm-settings --help       # Show help
wezterm-settings colors       # Jump to colors panel
wezterm-settings --config-dir ~/.config/wezterm  # Custom path
wezterm-settings --export     # Print current config as JSON
wezterm-settings --import config.json  # Import from JSON
```

---

## Success Criteria

1. Binary size < 5MB
2. Startup time < 100ms
3. Works over SSH
4. All settings round-trip correctly (load → edit → save → reload)
5. Generated Lua loads in WezTerm without errors

---

## Reference TUI Projects

No existing TUI specifically for Lua config editing was found, but these Rust TUI projects serve as excellent architectural references:

### Best References for Settings/Config UIs

| Project | Stars | Why It's Relevant |
|---------|-------|-------------------|
| [lazygit](https://github.com/jesseduffield/lazygit) | 68k+ | Gold standard for TUI UX - panels, vim bindings, navigation patterns (Go, but great UX reference) |
| [gitui](https://github.com/extrawurst/gitui) | 18k+ | Rust TUI for git - similar multi-panel layout, keyboard-driven |
| [bottom](https://github.com/ClementTsang/bottom) | 10k+ | System monitor - excellent example of tabbed panels in ratatui |
| [spotify-player](https://github.com/aome510/spotify-player) | 5k+ | Full-featured ratatui app with config management |

### Relevant from awesome-ratatui

**Widget Libraries** (for building our settings UI):
- `tui-input` - Text input widget (for hex color input, font names)
- `tui-textarea` - Multi-line text (if needed for Lua preview)
- `tui-tree-widget` - Tree view (for nested settings)
- `tui-widget-list` - Scrollable lists (for font selection, color lists)

**Settings/Config-Adjacent Apps**:
- [gpg-tui](https://github.com/orhun/gpg-tui) - Key management TUI - similar "browse and edit" pattern
- [passepartui](https://github.com/kardwen/passepartui) - Password manager TUI - handles sensitive config
- [chamber](https://github.com/mikeleppane/chamber) - Secrets manager TUI

### Key UX Patterns to Adopt

From lazygit/gitui:
1. **Panel-based layout**: Sidebar for categories, main panel for editing
2. **Vim + Arrow key navigation**: `j/k/h/l` plus arrows
3. **Context-sensitive help**: Show relevant keybindings at bottom
4. **Confirmation dialogs**: For destructive actions (overwrite config)
5. **Status bar**: Show current file, unsaved changes indicator

### Future Plugin Manager Reference

When implementing plugin management, use:
- [awesome-wezterm](https://github.com/michaelbrusegard/awesome-wezterm) - Curated plugin list
- Similar to how `lazygit` handles git operations, we'll handle plugin install/update/remove
