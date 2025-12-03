import { useEffect, useState } from "react";
import "./App.css";

// Dynamic import to handle when Tauri isn't available (e.g., browser dev)
const invoke = async <T,>(cmd: string, args?: Record<string, unknown>): Promise<T> => {
  // Check if we're in a Tauri context
  if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
    const { invoke: tauriInvoke } = await import("@tauri-apps/api/core");
    return tauriInvoke<T>(cmd, args);
  }
  throw new Error("Tauri API not available - please run with 'npm run tauri dev'");
};
import {
  AppearanceConfig,
  SystemInfo,
  DEFAULT_APPEARANCE_CONFIG,
} from "./types/config";

// Type for the config load result from backend
interface ConfigLoadResult {
  config: AppearanceConfig;
  config_path: string;
  config_exists: boolean;
  raw_content: string | null;
  parse_errors: string[];
}

// Type for the save result from backend
interface SaveConfigResult {
  success: boolean;
  config_path: string;
  backup_path: string | null;
}

// Settings panel type
type Panel = "general" | "colors" | "fonts" | "window" | "cursor" | "gpu" | "backdrops" | "command_palette" | "visual_bell";

// Color scheme info from backend
interface ColorSchemeInfo {
  name: string;
  category: string;
}

// Backdrop image info from backend
interface BackdropImage {
  filename: string;
  path: string;
}

