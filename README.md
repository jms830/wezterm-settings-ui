# WezTerm Settings GUI

A modern GUI application for managing WezTerm terminal configuration and plugins.

## Features (Planned)

- **Visual Settings Editor**: Configure WezTerm options without editing Lua files
  - Appearance (colors, fonts, backgrounds, transparency)
  - Keybindings with visual key capture
  - Tab bar and window decorations
  - GPU and performance settings
  
- **Plugin Manager**: Easy installation and management of WezTerm plugins
  - Browse available plugins from GitHub
  - One-click install/uninstall
  - Plugin configuration UI
  
- **Config Import/Export**: 
  - Import existing WezTerm configs
  - Export to clean Lua configuration
  - Preset themes and configurations

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
