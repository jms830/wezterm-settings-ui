# WezTerm Settings GUI Constitution

## Core Principles

### I. Modular Lua Output
All generated WezTerm configuration MUST follow Kevin Silvester's modular structure:
- Separate files for: `appearance.lua`, `fonts.lua`, `bindings.lua`, `general.lua`, `colors/custom.lua`
- Main `wezterm.lua` only imports and composes modules
- Each module is self-contained and independently editable
- Users can manually edit generated Lua without breaking GUI sync

### II. Non-Destructive Config Management
- NEVER overwrite user's existing config without explicit confirmation
- Always create backups before modifying existing configs
- Preserve user's manual edits and comments where possible
- Support "export only" mode for users who want to copy-paste

### III. WezTerm Compatibility First
- Generated Lua MUST be valid for WezTerm stable release
- Use WezTerm's documented config options only
- Test generated configs actually work in WezTerm
- Support both WebGpu and OpenGL frontends

### IV. Minimal Maintenance Burden
- Prefer well-maintained dependencies over custom implementations
- Use Tauri's native capabilities over npm packages where possible
- Keep the Rust backend thin - mostly file I/O and Lua generation
- React frontend handles all UI complexity

### V. User Experience Priority
- Settings should have sensible defaults
- Live preview where feasible (colors, fonts)
- Clear descriptions for each setting
- Group related settings logically

## Technology Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Tauri v2 (Rust)
- **Config Format**: Lua (WezTerm native)
- **State**: Local filesystem (no cloud, no accounts)

## Quality Gates

- All Rust code must compile without warnings
- TypeScript strict mode enabled
- Generated Lua must pass luacheck
- Manual testing in WezTerm required before merge

## Governance

Constitution supersedes implementation convenience. Any deviation requires:
1. Documentation of why deviation is necessary
2. Plan to return to compliance if temporary

**Version**: 1.0.0 | **Ratified**: 2024-11-28 | **Last Amended**: 2024-11-28
