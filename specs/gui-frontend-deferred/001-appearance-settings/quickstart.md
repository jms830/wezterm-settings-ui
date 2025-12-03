# Quickstart: Appearance Settings Development

**Feature**: 001-appearance-settings  
**Date**: 2024-11-28

## Prerequisites

Before starting development, ensure you have:

- **Node.js** 18+ (`node --version`)
- **Rust** 1.70+ (`rustc --version`)
- **Cargo** (`cargo --version`)
- **Platform dependencies**:
  - **Linux**: `sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`
  - **Windows**: WebView2 (usually pre-installed on Windows 10/11)
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)

## Setup

### 1. Clone & Install

```bash
cd /path/to/wezterm-settings-ui
git checkout 001-appearance-settings
npm install
```

### 2. Add Rust Dependencies

Edit `src-tauri/Cargo.toml` and add:

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tera = "1.20"           # Lua template generation
fontdb = "0.23"         # Font enumeration
dirs = "6.0"            # Config path detection
regex = "1"             # Config import
```

### 3. Add Frontend Dependencies

```bash
npm install react-colorful
```

### 4. Run Development Server

```bash
npm run tauri dev
```

This starts both:
- Vite dev server (frontend) at http://localhost:1420
- Tauri app with hot reload

## Project Structure

```
wezterm-settings-gui/
├── src/                          # React frontend
│   ├── components/
│   │   ├── settings/             # Settings panel components
│   │   └── common/               # Reusable UI components
│   ├── hooks/                    # React hooks
│   ├── services/                 # Tauri command wrappers
│   └── types/                    # TypeScript interfaces
├── src-tauri/                    # Rust backend
│   └── src/
│       ├── commands/             # Tauri IPC commands
│       ├── lua/                  # Lua generation
│       └── models/               # Data structs
└── specs/001-appearance-settings/
    ├── spec.md                   # Feature specification
    ├── plan.md                   # Implementation plan
    ├── research.md               # Technology decisions
    ├── data-model.md             # Entity schemas
    └── contracts/                # API contracts
```

## Key Files to Create

### Backend (Rust)

1. **`src-tauri/src/models/config.rs`** - Config structs matching data-model.md
2. **`src-tauri/src/commands/config.rs`** - Tauri commands from contracts/
3. **`src-tauri/src/lua/generator.rs`** - Tera template rendering
4. **`src-tauri/src/lua/templates/`** - Lua template files

### Frontend (React)

1. **`src/types/config.ts`** - TypeScript interfaces from data-model.md
2. **`src/services/tauri.ts`** - invoke() wrappers for all commands
3. **`src/components/settings/ColorSettings.tsx`** - Color editor UI
4. **`src/components/settings/FontSettings.tsx`** - Font selector UI

## Development Workflow

### 1. Start with P1 User Stories

Focus on Color Scheme and Font Settings first (P1 priority):

```
User Story 1: Configure Color Scheme
User Story 2: Configure Font Settings
```

### 2. Build Backend First

Implement Tauri commands in this order:
1. `get_config_path` - Test path detection
2. `get_monospace_fonts` - Test font enumeration
3. `get_config` / `save_config` - Core functionality

### 3. Build Frontend Second

Create components after backend commands work:
1. `ColorSettings.tsx` with `react-colorful`
2. `FontSettings.tsx` with dropdown
3. Wire up to Tauri commands

### 4. Test in WezTerm

After each save:
1. Open WezTerm
2. Verify config loads without errors
3. Check visual changes applied

## Common Tasks

### Adding a New Setting

1. Add field to `data-model.md` entity
2. Add to Rust struct in `models/config.rs`
3. Add to TypeScript interface in `types/config.ts`
4. Update Lua template in `lua/templates/`
5. Add UI control in appropriate settings component

### Testing Lua Output

```bash
# Validate generated Lua syntax
luacheck ~/.config/wezterm/

# Or manually test in WezTerm
wezterm start -- echo "Config loaded successfully"
```

### Debugging Tauri Commands

In Rust:
```rust
#[tauri::command]
fn my_command() -> Result<String, String> {
    println!("Debug: command called");  // Shows in terminal
    // ...
}
```

In TypeScript:
```typescript
const result = await invoke('my_command');
console.log('Result:', result);  // Shows in browser devtools
```

## Useful Commands

```bash
# Run frontend only (no Tauri)
npm run dev

# Build release
npm run tauri build

# Check Rust compilation
cd src-tauri && cargo check

# Format Rust code
cd src-tauri && cargo fmt

# Run Rust tests
cd src-tauri && cargo test
```

## Reference Materials

- [Tauri v2 Docs](https://v2.tauri.app/)
- [WezTerm Config Reference](https://wezfurlong.org/wezterm/config/lua/config/index.html)
- [react-colorful Docs](https://omgovich.github.io/react-colorful/)
- [Tera Template Docs](https://keats.github.io/tera/docs/)

## Troubleshooting

### "Tauri API not available"
- Ensure running via `npm run tauri dev`, not just `npm run dev`

### Font list empty
- Check `fontdb` is in Cargo.toml dependencies
- Verify system fonts exist in standard locations

### Lua syntax errors in WezTerm
- Run `luacheck` on generated files
- Check template escaping for strings with quotes

### Config not saving
- Check file permissions on `~/.config/wezterm/`
- Verify config directory exists
