# Research: Appearance Settings Editor

**Feature**: 001-appearance-settings  
**Date**: 2024-11-28

## Summary

This document resolves all NEEDS CLARIFICATION items from the implementation plan through targeted research on technology choices.

---

## 1. Lua Generation Strategy

**Question**: Hand-crafted string templates vs Lua AST library vs template engine?

### Decision: **Tera template engine**

### Rationale
For generating 5-6 small Lua config files, Tera provides the best balance of maintainability and correctness:

1. **Maintainability**: Templates are separate, readable files - easy to modify without Rust recompilation
2. **Type Safety**: Rust structs → serde JSON → template context catches errors at compile time
3. **Familiarity**: Jinja2-style syntax is widely known
4. **Correctness**: Structured approach prevents string escaping bugs

### Alternatives Considered

| Approach | Verdict | Why |
|----------|---------|-----|
| Hand-crafted `format!` strings | Rejected | Error-prone, hard to maintain as templates grow |
| `full_moon` Lua AST | Rejected | Over-engineered for pure generation (designed for parsing/linting) |
| Handlebars | Rejected | Similar to Tera but Tera has better Rust ergonomics |

### Implementation Pattern
```toml
# Cargo.toml
[dependencies]
tera = "1.20"
```

```rust
use tera::{Tera, Context};

let mut tera = Tera::default();
tera.add_raw_template("colors.lua", include_str!("../templates/colors.lua"))?;

let mut ctx = Context::new();
ctx.insert("foreground", "#cdd6f4");
ctx.insert("background", "#1f1f28");

let lua = tera.render("colors.lua", &ctx)?;
```

---

## 2. Font Discovery

**Question**: How to enumerate system fonts cross-platform in Tauri?

### Decision: **`fontdb` crate**

### Rationale
`fontdb` is ideal because:
1. **Built-in monospace detection**: `FaceInfo.monospaced` field - no manual heuristics
2. **Simple API**: Designed for enumeration/querying, not rasterization
3. **Cross-platform**: `load_system_fonts()` scans platform-specific directories automatically
4. **Lightweight**: Fewer dependencies than `font-kit`

### Alternatives Considered

| Crate | Verdict | Why |
|-------|---------|-----|
| `font-kit` | Rejected | More complex, designed for font loading/rendering |
| Tauri plugin | N/A | No official plugin exists for font enumeration |

### Platform Coverage
- **Windows**: `C:\Windows\Fonts`
- **macOS**: `/System/Library/Fonts`, `/Library/Fonts`, `~/Library/Fonts`
- **Linux**: `/usr/share/fonts`, `~/.fonts`, `~/.local/share/fonts`

### Implementation Pattern
```toml
# Cargo.toml
[dependencies]
fontdb = "0.23"
```

```rust
use fontdb::Database;
use std::collections::HashSet;

#[tauri::command]
pub fn get_monospace_fonts() -> Vec<String> {
    let mut db = Database::new();
    db.load_system_fonts();
    
    let mut families: HashSet<String> = HashSet::new();
    
    for face in db.faces() {
        if face.monospaced {
            if let Some((family_name, _)) = face.families.first() {
                families.insert(family_name.clone());
            }
        }
    }
    
    let mut result: Vec<String> = families.into_iter().collect();
    result.sort();
    result
}
```

---

## 3. Color Picker Component

**Question**: Which React color picker library for a settings panel?

### Decision: **react-colorful**

### Rationale
1. **Smallest bundle**: 2.8 KB gzipped - 13x smaller than react-color
2. **Zero dependencies**: Reduces vulnerability surface
3. **TypeScript native**: Ships with types, no `@types` package needed
4. **Hex support**: `HexColorPicker` + `HexColorInput` purpose-built for hex colors
5. **Stable**: Used by Storybook, feature-complete despite 2022 last release

### Alternatives Considered

| Library | Verdict | Why |
|---------|---------|-----|
| `react-color` | Rejected | Abandoned (last release May 2020), 13 KB bundle |
| `@uiw/react-color` | Alternative | Good if more picker styles needed, slightly larger |
| Native `<input type=color>` | Rejected | Inconsistent UI, ugly defaults |

### Implementation Pattern
```bash
npm install react-colorful
```

```tsx
import { HexColorPicker, HexColorInput } from "react-colorful";

function ColorSetting({ value, onChange }: { value: string; onChange: (hex: string) => void }) {
  return (
    <div className="color-setting">
      <HexColorPicker color={value} onChange={onChange} />
      <HexColorInput color={value} onChange={onChange} prefixed />
    </div>
  );
}
```

---

## 4. Config Directory Detection

