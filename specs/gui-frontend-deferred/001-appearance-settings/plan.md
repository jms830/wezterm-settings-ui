# Implementation Plan: Appearance Settings Editor

**Branch**: `001-appearance-settings` | **Date**: 2024-11-28 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-appearance-settings/spec.md`

## Summary

Build a visual settings editor for WezTerm appearance configuration (colors, fonts, window styling, backgrounds, cursor). The GUI uses React/TypeScript frontend with Tauri v2 Rust backend. Output is modular Lua config files following Kevin Silvester's structure pattern (`config/appearance.lua`, `config/fonts.lua`, `colors/custom.lua`, etc.).

## Technical Context

**Language/Version**: TypeScript 5.8+ (frontend), Rust 2021 edition (backend)  
**Primary Dependencies**: React 19, Tauri 2.x, Vite 7.x, serde (Rust)  
**Storage**: Local filesystem - WezTerm config directory (`~/.config/wezterm` on Linux/Mac, `%USERPROFILE%\.config\wezterm` on Windows)  
**Testing**: Vitest (frontend), cargo test (backend), manual WezTerm validation  
**Target Platform**: Windows, Linux, macOS (desktop only)  
**Project Type**: Desktop application (Tauri - web frontend + Rust backend)  
**Performance Goals**: UI responsive (<100ms interactions), config generation <1s  
**Constraints**: Must generate valid WezTerm Lua, must not corrupt existing configs  
**Scale/Scope**: Single-user desktop app, ~15-20 settings screens, ~50 configurable options

### NEEDS CLARIFICATION (to resolve in Phase 0)

1. **Lua Generation Strategy**: Hand-crafted string templates vs Lua AST library in Rust?
2. **Font Discovery**: How to enumerate system fonts cross-platform in Tauri?
3. **Color Picker Component**: Use existing React library or build custom?
4. **Config Directory Detection**: Platform-specific logic for finding WezTerm config path?
5. **Lua Parsing for Import**: Do we need full Lua parser or regex extraction sufficient for MVP?

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Modular Lua Output | PASS | Design explicitly uses separate files per Kevin's structure |
| II. Non-Destructive Config | PASS | FR-007 requires backup before overwrite |
| III. WezTerm Compatibility | PASS | SC-003 requires config loads without errors |
| IV. Minimal Maintenance | PASS | Using Tauri native + established React patterns |
| V. User Experience | PASS | P1 stories focus on core UX (colors, fonts) |

**Gate Status**: PASS - No violations. Proceed to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-appearance-settings/
├── plan.md              # This file
├── research.md          # Phase 0 output - technology decisions
├── data-model.md        # Phase 1 output - entity schemas
├── quickstart.md        # Phase 1 output - dev setup guide
├── contracts/           # Phase 1 output - Tauri command interfaces
└── tasks.md             # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Tauri Desktop Application Structure
src/                          # React frontend
├── components/
│   ├── settings/
│   │   ├── ColorSettings.tsx     # Color scheme editor
│   │   ├── FontSettings.tsx      # Font configuration
│   │   ├── WindowSettings.tsx    # Window appearance
│   │   ├── CursorSettings.tsx    # Cursor configuration
│   │   └── BackdropSettings.tsx  # Background images
│   ├── common/
│   │   ├── ColorPicker.tsx
│   │   ├── NumberInput.tsx
│   │   └── Select.tsx
│   └── layout/
│       ├── Sidebar.tsx
│       └── SettingsPanel.tsx
├── hooks/
│   ├── useConfig.ts              # Config state management
│   └── useWezTermPath.ts         # Config directory detection
├── services/
│   └── tauri.ts                  # Tauri command wrappers
├── types/
│   └── config.ts                 # TypeScript interfaces
├── App.tsx
└── main.tsx

src-tauri/                   # Rust backend
├── src/
│   ├── lib.rs                    # Tauri app setup
│   ├── main.rs                   # Entry point
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── config.rs             # Read/write config commands
│   │   └── system.rs             # Font discovery, paths
│   ├── lua/
│   │   ├── mod.rs
│   │   ├── generator.rs          # Lua code generation
│   │   └── templates.rs          # Lua file templates
│   └── models/
│       ├── mod.rs
│       └── config.rs             # Config structs (serde)
└── Cargo.toml

tests/                       # Test files
├── frontend/                # Vitest tests
└── backend/                 # Rust tests (in src-tauri/src/)
```

**Structure Decision**: Tauri standard structure with `src/` for React frontend and `src-tauri/` for Rust backend. Settings components organized by feature area. Lua generation logic isolated in `src-tauri/src/lua/` module.

## Complexity Tracking

> No violations detected. Table not required.

---

## Post-Design Constitution Check

*Re-evaluated after Phase 1 design completion.*

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Modular Lua Output | **PASS** | `data-model.md` maps entities to separate files: `colors/custom.lua`, `config/fonts.lua`, `config/appearance.lua` |
| II. Non-Destructive Config | **PASS** | `contracts/tauri-commands.md` includes `create_backup` command, `SaveResult` returns `backups_created` |
| III. WezTerm Compatibility | **PASS** | Research chose Tera templates for valid Lua syntax; `validate_config` command ensures correctness |
| IV. Minimal Maintenance | **PASS** | Dependencies are well-maintained: Tera (1.20), fontdb (0.23), react-colorful (5.6) |
| V. User Experience | **PASS** | `quickstart.md` provides clear dev guide; data-model has sensible defaults |

### Quality Gates Check

| Gate | Status | Notes |
|------|--------|-------|
| Rust compiles without warnings | PENDING | Implementation phase |
| TypeScript strict mode | PASS | Already enabled in tsconfig.json |
| Lua passes luacheck | PENDING | Implementation phase |
| WezTerm testing | PENDING | Implementation phase |

**Post-Design Status**: PASS - Ready for Phase 2 (task breakdown).
