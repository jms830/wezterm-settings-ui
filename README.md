# WezTerm Settings GUI

A modern GUI application for managing WezTerm terminal configuration and plugins.

## Installation Goals

This GUI is designed to be:

1. **Launchable via terminal command**: `wezterm-settings` or `wezterm settings`
2. **Installable as a WezTerm plugin**: Easy one-command installation from within WezTerm
3. **Standalone application**: Works independently without requiring WezTerm to be running

### Future Installation Methods (Planned)

```bash
# Option 1: System-wide installation
cargo install wezterm-settings-gui

# Option 2: WezTerm plugin (future)
# From WezTerm command palette or config:
wezterm.plugin.require("https://github.com/user/wezterm-settings-gui")

# Option 3: Package managers (future)
brew install wezterm-settings-gui  # macOS
apt install wezterm-settings-gui   # Debian/Ubuntu
```

### Launch Command

Once installed, launch with:
```bash
wezterm-settings          # Open the GUI
wezterm-settings --help   # Show help
wezterm-settings colors   # Jump directly to colors section
```

## Features

- **Visual Settings Editor**: Configure WezTerm options without editing Lua files
  - Appearance (colors, fonts, backgrounds, transparency)
  - Tab bar and window decorations
  - Cursor settings
  - GPU and performance settings
  
- **Basic Keybindings**: Configure common keyboard shortcuts
  - Tab operations (spawn, close, navigate, rename)
  - Pane operations (split, navigate, close)
  - Copy/paste
  - Window management
  - Command palette entries ("Rename Current Tab", "Reset Tab Title")
  
- **Config Import/Export**: 
  - Import existing WezTerm configs
  - Export to clean Lua configuration
  - Preset themes and configurations

### Keybindings Limitations

This TUI manages **basic keybindings** for built-in WezTerm actions. The following are **not supported** and must be edited manually in Lua:

- Custom event handlers with complex logic
- Plugin-specific keybindings (e.g., resurrect, smart-splits)
- Advanced action callbacks
- Custom command palette entries beyond the built-in ones

The TUI generates proper event handlers for:
- **"Rename Current Tab"** - Opens a prompt to rename the tab (persists until reset)
- **"Reset Tab Title"** - Restores automatic tab naming

These appear in the WezTerm command palette (Ctrl+Shift+P) with user-friendly names.

### Plugin Management (Planned)

- Browse available plugins from GitHub
- One-click install/uninstall
- Plugin configuration UI

## Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Tauri v2 (Rust)
- **Config Format**: Lua (WezTerm native)

## Development

### Prerequisites

- Node.js 18+
- Rust (latest stable)
- Platform-specific dependencies for Tauri:
  - **Linux**: `sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`
  - **Windows**: WebView2 (usually pre-installed on Windows 10/11)
  - **macOS**: Xcode Command Line Tools

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Project Structure

```
wezterm-settings-gui/
├── src/                    # React frontend
├── src-tauri/              # Rust backend
│   └── src/
│       ├── lib.rs          # Main Tauri application
│       └── ...             # Config parsing, file management
├── reference/              # (gitignored) Reference configs for analysis
└── .specify/               # Feature specs and planning
```

## License

MIT