**Question**: How to find WezTerm config path cross-platform?

### Decision: **`dirs` crate with custom WezTerm logic**

### Rationale
WezTerm uses `~/.config/wezterm` on ALL platforms (not standard OS config dirs), so we need custom logic using `dirs::home_dir()`.

### WezTerm Config Search Order
1. `$WEZTERM_CONFIG_FILE` environment variable
2. `$XDG_CONFIG_HOME/wezterm/wezterm.lua` (if XDG set)
3. `~/.config/wezterm/wezterm.lua`
4. `~/.wezterm.lua` (legacy fallback)

### Platform Paths

| Platform | WezTerm Config Dir |
|----------|-------------------|
| Linux | `~/.config/wezterm/` or `$XDG_CONFIG_HOME/wezterm/` |
| macOS | `~/.config/wezterm/` |
| Windows | `%USERPROFILE%\.config\wezterm\` |

### Implementation Pattern
```toml
# Cargo.toml
[dependencies]
dirs = "6.0"
```

```rust
use std::path::PathBuf;

pub fn get_wezterm_config_dir() -> Result<PathBuf, String> {
    // Check WEZTERM_CONFIG_FILE env var first
    if let Ok(config_file) = std::env::var("WEZTERM_CONFIG_FILE") {
        if let Some(parent) = PathBuf::from(config_file).parent() {
            if parent.exists() {
                return Ok(parent.to_path_buf());
            }
        }
    }

    // Check XDG_CONFIG_HOME
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        let path = PathBuf::from(xdg).join("wezterm");
        if path.exists() {
            return Ok(path);
        }
    }

    // Default: ~/.config/wezterm
    dirs::home_dir()
        .map(|h| h.join(".config").join("wezterm"))
        .ok_or_else(|| "Could not determine home directory".to_string())
}
```

---

## 5. Lua Parsing for Config Import

**Question**: Full Lua parser or regex extraction for importing existing configs?

### Decision: **Regex extraction for MVP, defer full parsing**

### Rationale
1. **MVP Scope**: For P3 priority "Import Existing Config", we only need to extract known values
2. **Complexity**: Full Lua parsing (`full_moon` or `mlua`) adds significant complexity
3. **80/20 Rule**: Regex can handle the modular config structure (simple key-value tables)
4. **Future Option**: Can add `full_moon` later if users need complex config import

### Approach for MVP
- Parse only modular config files (`colors/custom.lua`, `config/appearance.lua`, etc.)
- Use regex patterns for Lua table syntax: `key = 'value'`, `key = number`
- Warn user about unsupported patterns (functions, complex expressions)

### Implementation Pattern
```rust
use regex::Regex;

pub fn extract_color_value(lua_content: &str, key: &str) -> Option<String> {
    // Match: key = '#hexcolor' or key = "#hexcolor"
    let pattern = format!(r#"{}\s*=\s*['"]([#]?[0-9a-fA-F]{{6}})['"]"#, key);
    let re = Regex::new(&pattern).ok()?;
    re.captures(lua_content)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}
```

### Future Enhancement
If users request full config import, consider `full_moon` crate for AST-based extraction.

---

## Dependencies Summary

### Rust (src-tauri/Cargo.toml)
```toml
[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tera = "1.20"           # Lua template generation
fontdb = "0.23"         # Font enumeration
dirs = "6.0"            # Config path detection
regex = "1"             # Config import (MVP)
```

### Frontend (package.json)
```json
{
  "dependencies": {
    "react-colorful": "^5.6.1"
  }
}
```

---

## Open Items (Deferred)

| Item | Priority | Notes |
|------|----------|-------|
| Full Lua parser | Low | Add `full_moon` if complex import needed |
| Live preview | Medium | Investigate WezTerm IPC for real-time preview |
| Backup strategy | Medium | Implement timestamped backups before save |

---

## Future Reference: Plugin Manager

When implementing the plugin manager feature, use this curated list as a reference:

**Awesome WezTerm**: https://github.com/michaelbrusegard/awesome-wezterm

This repository contains:
- Curated list of WezTerm plugins
- Plugin categories (themes, utilities, integrations)
- Links to plugin repositories
- Installation instructions

### Plugin Manager Considerations (Future)

1. **Plugin Discovery**: Scrape/index awesome-wezterm list or maintain our own registry
2. **Installation**: Clone plugin repos to `~/.config/wezterm/plugins/`
3. **WezTerm Plugin API**: Use `wezterm.plugin.require()` for official plugin loading
4. **Updates**: Check for plugin updates via git or GitHub API
5. **Configuration**: Generate Lua code to require/configure each plugin
