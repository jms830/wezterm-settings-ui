# Tasks: Appearance Settings Editor

**Input**: Design documents from `/specs/001-appearance-settings/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not explicitly requested - omitting test tasks (can be added later if needed)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Frontend**: `src/` (React TypeScript)
- **Backend**: `src-tauri/src/` (Rust)
- **Lua Templates**: `src-tauri/templates/` (Tera templates)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and dependency configuration

- [ ] T001 Add Rust dependencies (tera, fontdb, dirs, regex) to src-tauri/Cargo.toml
- [ ] T002 Add frontend dependency (react-colorful) via npm install
- [ ] T003 [P] Create TypeScript interfaces from data-model in src/types/config.ts
- [ ] T004 [P] Create directory structure for Rust modules: src-tauri/src/commands/, src-tauri/src/lua/, src-tauri/src/models/
- [ ] T005 [P] Create directory structure for React components: src/components/settings/, src/components/common/, src/components/layout/
- [ ] T006 [P] Create Lua template directory: src-tauri/templates/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Backend Foundation

- [ ] T007 Create Rust config structs (AppearanceConfig, ColorScheme, FontConfig, etc.) in src-tauri/src/models/config.rs
- [ ] T008 Create Rust models module file in src-tauri/src/models/mod.rs
- [ ] T009 Implement get_wezterm_config_dir() function using dirs crate in src-tauri/src/commands/system.rs
- [ ] T010 [P] Implement get_config_path Tauri command in src-tauri/src/commands/system.rs
- [ ] T011 [P] Implement get_system_info Tauri command in src-tauri/src/commands/system.rs
- [ ] T012 Create commands module file in src-tauri/src/commands/mod.rs
- [ ] T013 Create Lua generator base with Tera setup in src-tauri/src/lua/generator.rs
- [ ] T014 Create lua module file in src-tauri/src/lua/mod.rs
- [ ] T015 Register all Tauri commands in src-tauri/src/lib.rs invoke_handler

### Frontend Foundation

- [ ] T016 Create Tauri command wrapper service in src/services/tauri.ts
- [ ] T017 Create useConfig hook for state management in src/hooks/useConfig.ts
- [ ] T018 Create useWezTermPath hook for config directory in src/hooks/useWezTermPath.ts
- [ ] T019 [P] Create Sidebar component shell in src/components/layout/Sidebar.tsx
- [ ] T020 [P] Create SettingsPanel container component in src/components/layout/SettingsPanel.tsx
- [ ] T021 Update App.tsx with basic layout structure (Sidebar + SettingsPanel) in src/App.tsx
- [ ] T022 [P] Create NumberInput common component in src/components/common/NumberInput.tsx
- [ ] T023 [P] Create Select dropdown common component in src/components/common/Select.tsx

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Configure Color Scheme (Priority: P1) 🎯 MVP

**Goal**: Users can customize terminal colors via GUI and save to `colors/custom.lua`

**Independent Test**: Change colors in GUI, save, verify `colors/custom.lua` loads in WezTerm without errors

### Backend for User Story 1

- [ ] T024 [US1] Create Tera template for colors/custom.lua in src-tauri/templates/colors_custom.lua
- [ ] T025 [US1] Implement generate_colors_lua() function in src-tauri/src/lua/generator.rs
- [ ] T026 [US1] Implement validate_color_scheme() validation function in src-tauri/src/commands/config.rs
- [ ] T027 [US1] Implement save_colors() function to write colors/custom.lua in src-tauri/src/commands/config.rs
- [ ] T028 [US1] Implement load_colors() function to read existing colors in src-tauri/src/commands/config.rs
- [ ] T029 [US1] Add get_config Tauri command (colors portion) in src-tauri/src/commands/config.rs
- [ ] T030 [US1] Add save_config Tauri command (colors portion) in src-tauri/src/commands/config.rs
- [ ] T031 [US1] Implement create_backup() for colors file in src-tauri/src/commands/config.rs

### Frontend for User Story 1

- [ ] T032 [P] [US1] Create ColorPicker wrapper component using react-colorful in src/components/common/ColorPicker.tsx
- [ ] T033 [US1] Create ColorSettings component with all color fields in src/components/settings/ColorSettings.tsx
- [ ] T034 [US1] Add ANSI palette (8 colors) editor section in src/components/settings/ColorSettings.tsx
- [ ] T035 [US1] Add brights palette (8 colors) editor section in src/components/settings/ColorSettings.tsx
- [ ] T036 [US1] Add tab bar colors editor section in src/components/settings/ColorSettings.tsx
- [ ] T037 [US1] Wire ColorSettings to useConfig hook and save_config command in src/components/settings/ColorSettings.tsx
- [ ] T038 [US1] Add "Colors" navigation item to Sidebar in src/components/layout/Sidebar.tsx
- [ ] T039 [US1] Add ColorSettings route/view to SettingsPanel in src/components/layout/SettingsPanel.tsx

**Checkpoint**: Color scheme editing is fully functional - can be tested independently in WezTerm

---

## Phase 4: User Story 2 - Configure Font Settings (Priority: P1)

**Goal**: Users can select fonts and adjust size, save to `config/fonts.lua`

**Independent Test**: Select a font, set size, save, verify WezTerm displays the new font

### Backend for User Story 2

- [ ] T040 [US2] Implement get_monospace_fonts() using fontdb in src-tauri/src/commands/system.rs
- [ ] T041 [US2] Create Tera template for config/fonts.lua in src-tauri/templates/config_fonts.lua
- [ ] T042 [US2] Implement generate_fonts_lua() function in src-tauri/src/lua/generator.rs
- [ ] T043 [US2] Implement validate_font_config() validation function in src-tauri/src/commands/config.rs
- [ ] T044 [US2] Add save_config support for fonts in src-tauri/src/commands/config.rs
- [ ] T045 [US2] Add get_config support for fonts (load existing) in src-tauri/src/commands/config.rs
- [ ] T046 [US2] Register get_monospace_fonts command in src-tauri/src/lib.rs

### Frontend for User Story 2

- [ ] T047 [US2] Create FontSettings component in src/components/settings/FontSettings.tsx
- [ ] T048 [US2] Add font family dropdown populated from get_monospace_fonts in src/components/settings/FontSettings.tsx
- [ ] T049 [US2] Add font size NumberInput field in src/components/settings/FontSettings.tsx
- [ ] T050 [US2] Add font weight Select dropdown in src/components/settings/FontSettings.tsx
- [ ] T051 [US2] Add freetype rendering options (advanced section) in src/components/settings/FontSettings.tsx
- [ ] T052 [US2] Wire FontSettings to useConfig hook and save_config command in src/components/settings/FontSettings.tsx
- [ ] T053 [US2] Add "Fonts" navigation item to Sidebar in src/components/layout/Sidebar.tsx
- [ ] T054 [US2] Add FontSettings route/view to SettingsPanel in src/components/layout/SettingsPanel.tsx

**Checkpoint**: Font settings editing is fully functional - can be tested independently in WezTerm

---

## Phase 5: User Story 3 - Configure Window Appearance (Priority: P2)

**Goal**: Users can adjust window styling (opacity, padding, tab bar) and save to `config/appearance.lua`

**Independent Test**: Adjust window opacity to 0.9, save, verify WezTerm shows transparency

### Backend for User Story 3

- [ ] T055 [US3] Create Tera template for config/appearance.lua in src-tauri/templates/config_appearance.lua
- [ ] T056 [US3] Implement generate_appearance_lua() function in src-tauri/src/lua/generator.rs
- [ ] T057 [US3] Implement validate_window_config() validation function in src-tauri/src/commands/config.rs
- [ ] T058 [US3] Add save_config support for window settings in src-tauri/src/commands/config.rs
- [ ] T059 [US3] Add get_config support for window settings (load existing) in src-tauri/src/commands/config.rs

### Frontend for User Story 3

- [ ] T060 [US3] Create WindowSettings component in src/components/settings/WindowSettings.tsx
- [ ] T061 [US3] Add window_background_opacity slider (0.0-1.0) in src/components/settings/WindowSettings.tsx
- [ ] T062 [US3] Add window_padding inputs (left, right, top, bottom) in src/components/settings/WindowSettings.tsx
- [ ] T063 [US3] Add window_decorations Select dropdown in src/components/settings/WindowSettings.tsx
- [ ] T064 [US3] Add tab bar settings section (enable, hide_if_one, fancy, max_width) in src/components/settings/WindowSettings.tsx
- [ ] T065 [US3] Add inactive_pane_hsb controls in src/components/settings/WindowSettings.tsx
- [ ] T066 [US3] Wire WindowSettings to useConfig hook and save_config command in src/components/settings/WindowSettings.tsx
- [ ] T067 [US3] Add "Window" navigation item to Sidebar in src/components/layout/Sidebar.tsx
- [ ] T068 [US3] Add WindowSettings route/view to SettingsPanel in src/components/layout/SettingsPanel.tsx

**Checkpoint**: Window settings editing is fully functional - can be tested independently in WezTerm

---

## Phase 6: User Story 4 - Configure Background Images (Priority: P2)

**Goal**: Users can add/manage background images and configure backdrop settings

**Independent Test**: Add an image, enable backdrops, save, verify image appears in WezTerm

### Backend for User Story 4

- [ ] T069 [US4] Implement list_backdrop_images() command in src-tauri/src/commands/config.rs
- [ ] T070 [US4] Implement add_backdrop_image() command (copy to backdrops/) in src-tauri/src/commands/config.rs
- [ ] T071 [US4] Implement remove_backdrop_image() command in src-tauri/src/commands/config.rs
- [ ] T072 [US4] Create Tera template for backdrop initialization in src-tauri/templates/utils_backdrops.lua
- [ ] T073 [US4] Implement generate_backdrops_lua() function in src-tauri/src/lua/generator.rs
- [ ] T074 [US4] Add save_config support for backdrop settings in src-tauri/src/commands/config.rs
- [ ] T075 [US4] Register backdrop commands in src-tauri/src/lib.rs

### Frontend for User Story 4

- [ ] T076 [US4] Create BackdropSettings component in src/components/settings/BackdropSettings.tsx
- [ ] T077 [US4] Add backdrop enable toggle in src/components/settings/BackdropSettings.tsx
- [ ] T078 [US4] Add image gallery showing current backdrops in src/components/settings/BackdropSettings.tsx
- [ ] T079 [US4] Add "Add Image" button with file picker integration in src/components/settings/BackdropSettings.tsx
- [ ] T080 [US4] Add image delete functionality in src/components/settings/BackdropSettings.tsx
- [ ] T081 [US4] Add overlay_opacity slider in src/components/settings/BackdropSettings.tsx
- [ ] T082 [US4] Add random_on_start toggle in src/components/settings/BackdropSettings.tsx
- [ ] T083 [US4] Add focus_color picker for focus mode in src/components/settings/BackdropSettings.tsx
- [ ] T084 [US4] Wire BackdropSettings to useConfig hook in src/components/settings/BackdropSettings.tsx
- [ ] T085 [US4] Add "Backgrounds" navigation item to Sidebar in src/components/layout/Sidebar.tsx
- [ ] T086 [US4] Add BackdropSettings route/view to SettingsPanel in src/components/layout/SettingsPanel.tsx

**Checkpoint**: Background image management is fully functional - can be tested independently in WezTerm

---

## Phase 7: User Story 5 - Configure Cursor Appearance (Priority: P3)

**Goal**: Users can customize cursor style and blink settings

**Independent Test**: Change cursor to BlinkingBar, save, verify in WezTerm

### Backend for User Story 5

- [ ] T087 [US5] Add cursor config fields to appearance.lua template in src-tauri/templates/config_appearance.lua
- [ ] T088 [US5] Implement validate_cursor_config() validation function in src-tauri/src/commands/config.rs
- [ ] T089 [US5] Add save_config support for cursor settings in src-tauri/src/commands/config.rs
- [ ] T090 [US5] Add get_config support for cursor settings in src-tauri/src/commands/config.rs

### Frontend for User Story 5

- [ ] T091 [US5] Create CursorSettings component in src/components/settings/CursorSettings.tsx
- [ ] T092 [US5] Add default_cursor_style Select dropdown (6 options) in src/components/settings/CursorSettings.tsx
- [ ] T093 [US5] Add cursor_blink_rate NumberInput in src/components/settings/CursorSettings.tsx
- [ ] T094 [US5] Add cursor_blink_ease_in/out Select dropdowns in src/components/settings/CursorSettings.tsx
- [ ] T095 [US5] Add animation_fps NumberInput in src/components/settings/CursorSettings.tsx
- [ ] T096 [US5] Wire CursorSettings to useConfig hook in src/components/settings/CursorSettings.tsx
- [ ] T097 [US5] Add "Cursor" navigation item to Sidebar in src/components/layout/Sidebar.tsx
- [ ] T098 [US5] Add CursorSettings route/view to SettingsPanel in src/components/layout/SettingsPanel.tsx

**Checkpoint**: Cursor settings editing is fully functional - can be tested independently in WezTerm

---

## Phase 8: User Story 6 - Import Existing Config (Priority: P3)

**Goal**: Users can import their existing WezTerm config into the GUI

**Independent Test**: Point to existing config, verify values populate in GUI

### Backend for User Story 6

- [ ] T099 [US6] Implement extract_color_value() regex parser in src-tauri/src/lua/parser.rs
- [ ] T100 [US6] Implement extract_number_value() regex parser in src-tauri/src/lua/parser.rs
- [ ] T101 [US6] Implement extract_string_value() regex parser in src-tauri/src/lua/parser.rs
- [ ] T102 [US6] Create parser module file in src-tauri/src/lua/mod.rs (add parser)
- [ ] T103 [US6] Implement parse_colors_lua() to extract ColorScheme in src-tauri/src/lua/parser.rs
- [ ] T104 [US6] Implement parse_fonts_lua() to extract FontConfig in src-tauri/src/lua/parser.rs
- [ ] T105 [US6] Implement parse_appearance_lua() to extract WindowConfig/CursorConfig in src-tauri/src/lua/parser.rs
- [ ] T106 [US6] Update get_config to use parsers for loading existing config in src-tauri/src/commands/config.rs

### Frontend for User Story 6

- [ ] T107 [US6] Add "Import Config" button to app header/menu in src/App.tsx
- [ ] T108 [US6] Create import confirmation dialog component in src/components/common/ImportDialog.tsx
- [ ] T109 [US6] Show warning for unsupported patterns during import in src/components/common/ImportDialog.tsx
- [ ] T110 [US6] Wire import to get_config command and populate state in src/hooks/useConfig.ts

**Checkpoint**: Config import is functional - existing WezTerm users can import their settings

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T111 [P] Add GPU settings (front_end, webgpu_power_preference, max_fps) to WindowSettings in src/components/settings/WindowSettings.tsx
- [ ] T112 [P] Create wezterm.lua main file template in src-tauri/templates/wezterm.lua
- [ ] T113 Generate main wezterm.lua that imports all modules in src-tauri/src/lua/generator.rs
- [ ] T114 [P] Add config/init.lua template for module composition in src-tauri/templates/config_init.lua
- [ ] T115 [P] Implement validate_config Tauri command (full validation) in src-tauri/src/commands/config.rs
- [ ] T116 [P] Implement restore_backup Tauri command in src-tauri/src/commands/config.rs
- [ ] T117 Add error handling UI for validation errors in src/components/common/ErrorDisplay.tsx
- [ ] T118 Add success toast/notification for save operations in src/App.tsx
- [ ] T119 [P] Add CSS styling for settings panels in src/App.css
- [ ] T120 Run cargo clippy and fix any warnings in src-tauri/
- [ ] T121 Run TypeScript strict mode check and fix errors
- [ ] T122 Manual WezTerm testing with generated config
- [ ] T123 Update README.md with usage instructions

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phases 3-8)**: All depend on Foundational phase completion
  - US1 (Colors) and US2 (Fonts) are P1 - do first
  - US3 (Window) and US4 (Backdrops) are P2 - do after P1s
  - US5 (Cursor) and US6 (Import) are P3 - do last
- **Polish (Phase 9)**: Depends on at least US1 and US2 being complete

### User Story Dependencies

| Story | Depends On | Can Run Parallel With |
|-------|------------|----------------------|
| US1 (Colors) | Foundation only | US2 |
| US2 (Fonts) | Foundation only | US1 |
| US3 (Window) | Foundation only | US4 |
| US4 (Backdrops) | Foundation only | US3 |
| US5 (Cursor) | Foundation only | US6 |
| US6 (Import) | Foundation only | US5 |

### Within Each User Story

1. Backend template → Backend generator → Backend commands
2. Frontend component → Wire to hooks → Add to navigation
3. Story complete before moving to next priority

### Parallel Opportunities

**Phase 1 (Setup):**
- T003, T004, T005, T006 can all run in parallel

**Phase 2 (Foundational):**
- T010, T011 can run in parallel
- T019, T020 can run in parallel
- T022, T023 can run in parallel

**Phase 3+ (User Stories):**
- Once Foundational completes, US1 and US2 can start in parallel
- US3 and US4 can start in parallel (after P1s if desired)
- US5 and US6 can start in parallel (after P2s if desired)

---

## Parallel Example: User Story 1 (Colors)

```bash
# After T025 (backend template) completes:
# T026, T027, T028, T031 can proceed (different functions)

