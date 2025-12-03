-- WezTerm Settings UI Plugin
-- https://github.com/jms830/wezterm-settings-ui
--
-- This plugin integrates the wezterm-settings-tui with WezTerm's command palette
-- and keybindings for easy access to the settings editor.
--
-- Installation:
--   1. Install the TUI: cargo install wezterm-settings-tui
--   2. Add to your wezterm.lua:
--      local settings = wezterm.plugin.require("https://github.com/jms830/wezterm-settings-ui")
--      settings.apply_to_config(config)

local wezterm = require("wezterm")

local M = {}

-- Default configuration
M.config = {
   -- Binary name/path (will search PATH if not absolute)
   binary = "wezterm-settings-tui",
   
   -- Keybinding to open settings (set to nil to disable)
   keybinding = {
      key = ",",
      mods = "CTRL|SHIFT",
   },
   
   -- Show in command palette
   command_palette = true,
   
   -- Check for updates on startup (shows notification if outdated)
   check_updates = true,
   
   -- How to open the TUI
   -- "tab" = new tab, "window" = new window, "pane" = split pane
   open_mode = "tab",
}

-- Find the binary path
local function find_binary(name)
   -- If absolute path, use directly
   if name:sub(1, 1) == "/" or name:sub(2, 2) == ":" then
      local f = io.open(name, "r")
      if f then
         f:close()
         return name
      end
      return nil
   end
   
   -- Check common locations
   local home = wezterm.home_dir
   local paths = {
      home .. "/.cargo/bin/" .. name,
      "/usr/local/bin/" .. name,
      "/opt/homebrew/bin/" .. name,
      "/usr/bin/" .. name,
      home .. "/.local/bin/" .. name,
   }
   
   for _, path in ipairs(paths) do
      local f = io.open(path, "r")
      if f then
         f:close()
         return path
      end
   end
   
   -- Fall back to name (relies on PATH)
   return name
end

-- Get the action to spawn the TUI based on open_mode
local function get_spawn_action(binary_path, open_mode, panel)
   local args = { binary_path }
   if panel then
      table.insert(args, panel)
   end
   
   if open_mode == "window" then
      return wezterm.action.SpawnCommandInNewWindow({ args = args })
   elseif open_mode == "pane" then
      return wezterm.action.SplitPane({
         direction = "Right",
         command = { args = args },
         size = { Percent = 50 },
      })
   else
      -- Default: new tab
      return wezterm.action.SpawnCommandInNewTab({ args = args })
   end
end

-- Check if binary is installed
local function check_binary_installed(binary_path)
   local success, _, _ = wezterm.run_child_process({ binary_path, "--version" })
   return success
end

-- Apply plugin configuration to WezTerm config
function M.apply_to_config(config, opts)
   -- Merge user options with defaults
   opts = opts or {}
   for k, v in pairs(opts) do
      M.config[k] = v
   end
   
   local binary_path = find_binary(M.config.binary)
   local open_mode = M.config.open_mode
   
   -- Add keybinding if configured
   if M.config.keybinding then
      config.keys = config.keys or {}
      table.insert(config.keys, {
         key = M.config.keybinding.key,
         mods = M.config.keybinding.mods,
         action = get_spawn_action(binary_path, open_mode, nil),
      })
   end
   
   -- Add to command palette
   if M.config.command_palette then
      wezterm.on("augment-command-palette", function(window, pane)
         local commands = {
            {
               brief = "Settings: Open WezTerm Settings",
               icon = "md_cog",
               action = get_spawn_action(binary_path, open_mode, nil),
            },
            {
               brief = "Settings: Colors",
               icon = "md_palette",
               action = get_spawn_action(binary_path, open_mode, "colors"),
            },
            {
               brief = "Settings: Fonts", 
               icon = "md_format_font",
               action = get_spawn_action(binary_path, open_mode, "fonts"),
            },
            {
               brief = "Settings: Keybindings",
               icon = "md_keyboard",
               action = get_spawn_action(binary_path, open_mode, "keys"),
            },
         }
         return commands
      end)
   end
   
   -- Check for binary installation on startup
   wezterm.on("gui-startup", function()
      if not check_binary_installed(binary_path) then
         wezterm.log_warn("wezterm-settings-tui not found. Install with: cargo install wezterm-settings-tui")
      end
   end)
end

-- Manual function to open settings (can be called from keybindings)
function M.open(window, pane, panel)
   local binary_path = find_binary(M.config.binary)
   local args = { binary_path }
   if panel then
      table.insert(args, panel)
   end
   
   window:perform_action(
      wezterm.action.SpawnCommandInNewTab({ args = args }),
      pane
   )
end

-- Update the plugin (pulls latest from git)
function M.update()
   wezterm.plugin.update_all()
end

return M