function App() {
  const [config, setConfig] = useState<AppearanceConfig>(DEFAULT_APPEARANCE_CONFIG);
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [configPath, setConfigPath] = useState<string>("");
  const [configExists, setConfigExists] = useState<boolean>(false);
  const [parseErrors, setParseErrors] = useState<string[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [saving, setSaving] = useState<boolean>(false);
  const [hasChanges, setHasChanges] = useState<boolean>(false);
  const [statusMessage, setStatusMessage] = useState<string>("");
  const [activePanel, setActivePanel] = useState<Panel>("general");
  const [builtinSchemes, setBuiltinSchemes] = useState<ColorSchemeInfo[]>([]);
  const [backdropImages, setBackdropImages] = useState<BackdropImage[]>([]);

  // Load config on mount
  useEffect(() => {
    loadConfig();
    loadSystemInfo();
    loadBuiltinSchemes();
  }, []);

  async function loadBuiltinSchemes() {
    try {
      const schemes = await invoke<ColorSchemeInfo[]>("get_builtin_color_schemes");
      setBuiltinSchemes(schemes);
    } catch (error) {
      console.error("Failed to load builtin schemes:", error);
    }
  }

  async function loadConfig() {
    setLoading(true);
    try {
      const result = await invoke<ConfigLoadResult>("load_wezterm_config");
      setConfig(result.config);
      setConfigPath(result.config_path);
      setConfigExists(result.config_exists);
      setParseErrors(result.parse_errors);
      setHasChanges(false);
      if (result.parse_errors.length > 0) {
        setStatusMessage(`Loaded with ${result.parse_errors.length} warnings`);
      } else if (result.config_exists) {
        setStatusMessage("Config loaded successfully");
      } else {
        setStatusMessage("Using default config (no config file found)");
      }
    } catch (error) {
      console.error("Failed to load config:", error);
      const errorMsg = error instanceof Error ? error.message : String(error);
      if (errorMsg.includes("Tauri API not available")) {
        setStatusMessage("Please run with 'npm run tauri dev' to use this app");
      } else {
        setStatusMessage(`Error loading config: ${errorMsg}`);
      }
    } finally {
      setLoading(false);
    }
  }

  async function loadSystemInfo() {
    try {
      const info = await invoke<SystemInfo>("get_system_info");
      setSystemInfo(info);
    } catch (error) {
      console.error("Failed to load system info:", error);
    }
  }

  async function saveConfig() {
    setSaving(true);
    try {
      const result = await invoke<SaveConfigResult>("save_wezterm_config", {
        config,
      });
      if (result.success) {
        setHasChanges(false);
        setConfigExists(true);
        setStatusMessage(
          result.backup_path
            ? `Saved! Backup created at ${result.backup_path}`
            : "Config saved successfully!"
        );
      }
    } catch (error) {
      console.error("Failed to save config:", error);
      setStatusMessage(`Error saving config: ${error}`);
    } finally {
      setSaving(false);
    }
  }

  async function resetToDefaults() {
    try {
      const defaultConfig = await invoke<AppearanceConfig>("get_default_config");
      setConfig(defaultConfig);
      setHasChanges(true);
      setStatusMessage("Reset to defaults (not saved yet)");
    } catch (error) {
      console.error("Failed to get defaults:", error);
    }
  }

  // Update config helper for object sections
  function updateConfig<K extends keyof AppearanceConfig>(
    section: K,
    updates: Partial<AppearanceConfig[K]>
  ) {
    setConfig((prev) => {
      const currentValue = prev[section];
      // Handle object sections (colors, fonts, window, etc.)
      if (typeof currentValue === 'object' && currentValue !== null) {
        return {
          ...prev,
          [section]: { ...currentValue, ...updates },
        };
      }
      // Handle primitive sections (shouldn't happen, but just in case)
      return {
        ...prev,
        [section]: updates,
      };
    });
    setHasChanges(true);
  }

  // Update color_scheme directly
  function setColorScheme(schemeName: string | undefined) {
    setConfig((prev) => ({
      ...prev,
      color_scheme: schemeName,
    }));
    setHasChanges(true);
  }

  if (loading) {
    return (
      <main className="container loading">
        <h1>Loading WezTerm Settings...</h1>
      </main>
    );
  }

  return (
    <main className="container">
      <header className="header">
        <h1>WezTerm Settings</h1>
        <div className="header-info">
          {systemInfo && (
            <span className="platform-badge">{systemInfo.platform}</span>
          )}
          <span className="config-path" title={configPath}>
            {configExists ? "Config loaded" : "New config"}
          </span>
        </div>
      </header>

      {/* Status bar */}
      {statusMessage && (
        <div className={`status-bar ${parseErrors.length > 0 ? "warning" : ""}`}>
          {statusMessage}
        </div>
      )}

      {/* Parse errors */}
      {parseErrors.length > 0 && (
        <div className="parse-errors">
          <details>
            <summary>Parse warnings ({parseErrors.length})</summary>
            <ul>
              {parseErrors.map((err, i) => (
                <li key={i}>{err}</li>
              ))}
            </ul>
          </details>
        </div>
      )}

      <div className="main-layout">
        {/* Sidebar */}
        <nav className="sidebar">
          <ul>
            {(["general", "colors", "fonts", "window", "cursor", "gpu", "backdrops", "command_palette", "visual_bell"] as Panel[]).map(
              (panel) => (
                <li key={panel}>
                  <button
                    className={activePanel === panel ? "active" : ""}
                    onClick={() => setActivePanel(panel)}
                  >
                    {panel.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ')}
                  </button>
                </li>
              )
            )}
          </ul>
        </nav>

        {/* Main content */}
        <div className="content">
          {activePanel === "general" && (
            <GeneralPanel config={config} updateConfig={updateConfig} />
          )}
          {activePanel === "colors" && (
            <ColorsPanel
              config={config}
              updateConfig={updateConfig}
              builtinSchemes={builtinSchemes}
              setColorScheme={setColorScheme}
            />
          )}
          {activePanel === "fonts" && (
            <FontsPanel config={config} updateConfig={updateConfig} />
          )}
          {activePanel === "window" && (
            <WindowPanel config={config} updateConfig={updateConfig} />
          )}
          {activePanel === "cursor" && (
            <CursorPanel config={config} updateConfig={updateConfig} />
          )}
          {activePanel === "gpu" && (
            <GPUPanel config={config} updateConfig={updateConfig} />
          )}
          {activePanel === "backdrops" && (
            <BackdropsPanel
              config={config}
              updateConfig={updateConfig}
              backdropImages={backdropImages}
              setBackdropImages={setBackdropImages}
            />
          )}
          {activePanel === "command_palette" && (
            <CommandPalettePanel config={config} updateConfig={updateConfig} />
          )}
          {activePanel === "visual_bell" && (
            <VisualBellPanel config={config} updateConfig={updateConfig} />
          )}
        </div>
      </div>

      {/* Footer actions */}
      <footer className="footer">
        <button onClick={resetToDefaults} disabled={saving}>
          Reset to Defaults
        </button>
        <button onClick={loadConfig} disabled={saving}>
          Reload
        </button>
        <button
          onClick={saveConfig}
          disabled={saving || !hasChanges}
          className="primary"
        >
          {saving ? "Saving..." : hasChanges ? "Save Changes" : "Saved"}
        </button>
      </footer>
    </main>
  );
}

// ============================================================================
// Panel Components
// ============================================================================

interface PanelProps {
  config: AppearanceConfig;
  updateConfig: <K extends keyof AppearanceConfig>(
    section: K,
    updates: Partial<AppearanceConfig[K]>
  ) => void;
}

function GeneralPanel({ config, updateConfig }: PanelProps) {
  return (
    <div className="panel">
      <h2>General</h2>

      <div className="form-section">
        <h3>Behavior</h3>

        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="automatically_reload_config"
            checked={config.general.automatically_reload_config}
            onChange={(e) =>
              updateConfig("general", { automatically_reload_config: e.target.checked })
            }
          />
          <label htmlFor="automatically_reload_config">Automatically Reload Config</label>
        </div>

        <div className="form-group">
          <label>Exit Behavior</label>
          <select
            value={config.general.exit_behavior}
            onChange={(e) =>
              updateConfig("general", { exit_behavior: e.target.value as any })
            }
          >
            <option value="Close">Close</option>
            <option value="CloseOnCleanExit">Close on Clean Exit</option>
            <option value="Hold">Hold</option>
          </select>
          <p className="help-text">
            Controls what happens when the shell exits.
          </p>
        </div>

        <div className="form-group">
          <label>Audible Bell</label>
          <select
            value={config.general.audible_bell}
            onChange={(e) =>
              updateConfig("general", { audible_bell: e.target.value as any })
            }
          >
            <option value="SystemBeep">System Beep</option>
            <option value="Disabled">Disabled</option>
          </select>
        </div>
      </div>

      <div className="form-section">
        <h3>Window Size</h3>

        <div className="form-group">
          <label>Initial Rows</label>
          <input
            type="number"
            min="1"
            max="500"
            value={config.general.initial_rows}
            onChange={(e) =>
              updateConfig("general", { initial_rows: parseInt(e.target.value) || 24 })
            }
          />
        </div>

        <div className="form-group">
          <label>Initial Columns</label>
          <input
            type="number"
            min="1"
            max="500"
            value={config.general.initial_cols}
            onChange={(e) =>
              updateConfig("general", { initial_cols: parseInt(e.target.value) || 80 })
            }
          />
        </div>

        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="adjust_window_size_when_changing_font_size"
            checked={config.general.adjust_window_size_when_changing_font_size}
            onChange={(e) =>
              updateConfig("general", {
                adjust_window_size_when_changing_font_size: e.target.checked,
              })
            }
          />
          <label htmlFor="adjust_window_size_when_changing_font_size">
            Adjust Window Size When Changing Font Size
          </label>
        </div>
      </div>

      <div className="form-section">
        <h3>Scrollback</h3>

        <div className="form-group">
          <label>Scrollback Lines</label>
          <input
            type="number"
            min="0"
            max="100000"
            step="500"
            value={config.general.scrollback_lines}
            onChange={(e) =>
              updateConfig("general", { scrollback_lines: parseInt(e.target.value) || 3500 })
            }
          />
          <p className="help-text">
            Number of lines to keep in scrollback buffer (0 = unlimited).
          </p>
        </div>

        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="enable_scroll_bar"
            checked={config.general.enable_scroll_bar}
            onChange={(e) =>
              updateConfig("general", { enable_scroll_bar: e.target.checked })
            }
          />
          <label htmlFor="enable_scroll_bar">Enable Scroll Bar</label>
        </div>
      </div>

      <div className="form-section">
        <h3>Tabs</h3>

        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="switch_to_last_active_tab_when_closing_tab"
            checked={config.general.switch_to_last_active_tab_when_closing_tab}
            onChange={(e) =>
              updateConfig("general", {
                switch_to_last_active_tab_when_closing_tab: e.target.checked,
              })
            }
          />
          <label htmlFor="switch_to_last_active_tab_when_closing_tab">
            Switch to Last Active Tab When Closing Tab
          </label>
        </div>
      </div>
    </div>
  );
}

interface ColorsPanelProps extends PanelProps {
  builtinSchemes: ColorSchemeInfo[];
  setColorScheme: (schemeName: string | undefined) => void;
}

function ColorsPanel({ config, updateConfig, builtinSchemes, setColorScheme }: ColorsPanelProps) {
  // Group schemes by category for the dropdown
  const schemesByCategory = builtinSchemes.reduce((acc, scheme) => {
    if (!acc[scheme.category]) {
      acc[scheme.category] = [];
    }
    acc[scheme.category].push(scheme.name);
    return acc;
  }, {} as Record<string, string[]>);

  const sortedCategories = Object.keys(schemesByCategory).sort();

  return (
    <div className="panel">
      <h2>Colors</h2>

      <div className="form-section">
        <h3>Theme</h3>
        <div className="form-group">
          <label>Color Scheme</label>
          <select
            value={config.color_scheme || ""}
            onChange={(e) => setColorScheme(e.target.value || undefined)}
          >
            <option value="">Custom (use colors below)</option>
            {sortedCategories.map((category) => (
              <optgroup key={category} label={category}>
                {schemesByCategory[category].sort().map((name) => (
                  <option key={name} value={name}>
                    {name}
                  </option>
                ))}
              </optgroup>
            ))}
          </select>
          <p className="help-text">
            {config.color_scheme
              ? `Using built-in scheme "${config.color_scheme}". Custom colors below are ignored.`
              : "Using custom colors defined below."}
          </p>
        </div>
      </div>

      <div className="form-section">
        <h3>Core Colors</h3>
        <div className="form-grid">
          <ColorInput
            label="Foreground"
            value={config.colors.foreground}
            onChange={(v) => updateConfig("colors", { foreground: v })}
          />
          <ColorInput
            label="Background"
            value={config.colors.background}
            onChange={(v) => updateConfig("colors", { background: v })}
          />
          <ColorInput
            label="Cursor BG"
            value={config.colors.cursor_bg}
            onChange={(v) => updateConfig("colors", { cursor_bg: v })}
          />
          <ColorInput
            label="Cursor FG"
            value={config.colors.cursor_fg}
            onChange={(v) => updateConfig("colors", { cursor_fg: v })}
          />
          <ColorInput
            label="Selection BG"
            value={config.colors.selection_bg}
            onChange={(v) => updateConfig("colors", { selection_bg: v })}
          />
          <ColorInput
            label="Selection FG"
            value={config.colors.selection_fg}
            onChange={(v) => updateConfig("colors", { selection_fg: v })}
          />
        </div>
      </div>

      <div className="form-section">
        <h3>ANSI Colors</h3>
        <div className="color-palette">
          {config.colors.ansi.map((color, i) => (
            <ColorInput
              key={`ansi-${i}`}
              label={`ANSI ${i}`}
              value={color}
              onChange={(v) => {
                const newAnsi = [...config.colors.ansi] as typeof config.colors.ansi;
                newAnsi[i] = v;
                updateConfig("colors", { ansi: newAnsi });
              }}
            />
          ))}
        </div>
      </div>

      <div className="form-section">
        <h3>Bright Colors</h3>
        <div className="color-palette">
          {config.colors.brights.map((color, i) => (
            <ColorInput
              key={`bright-${i}`}
              label={`Bright ${i}`}
              value={color}
              onChange={(v) => {
                const newBrights = [...config.colors.brights] as typeof config.colors.brights;
                newBrights[i] = v;
                updateConfig("colors", { brights: newBrights });
              }}
            />
          ))}
        </div>
      </div>
    </div>
  );
}

function FontsPanel({ config, updateConfig }: PanelProps) {
  return (
    <div className="panel">
      <h2>Fonts</h2>

      <div className="form-section">
        <div className="form-group">
          <label>Font Family</label>
          <input
            type="text"
            value={config.fonts.family}
            onChange={(e) => updateConfig("fonts", { family: e.target.value })}
          />
        </div>

        <div className="form-group">
          <label>Font Size</label>
          <input
            type="number"
            min="6"
            max="72"
            step="0.5"
            value={config.fonts.size}
            onChange={(e) =>
              updateConfig("fonts", { size: parseFloat(e.target.value) })
            }
          />
        </div>

        <div className="form-group">
          <label>Font Weight</label>
          <select
            value={config.fonts.weight || ""}
            onChange={(e) =>
              updateConfig("fonts", {
                weight: e.target.value as any || undefined,
              })
            }
          >
            <option value="">Default</option>
            <option value="Thin">Thin</option>
            <option value="ExtraLight">Extra Light</option>
            <option value="Light">Light</option>
            <option value="Regular">Regular</option>
            <option value="Medium">Medium</option>
            <option value="DemiBold">DemiBold</option>
            <option value="Bold">Bold</option>
            <option value="ExtraBold">Extra Bold</option>
            <option value="Black">Black</option>
          </select>
        </div>

        <div className="form-group">
          <label>Freetype Load Target</label>
          <select
            value={config.fonts.freetype_load_target || "Normal"}
            onChange={(e) =>
              updateConfig("fonts", {
                freetype_load_target: e.target.value as any,
              })
            }
          >
            <option value="Normal">Normal</option>
            <option value="Light">Light</option>
            <option value="Mono">Mono</option>
            <option value="HorizontalLcd">Horizontal LCD</option>
          </select>
        </div>
      </div>
    </div>
  );
}

function WindowPanel({ config, updateConfig }: PanelProps) {
  return (
    <div className="panel">
      <h2>Window</h2>

      <div className="form-section">
        <h3>Appearance</h3>

        <div className="form-group">
          <label>Background Opacity</label>
          <input
            type="range"
            min="0"
            max="1"
            step="0.05"
            value={config.window.window_background_opacity}
            onChange={(e) =>
              updateConfig("window", {
                window_background_opacity: parseFloat(e.target.value),
              })
            }
          />
          <span>{config.window.window_background_opacity.toFixed(2)}</span>
        </div>

        <div className="form-group">
          <label>Window Decorations</label>
          <select
            value={config.window.window_decorations}
            onChange={(e) =>
              updateConfig("window", {
                window_decorations: e.target.value as any,
              })
            }
          >
            <option value="FULL">Full</option>
            <option value="RESIZE">Resize</option>
            <option value="NONE">None</option>
            <option value="TITLE">Title</option>
            <option value="INTEGRATED_BUTTONS|RESIZE">
              Integrated Buttons + Resize
            </option>
          </select>
        </div>
      </div>

      <div className="form-section">
        <h3>Tab Bar</h3>

        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="enable_tab_bar"
            checked={config.window.enable_tab_bar}
            onChange={(e) =>
              updateConfig("window", { enable_tab_bar: e.target.checked })
            }
          />
          <label htmlFor="enable_tab_bar">Enable Tab Bar</label>
        </div>

        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="hide_tab_bar_if_only_one_tab"
            checked={config.window.hide_tab_bar_if_only_one_tab}
            onChange={(e) =>
              updateConfig("window", {
                hide_tab_bar_if_only_one_tab: e.target.checked,
              })
            }
          />
          <label htmlFor="hide_tab_bar_if_only_one_tab">
            Hide Tab Bar If Only One Tab
          </label>
        </div>

        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="use_fancy_tab_bar"
            checked={config.window.use_fancy_tab_bar}
            onChange={(e) =>
              updateConfig("window", { use_fancy_tab_bar: e.target.checked })
            }
          />
          <label htmlFor="use_fancy_tab_bar">Use Fancy Tab Bar</label>
        </div>

        <div className="form-group">
          <label>Tab Max Width</label>
          <input
            type="number"
            min="10"
            max="100"
            value={config.window.tab_max_width}
            onChange={(e) =>
              updateConfig("window", {
                tab_max_width: parseInt(e.target.value),
              })
            }
          />
        </div>
      </div>

      <div className="form-section">
        <h3>Padding</h3>
        <div className="form-grid">
          <div className="form-group">
            <label>Left</label>
            <input
              type="number"
              min="0"
              value={config.window.window_padding.left}
              onChange={(e) =>
                updateConfig("window", {
                  window_padding: {
                    ...config.window.window_padding,
                    left: parseFloat(e.target.value),
                  },
                })
              }
            />
          </div>
          <div className="form-group">
            <label>Right</label>
            <input
              type="number"
              min="0"
              value={config.window.window_padding.right}
              onChange={(e) =>
                updateConfig("window", {
                  window_padding: {
                    ...config.window.window_padding,
                    right: parseFloat(e.target.value),
                  },
                })
              }
            />
          </div>
          <div className="form-group">
            <label>Top</label>
            <input
              type="number"
              min="0"
              value={config.window.window_padding.top}
              onChange={(e) =>
                updateConfig("window", {
                  window_padding: {
                    ...config.window.window_padding,
                    top: parseFloat(e.target.value),
                  },
                })
              }
            />
          </div>
          <div className="form-group">
            <label>Bottom</label>
            <input
              type="number"
              min="0"
              value={config.window.window_padding.bottom}
              onChange={(e) =>
                updateConfig("window", {
                  window_padding: {
                    ...config.window.window_padding,
                    bottom: parseFloat(e.target.value),
                  },
                })
              }
            />
          </div>
        </div>
      </div>
    </div>
  );
}

function CursorPanel({ config, updateConfig }: PanelProps) {
  return (
    <div className="panel">
      <h2>Cursor</h2>

      <div className="form-section">
        <div className="form-group">
          <label>Cursor Style</label>
          <select
            value={config.cursor.default_cursor_style}
            onChange={(e) =>
              updateConfig("cursor", {
                default_cursor_style: e.target.value as any,
              })
            }
          >
            <option value="SteadyBlock">Steady Block</option>
            <option value="BlinkingBlock">Blinking Block</option>
            <option value="SteadyUnderline">Steady Underline</option>
            <option value="BlinkingUnderline">Blinking Underline</option>
            <option value="SteadyBar">Steady Bar</option>
            <option value="BlinkingBar">Blinking Bar</option>
          </select>
        </div>

        <div className="form-group">
          <label>Cursor Blink Rate (ms)</label>
          <input
            type="number"
            min="0"
            max="2000"
            step="50"
            value={config.cursor.cursor_blink_rate}
            onChange={(e) =>
              updateConfig("cursor", {
                cursor_blink_rate: parseInt(e.target.value),
              })
            }
          />
        </div>

        <div className="form-group">
          <label>Animation FPS</label>
          <input
            type="number"
            min="1"
            max="240"
            value={config.cursor.animation_fps}
            onChange={(e) =>
              updateConfig("cursor", {
                animation_fps: parseInt(e.target.value),
              })
            }
          />
        </div>

        <div className="form-group">
          <label>Blink Ease In</label>
          <select
            value={config.cursor.cursor_blink_ease_in}
            onChange={(e) =>
              updateConfig("cursor", {
                cursor_blink_ease_in: e.target.value as any,
              })
            }
          >
            <option value="Linear">Linear</option>
            <option value="EaseIn">Ease In</option>
            <option value="EaseOut">Ease Out</option>
            <option value="EaseInOut">Ease In Out</option>
            <option value="Constant">Constant</option>
          </select>
        </div>

        <div className="form-group">
          <label>Blink Ease Out</label>
          <select
            value={config.cursor.cursor_blink_ease_out}
            onChange={(e) =>
              updateConfig("cursor", {
                cursor_blink_ease_out: e.target.value as any,
              })
            }
          >
            <option value="Linear">Linear</option>
            <option value="EaseIn">Ease In</option>
            <option value="EaseOut">Ease Out</option>
            <option value="EaseInOut">Ease In Out</option>
            <option value="Constant">Constant</option>
          </select>
        </div>
      </div>
    </div>
  );
}

function GPUPanel({ config, updateConfig }: PanelProps) {
  return (
    <div className="panel">
      <h2>GPU</h2>

      <div className="form-section">
        <div className="form-group">
          <label>Front End</label>
          <select
            value={config.gpu.front_end}
            onChange={(e) =>
              updateConfig("gpu", { front_end: e.target.value as any })
            }
          >
            <option value="WebGpu">WebGPU</option>
            <option value="OpenGL">OpenGL</option>
            <option value="Software">Software</option>
          </select>
        </div>

        <div className="form-group">
          <label>WebGPU Power Preference</label>
          <select
            value={config.gpu.webgpu_power_preference}
            onChange={(e) =>
              updateConfig("gpu", {
                webgpu_power_preference: e.target.value as any,
              })
            }
          >
            <option value="HighPerformance">High Performance</option>
            <option value="LowPower">Low Power</option>
          </select>
        </div>

        <div className="form-group">
          <label>Max FPS</label>
          <input
            type="number"
            min="1"
            max="240"
            value={config.gpu.max_fps}
            onChange={(e) =>
              updateConfig("gpu", { max_fps: parseInt(e.target.value) })
            }
          />
        </div>
      </div>
    </div>
  );
}

interface BackdropsPanelProps extends PanelProps {
  backdropImages: BackdropImage[];
  setBackdropImages: (images: BackdropImage[]) => void;
}

function BackdropsPanel({ config, updateConfig, backdropImages, setBackdropImages }: BackdropsPanelProps) {
  const [loadingImages, setLoadingImages] = useState(false);

  // Load images when directory changes
  async function loadImagesFromDirectory(directory: string) {
    if (!directory) {
      setBackdropImages([]);
      return;
    }
    
    setLoadingImages(true);
    try {
      const images = await invoke<BackdropImage[]>("list_backdrop_images", { directory });
      setBackdropImages(images);
    } catch (error) {
      console.error("Failed to load backdrop images:", error);
      setBackdropImages([]);
    } finally {
      setLoadingImages(false);
    }
  }

  // Open folder picker dialog
  async function selectFolder() {
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Backdrop Images Folder",
      });
      
      if (selected && typeof selected === 'string') {
        updateConfig("backdrop", { images_dir: selected });
        loadImagesFromDirectory(selected);
      }
    } catch (error) {
      console.error("Failed to open folder dialog:", error);
    }
  }

  // Open file picker dialog for single image
  async function selectImage() {
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const selected = await open({
        multiple: true,
        title: "Select Backdrop Images",
        filters: [{
          name: "Images",
          extensions: ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff"],
        }],
      });
      
      if (selected) {
        const paths = Array.isArray(selected) ? selected : [selected];
        const newImages = paths.map(p => ({
          filename: p.split(/[/\\]/).pop() || p,
          path: p,
        }));
        
        // Add to existing images list
        const existingPaths = new Set(config.backdrop.images);
        const uniqueNewPaths = paths.filter(p => !existingPaths.has(p));
        
        if (uniqueNewPaths.length > 0) {
          updateConfig("backdrop", { 
            images: [...config.backdrop.images, ...uniqueNewPaths] 
          });
          setBackdropImages([...backdropImages, ...newImages.filter(img => uniqueNewPaths.includes(img.path))]);
        }
      }
    } catch (error) {
      console.error("Failed to open file dialog:", error);
    }
  }

  // Remove an image from the list
  function removeImage(path: string) {
    updateConfig("backdrop", {
      images: config.backdrop.images.filter(p => p !== path),
    });
    setBackdropImages(backdropImages.filter(img => img.path !== path));
  }

  // Load images on mount if directory is set
  useEffect(() => {
    if (config.backdrop.images_dir) {
      loadImagesFromDirectory(config.backdrop.images_dir);
    }
  }, []);

  return (
    <div className="panel">
      <h2>Backdrops</h2>

      <div className="form-section">
        <h3>Settings</h3>

        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="backdrop_enabled"
            checked={config.backdrop.enabled}
            onChange={(e) =>
              updateConfig("backdrop", { enabled: e.target.checked })
            }
          />
          <label htmlFor="backdrop_enabled">Enable Background Images</label>
        </div>

        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="random_on_start"
            checked={config.backdrop.random_on_start}
            onChange={(e) =>
              updateConfig("backdrop", { random_on_start: e.target.checked })
            }
          />
          <label htmlFor="random_on_start">Random Image on Start</label>
        </div>

        <div className="form-group">
          <label>Overlay Opacity</label>
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={config.backdrop.overlay_opacity}
            onChange={(e) =>
              updateConfig("backdrop", {
                overlay_opacity: parseFloat(e.target.value),
              })
            }
          />
          <span>{config.backdrop.overlay_opacity.toFixed(2)}</span>
        </div>

        <div className="form-group">
          <label>Focus Color (solid background when focus mode is on)</label>
          <div className="color-input-row">
            <input
              type="color"
              value={config.backdrop.focus_color.startsWith("#") ? config.backdrop.focus_color : "#000000"}
              onChange={(e) => updateConfig("backdrop", { focus_color: e.target.value })}
            />
            <input
              type="text"
              value={config.backdrop.focus_color}
              onChange={(e) => updateConfig("backdrop", { focus_color: e.target.value })}
              placeholder="#000000"
            />
          </div>
        </div>
      </div>

      <div className="form-section">
        <h3>Images Folder</h3>
        
        <div className="form-group">
          <label>Images Directory</label>
          <div className="input-with-button">
            <input
              type="text"
              value={config.backdrop.images_dir}
              onChange={(e) => {
                updateConfig("backdrop", { images_dir: e.target.value });
              }}
              onBlur={() => loadImagesFromDirectory(config.backdrop.images_dir)}
              placeholder="/path/to/backdrops"
            />
            <button type="button" onClick={selectFolder}>
              Browse...
            </button>
          </div>
          <p className="help-text">
            Set a folder containing background images. Images will be loaded automatically.
          </p>
        </div>

        <div className="form-group">
          <button type="button" onClick={selectImage} className="secondary">
            Add Individual Images...
          </button>
        </div>
      </div>

      <div className="form-section">
        <h3>
          Images {loadingImages && "(loading...)"} 
          {!loadingImages && backdropImages.length > 0 && `(${backdropImages.length})`}
        </h3>
        
        {backdropImages.length === 0 && !loadingImages && (
          <p className="help-text">
            No images found. Select a folder or add individual images above.
          </p>
        )}

        {backdropImages.length > 0 && (
          <div className="backdrop-gallery">
            {backdropImages.map((img) => (
              <div key={img.path} className="backdrop-item">
                <div className="backdrop-preview">
                  <img 
                    src={`asset://localhost/${img.path}`} 
                    alt={img.filename}
                    onError={(e) => {
                      // Fallback for when asset protocol doesn't work
                      (e.target as HTMLImageElement).style.display = 'none';
                    }}
                  />
                </div>
                <div className="backdrop-info">
                  <span className="backdrop-name" title={img.path}>
                    {img.filename}
                  </span>
                  <button 
                    type="button" 
                    className="backdrop-remove"
                    onClick={() => removeImage(img.path)}
                    title="Remove image"
                  >
                    x
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function CommandPalettePanel({ config, updateConfig }: PanelProps) {
  return (
    <div className="panel">
      <h2>Command Palette</h2>

      <div className="form-section">
        <h3>Appearance</h3>

        <ColorInput
          label="Foreground Color"
          value={config.command_palette.fg_color}
          onChange={(v) => updateConfig("command_palette", { fg_color: v })}
        />

        <ColorInput
          label="Background Color"
          value={config.command_palette.bg_color}
          onChange={(v) => updateConfig("command_palette", { bg_color: v })}
        />

        <div className="form-group">
          <label>Font Size</label>
          <input
            type="number"
            min="8"
            max="48"
            step="0.5"
            value={config.command_palette.font_size}
            onChange={(e) =>
              updateConfig("command_palette", { font_size: parseFloat(e.target.value) || 14 })
            }
          />
        </div>
      </div>

      <div className="form-section">
        <p className="help-text">
          The command palette is accessed with Ctrl+Shift+P (Cmd+Shift+P on macOS).
          It provides quick access to WezTerm commands and actions.
        </p>
      </div>
    </div>
  );
}

function VisualBellPanel({ config, updateConfig }: PanelProps) {
  return (
    <div className="panel">
      <h2>Visual Bell</h2>

      <div className="form-section">
        <h3>Target</h3>

        <div className="form-group">
          <label>Visual Bell Target</label>
          <select
            value={config.visual_bell.target}
            onChange={(e) =>
              updateConfig("visual_bell", { target: e.target.value })
            }
          >
            <option value="BackgroundColor">Background Color</option>
            <option value="CursorColor">Cursor Color</option>
          </select>
          <p className="help-text">
            What element should flash when the bell is triggered.
          </p>
        </div>
      </div>

      <div className="form-section">
        <h3>Animation</h3>

        <div className="form-group">
          <label>Fade In Duration (ms)</label>
          <input
            type="number"
            min="0"
            max="1000"
            step="25"
            value={config.visual_bell.fade_in_duration_ms}
            onChange={(e) =>
              updateConfig("visual_bell", { fade_in_duration_ms: parseInt(e.target.value) || 75 })
            }
          />
        </div>

        <div className="form-group">
          <label>Fade Out Duration (ms)</label>
          <input
            type="number"
            min="0"
            max="1000"
            step="25"
            value={config.visual_bell.fade_out_duration_ms}
            onChange={(e) =>
              updateConfig("visual_bell", { fade_out_duration_ms: parseInt(e.target.value) || 150 })
            }
          />
        </div>

        <div className="form-group">
          <label>Fade In Function</label>
          <select
            value={config.visual_bell.fade_in_function}
            onChange={(e) =>
              updateConfig("visual_bell", { fade_in_function: e.target.value as any })
            }
          >
            <option value="Linear">Linear</option>
            <option value="EaseIn">Ease In</option>
            <option value="EaseOut">Ease Out</option>
            <option value="EaseInOut">Ease In Out</option>
            <option value="Constant">Constant</option>
          </select>
        </div>

        <div className="form-group">
          <label>Fade Out Function</label>
          <select
            value={config.visual_bell.fade_out_function}
            onChange={(e) =>
              updateConfig("visual_bell", { fade_out_function: e.target.value as any })
            }
          >
            <option value="Linear">Linear</option>
            <option value="EaseIn">Ease In</option>
            <option value="EaseOut">Ease Out</option>
            <option value="EaseInOut">Ease In Out</option>
            <option value="Constant">Constant</option>
          </select>
        </div>
      </div>
    </div>
  );
}

// ============================================================================
// Reusable Components
// ============================================================================

interface ColorInputProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
}

function ColorInput({ label, value, onChange }: ColorInputProps) {
  return (
    <div className="color-input">
      <label>{label}</label>
      <div className="color-input-row">
        <input
          type="color"
          value={value.startsWith("#") ? value : "#000000"}
          onChange={(e) => onChange(e.target.value)}
        />
        <input
          type="text"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder="#000000"
        />
      </div>
    </div>
  );
}

export default App;