# After T032 (ColorPicker component) completes:
# T033 can proceed (ColorSettings depends on ColorPicker)

# T034, T035, T036 are all sections within ColorSettings - 
# implement sequentially within the same file
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only)

1. Complete Phase 1: Setup (T001-T006)
2. Complete Phase 2: Foundational (T007-T023)
3. Complete Phase 3: User Story 1 - Colors (T024-T039)
4. **STOP and VALIDATE**: Test color editing in WezTerm
5. Complete Phase 4: User Story 2 - Fonts (T040-T054)
6. **STOP and VALIDATE**: Test font selection in WezTerm
7. Deploy/demo MVP - users can configure colors and fonts!

### Incremental Delivery

| Milestone | Stories | User Value |
|-----------|---------|------------|
| MVP | US1 + US2 | Configure colors and fonts |
| v0.2 | + US3 + US4 | Window styling and backgrounds |
| v0.3 | + US5 + US6 | Cursor settings and config import |
| v1.0 | + Polish | Production-ready release |

### Suggested Task Count per Day (Solo Developer)

- **Day 1**: T001-T015 (Setup + Backend Foundation)
- **Day 2**: T016-T023 (Frontend Foundation)
- **Day 3**: T024-T031 (US1 Backend)
- **Day 4**: T032-T039 (US1 Frontend) → **MVP Colors!**
- **Day 5**: T040-T046 (US2 Backend)
- **Day 6**: T047-T054 (US2 Frontend) → **MVP Fonts!**
- Continue as needed for P2/P3 stories...

---

## Notes

- [P] tasks = different files, no dependencies on incomplete tasks in same phase
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- All Lua templates use Tera syntax (Jinja2-style)
- Color values are hex strings (#RRGGBB)
- Manual WezTerm testing required after each story completes
