# Tauri Commands Contract

**Feature**: 001-appearance-settings  
**Date**: 2024-11-28

## Overview

This document defines the Tauri IPC commands (Rust → TypeScript) for the appearance settings feature. These commands handle config reading, writing, and system queries.

---

## Command Categories

1. **Config Commands** - Read/write WezTerm configuration
2. **System Commands** - Font discovery, path detection
3. **File Commands** - Backup, image management

---

## Config Commands

### `get_config`

Reads the current appearance configuration from disk.

**Signature:**
```rust
#[tauri::command]
async fn get_config() -> Result<AppearanceConfig, String>
```

**TypeScript:**
```typescript
invoke<AppearanceConfig>('get_config')
```

**Returns:**
- `Ok(AppearanceConfig)` - Complete config from disk (or defaults if not found)
- `Err(String)` - Error message if read fails

**Behavior:**
1. Detect WezTerm config directory
2. Read each config file (`colors/custom.lua`, `config/fonts.lua`, etc.)
3. Parse values using regex extraction
4. Return merged config with defaults for missing values

---

### `save_config`

Saves the appearance configuration to disk.

**Signature:**
```rust
#[tauri::command]
async fn save_config(config: AppearanceConfig) -> Result<SaveResult, String>
```

**TypeScript:**
```typescript
invoke<SaveResult>('save_config', { config: AppearanceConfig })
```

**Input:**
```typescript
interface AppearanceConfig {
  colors: ColorScheme;
  fonts: FontConfig;
  window: WindowConfig;
  cursor: CursorConfig;
  backdrop: BackdropConfig;
  gpu: GPUConfig;
}
```

**Returns:**
```typescript
interface SaveResult {
  success: boolean;
  files_written: string[];      // List of files modified
  backups_created: string[];    // List of backup files created
  config_dir: string;           // Path to config directory
}
```

**Behavior:**
1. Validate all config values
2. Create backups of existing files
3. Generate Lua files using Tera templates
4. Write files to WezTerm config directory
5. Return summary of changes

---

### `get_config_path`

Returns the detected WezTerm config directory path.

**Signature:**
```rust
#[tauri::command]
fn get_config_path() -> Result<String, String>
```

**TypeScript:**
```typescript
invoke<string>('get_config_path')
```

**Returns:**
- Path string, e.g., `/home/user/.config/wezterm`

---

### `validate_config`

Validates a config without saving.

**Signature:**
```rust
#[tauri::command]
fn validate_config(config: AppearanceConfig) -> Result<ValidationResult, String>
```

**TypeScript:**
```typescript
invoke<ValidationResult>('validate_config', { config })
```

**Returns:**
```typescript
interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

interface ValidationError {
  field: string;        // e.g., "colors.foreground"
  message: string;      // e.g., "Invalid hex color format"
  value: string;        // The invalid value
}
```

---

## System Commands

### `get_monospace_fonts`

Returns list of installed monospace fonts.

**Signature:**
```rust
#[tauri::command]
fn get_monospace_fonts() -> Vec<String>
```

**TypeScript:**
```typescript
invoke<string[]>('get_monospace_fonts')
```

**Returns:**
- Sorted array of font family names
- Example: `["Cascadia Code", "Fira Code", "JetBrainsMono Nerd Font", "Menlo"]`

**Behavior:**
1. Use `fontdb` to scan system font directories
2. Filter to monospace fonts only
3. Deduplicate by family name
4. Sort alphabetically

---

### `get_system_info`

Returns system information relevant to configuration.

**Signature:**
```rust
#[tauri::command]
fn get_system_info() -> SystemInfo
```

**TypeScript:**
```typescript
invoke<SystemInfo>('get_system_info')
```

**Returns:**
```typescript
interface SystemInfo {
  platform: 'windows' | 'macos' | 'linux';
  config_dir: string;
  config_exists: boolean;
  wezterm_installed: boolean;
}
```

---

## File Commands

### `list_backdrop_images`

Lists images in the backdrops directory.

**Signature:**
```rust
#[tauri::command]
fn list_backdrop_images() -> Result<Vec<ImageInfo>, String>
```

**TypeScript:**
```typescript
invoke<ImageInfo[]>('list_backdrop_images')
```

**Returns:**
```typescript
interface ImageInfo {
  filename: string;
  path: string;
  thumbnail?: string;  // Base64 encoded thumbnail (optional)
}
```

---

### `add_backdrop_image`

Copies an image to the backdrops directory.

**Signature:**
```rust
#[tauri::command]
async fn add_backdrop_image(source_path: String) -> Result<ImageInfo, String>
```

**TypeScript:**
```typescript
invoke<ImageInfo>('add_backdrop_image', { sourcePath: string })
```

**Behavior:**
1. Validate source file exists and is an image
2. Copy to `{config_dir}/backdrops/` with original filename
3. If filename exists, append number suffix
4. Return new image info

---

### `remove_backdrop_image`

Removes an image from the backdrops directory.

**Signature:**
```rust
#[tauri::command]
fn remove_backdrop_image(filename: String) -> Result<(), String>
```

**TypeScript:**
```typescript
invoke<void>('remove_backdrop_image', { filename: string })
```

---

### `create_backup`

Creates a manual backup of current config.

**Signature:**
```rust
#[tauri::command]
fn create_backup() -> Result<String, String>
```

**TypeScript:**
```typescript
invoke<string>('create_backup')
```

**Returns:**
- Path to backup directory, e.g., `/home/user/.config/wezterm.backup.20241128_143022`

---

### `restore_backup`

Restores config from a backup.

**Signature:**
```rust
#[tauri::command]
async fn restore_backup(backup_path: String) -> Result<(), String>
```

**TypeScript:**
```typescript
invoke<void>('restore_backup', { backupPath: string })
```

---

## Error Handling

All commands return `Result<T, String>` where the error string is user-friendly.

### Error Codes (in error message prefix)

| Prefix | Meaning |
|--------|---------|
| `CONFIG_NOT_FOUND:` | Config file doesn't exist |
| `PARSE_ERROR:` | Failed to parse existing config |
| `VALIDATION_ERROR:` | Config values invalid |
| `IO_ERROR:` | File system operation failed |
| `PERMISSION_ERROR:` | Insufficient permissions |

### TypeScript Error Handling

```typescript
try {
  const config = await invoke<AppearanceConfig>('get_config');
} catch (error) {
  if (typeof error === 'string') {
    if (error.startsWith('CONFIG_NOT_FOUND:')) {
      // Handle missing config - use defaults
    } else if (error.startsWith('PARSE_ERROR:')) {
      // Handle corrupted config
    }
  }
}
```

---

## Rate Limits & Performance

| Command | Expected Latency | Caching |
|---------|-----------------|---------|
| `get_config` | <50ms | None |
| `save_config` | <200ms | None |
| `get_monospace_fonts` | <500ms first call | Cache for session |
| `list_backdrop_images` | <100ms | None |

Font enumeration is slow (~500ms) on first call. The frontend should call once on startup and cache the result.
