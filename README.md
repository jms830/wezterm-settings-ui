# WezTerm Settings UI

A modern TUI for managing WezTerm terminal configuration. Configure colors, fonts, keybindings, and more without editing Lua files.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)

## Quick Install

**Two simple steps:**

```bash
# Step 1: Install the TUI binary
cargo install --git https://github.com/jms830/wezterm-settings-ui wezterm-settings-tui

# Step 2: Add plugin to your wezterm.lua
```

Add this to your `~/.config/wezterm/wezterm.lua`:

```lua
local wezterm = require("wezterm")
local config = wezterm.config_builder()

-- Load the settings plugin
local settings = wezterm.plugin.require("https://github.com/jms830/wezterm-settings-ui")
settings.apply_to_config(config)

-- ... rest of your config ...

return config
```

**That's it!** Now you can:
- Press `Ctrl+Shift+,` to open settings
- Or use the command palette (`Ctrl+Shift+P`) and search "Settings"

## Updating

### Quick Update (Recommended)
```bash
# Self-update command - checks and installs latest version
wezterm-settings-tui update
```

### Check for Updates
```bash
# Just check if updates are available
wezterm-settings-tui check-update
```

### Manual Update
```bash
# Update the TUI binary manually
cargo install --git https://github.com/jms830/wezterm-settings-ui wezterm-settings-tui --force

# Update the plugin (in WezTerm's debug overlay: Ctrl+Shift+L)
> wezterm.plugin.update_all()
```

## Features

### Visual Settings Editor
Configure WezTerm options without editing Lua files:
- **Colors** - Foreground, background, cursor, ANSI palette
- **Fonts** - Family, size, weight (with system font detection)
- **Window** - Opacity, padding, tab bar, decorations
- **Cursor** - Style, blink rate, animation
- **Keybindings** - Common shortcuts with command palette integration
- **GPU** - Frontend, power preference, max FPS

### Command Palette Integration
The plugin adds these commands to WezTerm's command palette:
- **Settings: Open WezTerm Settings** - Full settings editor
- **Settings: Colors** - Jump to colors panel
- **Settings: Fonts** - Jump to fonts panel
- **Settings: Keybindings** - Jump to keybindings panel

### Generated Lua Config
The TUI generates clean, readable Lua configuration that:
- Uses `wezterm.config_builder()` pattern
- Includes proper event handlers for tab renaming
- Creates backups before overwriting existing config

## Configuration Options

Customize the plugin behavior:

```lua
local settings = wezterm.plugin.require("https://github.com/jms830/wezterm-settings-ui")
settings.apply_to_config(config, {
   -- Change the keybinding (default: Ctrl+Shift+,)
   keybinding = { key = "s", mods = "LEADER" },
   
   -- Or disable the keybinding entirely
   -- keybinding = nil,
   
   -- How to open: "tab", "window", or "pane"
   open_mode = "tab",
   
   -- Show in command palette (default: true)
   command_palette = true,
})
```

## Manual Usage (Without Plugin)

You can also run the TUI directly:

```bash
wezterm-settings-tui              # Open full settings
wezterm-settings-tui colors       # Jump to colors panel
wezterm-settings-tui fonts        # Jump to fonts panel
wezterm-settings-tui keys         # Jump to keybindings panel
wezterm-settings-tui --help       # Show all options
```

## Keybindings in the TUI

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `h` / `←` | Back / collapse |
| `l` / `→` / `Enter` | Select / expand / edit |
| `Tab` | Next field |
| `Shift+Tab` | Previous field |
| `Ctrl+S` | Save all changes |
| `q` / `Esc` | Quit |

## Limitations

This TUI manages **basic settings** and **built-in keybindings**. The following require manual Lua editing:

- Custom event handlers with complex logic
- Plugin-specific keybindings (resurrect, smart-splits, etc.)
- Advanced action callbacks
- Custom domains and multiplexing

## Development

```bash
# Clone the repository
git clone https://github.com/jms830/wezterm-settings-ui
cd wezterm-settings-ui

# Build and run
cargo run

# Run tests
cargo test

# Build release binary
cargo build --release
```

## Project Structure

```
wezterm-settings-ui/
├── plugin/                 # WezTerm Lua plugin
│   └── init.lua           # Plugin entry point
├── src/                   # TUI application (Rust)
│   ├── main.rs           # Entry point
│   ├── app.rs            # Application state
│   └── ui/               # UI panels and widgets
├── src-tauri/            # Shared library (config models, Lua generation)
│   └── src/
│       ├── models/       # Config data structures
│       ├── lua/          # Lua parser and generator
│       └── commands/     # Config operations
└── specs/                # Feature specifications
```

## License

MIT

## Contributing

Contributions welcome! Please read the [SECURITY.md](.github/SECURITY.md) for security-related guidelines.
