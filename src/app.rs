// App state and main event loop

use crate::config;
use crate::models::AppearanceConfig;
use crate::ui;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, widgets::ListState, Terminal};
use std::io;

/// Settings panel categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Themes,
    Colors,
    Fonts,
    Window,
    Cursor,
    Gpu,
    Keybindings,
}

impl Panel {
    pub fn all() -> &'static [Panel] {
        &[
            Panel::Themes,
            Panel::Colors,
            Panel::Fonts,
            Panel::Window,
            Panel::Cursor,
            Panel::Gpu,
            Panel::Keybindings,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Panel::Themes => "Themes",
            Panel::Colors => "Colors",
            Panel::Fonts => "Fonts",
            Panel::Window => "Window",
            Panel::Cursor => "Cursor",
            Panel::Gpu => "GPU",
            Panel::Keybindings => "Commands",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Panel::Themes => "󰔎",
            Panel::Colors => "󰏘",
            Panel::Fonts => "󰛖",
            Panel::Window => "󰖲",
            Panel::Cursor => "󰇀",
            Panel::Gpu => "󰢮",
            Panel::Keybindings => "󰌌",
        }
    }

    pub fn from_str(s: &str) -> Option<Panel> {
        match s.to_lowercase().as_str() {
            "themes" => Some(Panel::Themes),
            "colors" => Some(Panel::Colors),
            "fonts" => Some(Panel::Fonts),
            "window" => Some(Panel::Window),
            "cursor" => Some(Panel::Cursor),
            "gpu" => Some(Panel::Gpu),
            "keybindings" | "commands" => Some(Panel::Keybindings),
            _ => None,
        }
    }
}

/// Input mode for the app
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal navigation mode
    Normal,
    /// Editing a field
    Editing,
    /// Showing help overlay
    Help,
    /// Confirmation dialog
    Confirm,
}

/// Main application state
pub struct App {
    /// Current config being edited
    pub config: AppearanceConfig,
    /// Original config (for detecting changes)
    original_config: AppearanceConfig,
    /// Path to config directory
    pub config_dir: Option<String>,
    /// Currently selected panel
    pub current_panel: Panel,
    /// Index of selected item in sidebar
    pub sidebar_index: usize,
    /// Index of selected field in current panel
    pub field_index: usize,
    /// Current input mode
    pub input_mode: InputMode,
    /// Whether there are unsaved changes
    pub has_changes: bool,
    /// Status message to display
    pub status_message: Option<String>,
    /// Should the app quit
    pub should_quit: bool,
    /// Current input buffer for editing
    pub input_buffer: String,
    /// Available color schemes (built-in WezTerm schemes)
    pub available_themes: Vec<String>,
    /// Currently selected theme in theme browser
    pub theme_index: usize,
    /// Theme search/filter string
    pub theme_filter: String,
    /// Filtered theme list
    pub filtered_themes: Vec<String>,
    /// List state for theme scrolling
    pub theme_list_state: ListState,
    /// Available fonts (common programming fonts)
    pub available_fonts: Vec<String>,
    /// Currently selected font in font browser
    pub font_index: usize,
    /// Font search/filter string
    pub font_filter: String,
    /// Filtered font list
    pub filtered_fonts: Vec<String>,
    /// List state for font scrolling
    pub font_list_state: ListState,
}

impl App {
    pub fn new(config_dir: Option<String>, initial_panel: Option<String>) -> Result<Self> {
        let config = config::load_config(config_dir.as_deref()).unwrap_or_default();
        let original_config = config.clone();

        let current_panel = initial_panel
            .and_then(|s| Panel::from_str(&s))
            .unwrap_or(Panel::Colors);

        let sidebar_index = Panel::all()
            .iter()
            .position(|&p| p == current_panel)
            .unwrap_or(0);

        // Load built-in WezTerm theme names
        let available_themes = get_builtin_themes();
        let filtered_themes = available_themes.clone();
        let mut theme_list_state = ListState::default();
        theme_list_state.select(Some(0));

        // Load common programming fonts
        let available_fonts = get_common_fonts();
        let filtered_fonts = available_fonts.clone();
        let mut font_list_state = ListState::default();
        font_list_state.select(Some(0));

        Ok(Self {
            config,
            original_config,
            config_dir,
            current_panel,
            sidebar_index,
            field_index: 0,
            input_mode: InputMode::Normal,
            has_changes: false,
            status_message: None,
            should_quit: false,
            input_buffer: String::new(),
            available_themes,
            theme_index: 0,
            theme_filter: String::new(),
            filtered_themes,
            theme_list_state,
            available_fonts,
            font_index: 0,
            font_filter: String::new(),
            filtered_fonts,
            font_list_state,
        })
    }

    /// Run the main event loop
    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main loop
        let result = self.event_loop(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    fn event_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            // Draw UI
            terminal.draw(|f| ui::draw(f, self))?;

            // Handle input
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key.code, key.modifiers);
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Clear status message on any key
        self.status_message = None;

        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode(key, modifiers),
            InputMode::Editing => self.handle_editing_mode(key, modifiers),
            InputMode::Help => self.handle_help_mode(key),
            InputMode::Confirm => self.handle_confirm_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Special handling for Themes panel search
        if self.current_panel == Panel::Themes && self.field_index > 0 {
            match key {
                KeyCode::Char('/') => {
                    // Enter search mode for themes
                    self.input_mode = InputMode::Editing;
                    self.input_buffer = self.theme_filter.clone();
                    return;
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if self.theme_index > 0 {
                        self.theme_index -= 1;
                        self.theme_list_state.select(Some(self.theme_index));
                    }
                    return;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if self.theme_index + 1 < self.filtered_themes.len() {
                        self.theme_index += 1;
                        self.theme_list_state.select(Some(self.theme_index));
                    }
                    return;
                }
                KeyCode::Enter => {
                    self.apply_selected_theme();
                    return;
                }
                KeyCode::Char('h') | KeyCode::Left => {
                    self.field_index = 0;
                    return;
                }
                _ => {}
            }
        }

        // Special handling for Fonts panel font selector (field_index > 5)
        if self.current_panel == Panel::Fonts && self.field_index > 5 {
            match key {
                KeyCode::Char('/') => {
                    // Enter search mode for fonts
                    self.input_mode = InputMode::Editing;
                    self.input_buffer = self.font_filter.clone();
                    return;
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if self.font_index > 0 {
                        self.font_index -= 1;
                        self.font_list_state.select(Some(self.font_index));
                    }
                    return;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if self.font_index + 1 < self.filtered_fonts.len() {
                        self.font_index += 1;
                        self.font_list_state.select(Some(self.font_index));
                    }
                    return;
                }
                KeyCode::Enter => {
                    self.apply_selected_font();
                    return;
                }
                KeyCode::Char('h') | KeyCode::Left => {
                    self.field_index = 1; // Go back to settings
                    return;
                }
                _ => {}
            }
        }

        match key {
            // Quit
            KeyCode::Char('q') => {
                if self.has_changes {
                    self.input_mode = InputMode::Confirm;
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Esc => {
                if self.has_changes {
                    self.input_mode = InputMode::Confirm;
                } else {
                    self.should_quit = true;
                }
            }

            // Save
            KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.save_config();
            }

            // Help
            KeyCode::Char('?') => {
                self.input_mode = InputMode::Help;
            }

            // Navigation - up
            KeyCode::Char('k') | KeyCode::Up => {
                self.navigate_up();
            }

            // Navigation - down
            KeyCode::Char('j') | KeyCode::Down => {
                self.navigate_down();
            }

            // Navigation - left (back to sidebar)
            KeyCode::Char('h') | KeyCode::Left => {
                self.field_index = 0;
            }

            // Navigation - right / enter (into panel / edit)
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                self.enter_edit_mode();
            }

            // Tab - next field
            KeyCode::Tab => {
                self.next_field();
            }

            // Shift+Tab - previous field
            KeyCode::BackTab => {
                self.prev_field();
            }

            _ => {}
        }
    }

    fn handle_editing_mode(&mut self, key: KeyCode, _modifiers: KeyModifiers) {
        // Special handling for theme search
        if self.current_panel == Panel::Themes {
            match key {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input_buffer.clear();
                }
                KeyCode::Enter => {
                    // Apply filter and exit search mode
                    self.theme_filter = self.input_buffer.clone();
                    self.update_theme_filter();
                    self.input_mode = InputMode::Normal;
                    self.input_buffer.clear();
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                    // Live filter as you type
                    self.theme_filter = self.input_buffer.clone();
                    self.update_theme_filter();
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                    self.theme_filter = self.input_buffer.clone();
                    self.update_theme_filter();
                }
                _ => {}
            }
            return;
        }

        // Special handling for font search
        if self.current_panel == Panel::Fonts && self.field_index > 5 {
            match key {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input_buffer.clear();
                }
                KeyCode::Enter => {
                    // Apply filter and exit search mode
                    self.font_filter = self.input_buffer.clone();
                    self.update_font_filter();
                    self.input_mode = InputMode::Normal;
                    self.input_buffer.clear();
                }
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                    // Live filter as you type
                    self.font_filter = self.input_buffer.clone();
                    self.update_font_filter();
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                    self.font_filter = self.input_buffer.clone();
                    self.update_font_filter();
                }
                _ => {}
            }
            return;
        }

        match key {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.input_buffer.clear();
            }
            KeyCode::Enter => {
                self.apply_edit();
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            _ => {}
        }
    }

    fn handle_help_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
                self.input_mode = InputMode::Normal;
            }
            _ => {}
        }
    }

    fn handle_confirm_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.should_quit = true;
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.save_config();
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn navigate_up(&mut self) {
        if self.field_index == 0 {
            // In sidebar
            if self.sidebar_index > 0 {
                self.sidebar_index -= 1;
                self.current_panel = Panel::all()[self.sidebar_index];
            }
        } else {
            self.field_index = self.field_index.saturating_sub(1);
        }
    }

    fn navigate_down(&mut self) {
        if self.field_index == 0 {
            // In sidebar
            if self.sidebar_index < Panel::all().len() - 1 {
                self.sidebar_index += 1;
                self.current_panel = Panel::all()[self.sidebar_index];
            }
        } else {
            let max_fields = self.get_field_count();
            if self.field_index < max_fields {
                self.field_index += 1;
            }
        }
    }

    fn next_field(&mut self) {
        let max_fields = self.get_field_count();
        if self.field_index < max_fields {
            self.field_index += 1;
        } else {
            self.field_index = 1; // Wrap to first field (not sidebar)
        }
    }

    fn prev_field(&mut self) {
        if self.field_index > 1 {
            self.field_index -= 1;
        } else {
            self.field_index = self.get_field_count();
        }
    }

    fn enter_edit_mode(&mut self) {
        if self.field_index == 0 {
            // Move from sidebar into panel
            self.field_index = 1;
        } else if self.current_panel == Panel::Fonts && self.field_index == 1 {
            // From "Current Font" field, jump to font selector
            self.field_index = 6; // Enter font selector
        } else if self.current_panel == Panel::Fonts && self.field_index <= 5 {
            // Other font settings - enter edit mode for numeric/dropdown values
            self.input_buffer = self.get_current_field_value();
            self.input_mode = InputMode::Editing;
        } else if self.current_panel == Panel::Keybindings {
            // Toggle keybinding enabled/disabled
            self.toggle_keybinding();
        } else {
            // Start editing the current field
            self.input_buffer = self.get_current_field_value();
            self.input_mode = InputMode::Editing;
        }
    }
    
    /// Toggle a keybinding's enabled state based on current field
    fn toggle_keybinding(&mut self) {
        // The keybindings panel has multiple categories
        // We'll handle the most common toggles first
        let toggled = match self.field_index {
            // Custom commands (1-2)
            1 => {
                self.config.keybindings.custom_commands.settings_tui = !self.config.keybindings.custom_commands.settings_tui;
                Some(("Settings-TUI in command palette", self.config.keybindings.custom_commands.settings_tui))
            }
            2 => {
                self.config.keybindings.custom_commands.rename_tab = !self.config.keybindings.custom_commands.rename_tab;
                Some(("Rename Tab in command palette", self.config.keybindings.custom_commands.rename_tab))
            }
            // Mouse bindings (3-4)
            3 => {
                self.config.keybindings.mouse.ctrl_click_open_link = !self.config.keybindings.mouse.ctrl_click_open_link;
                Some(("Ctrl+Click open link", self.config.keybindings.mouse.ctrl_click_open_link))
            }
            4 => {
                self.config.keybindings.mouse.right_click_command_palette = !self.config.keybindings.mouse.right_click_command_palette;
                Some(("Right-click command palette", self.config.keybindings.mouse.right_click_command_palette))
            }
            // Disable defaults (5)
            5 => {
                self.config.keybindings.disable_defaults = !self.config.keybindings.disable_defaults;
                Some(("Disable default keybindings", self.config.keybindings.disable_defaults))
            }
            // Leader key (6)
            6 => {
                self.config.keybindings.leader.enabled = !self.config.keybindings.leader.enabled;
                Some(("Leader key", self.config.keybindings.leader.enabled))
            }
            _ => None,
        };
        
        if let Some((name, state)) = toggled {
            let state_str = if state { "enabled" } else { "disabled" };
            self.status_message = Some(format!("{} {}", name, state_str));
            self.has_changes = true;
        }
    }

    fn get_field_count(&self) -> usize {
        match self.current_panel {
            Panel::Themes => self.filtered_themes.len().max(1), // theme list
            Panel::Colors => 23, // fg, bg, cursor colors, 8 ansi, 8 brights, selection, etc.
            Panel::Fonts => 5,   // family, size, weight, load_target, render_target
            Panel::Window => 10, // opacity, padding (4), decorations, tab bar opts
            Panel::Cursor => 5,  // style, blink_rate, ease_in, ease_out, fps
            Panel::Gpu => 3,     // front_end, power_pref, max_fps
            Panel::Keybindings => 6, // settings_tui, rename_tab, mouse (2), disable_defaults, leader
        }
    }

    fn get_current_field_value(&self) -> String {
        // TODO: Implement based on current panel and field index
        String::new()
    }

    fn apply_edit(&mut self) {
        // TODO: Apply the input_buffer value to the appropriate config field
        self.has_changes = true;
        self.input_buffer.clear();
    }

    fn save_config(&mut self) {
        match config::save_config(&self.config, self.config_dir.as_deref()) {
            Ok(_) => {
                self.has_changes = false;
                self.original_config = self.config.clone();
                self.status_message = Some("Config saved successfully!".to_string());
            }
            Err(e) => {
                self.status_message = Some(format!("Error saving config: {}", e));
            }
        }
    }

    /// Update the filtered themes based on current filter
    pub fn update_theme_filter(&mut self) {
        let filter = self.theme_filter.to_lowercase();
        self.filtered_themes = self
            .available_themes
            .iter()
            .filter(|t| t.to_lowercase().contains(&filter))
            .cloned()
            .collect();
        self.theme_index = 0;
        self.theme_list_state.select(Some(0));
    }

    /// Apply the selected theme
    pub fn apply_selected_theme(&mut self) {
        if let Some(theme_name) = self.filtered_themes.get(self.theme_index) {
            self.config.color_scheme = Some(theme_name.clone());
            self.has_changes = true;
            self.status_message = Some(format!("Theme set to: {}", theme_name));
        }
    }

    /// Update the filtered fonts based on current filter
    pub fn update_font_filter(&mut self) {
        let filter = self.font_filter.to_lowercase();
        self.filtered_fonts = self
            .available_fonts
            .iter()
            .filter(|f| f.to_lowercase().contains(&filter))
            .cloned()
            .collect();
        self.font_index = 0;
        self.font_list_state.select(Some(0));
    }

    /// Apply the selected font
    pub fn apply_selected_font(&mut self) {
        if let Some(font_name) = self.filtered_fonts.get(self.font_index) {
            self.config.fonts.family = font_name.clone();
            self.has_changes = true;
            self.status_message = Some(format!("Font set to: {}", font_name));
        }
    }
}

/// Get list of built-in WezTerm color schemes
/// This is a curated list of popular schemes - WezTerm has 700+ built-in
fn get_builtin_themes() -> Vec<String> {
    vec![
        // Popular dark themes
        "Catppuccin Mocha".to_string(),
        "Catppuccin Macchiato".to_string(),
        "Catppuccin Frappe".to_string(),
        "Catppuccin Latte".to_string(),
        "Dracula".to_string(),
        "Dracula+".to_string(),
        "Gruvbox Dark".to_string(),
        "Gruvbox dark, hard (base16)".to_string(),
        "Gruvbox dark, medium (base16)".to_string(),
        "Gruvbox dark, soft (base16)".to_string(),
        "Gruvbox Light".to_string(),
        "Nord".to_string(),
        "Nord (Gogh)".to_string(),
        "Tokyo Night".to_string(),
        "Tokyo Night Storm".to_string(),
        "Tokyo Night Moon".to_string(),
        "tokyonight".to_string(),
        "tokyonight_night".to_string(),
        "tokyonight_storm".to_string(),
        "Solarized Dark".to_string(),
        "Solarized Dark Higher Contrast".to_string(),
        "Solarized Light".to_string(),
        "One Dark".to_string(),
        "OneDark".to_string(),
        "One Half Dark".to_string(),
        "One Half Light".to_string(),
        "Monokai".to_string(),
        "Monokai Pro".to_string(),
        "Monokai Remastered".to_string(),
        "Monokai Soda".to_string(),
        "GitHub Dark".to_string(),
        "GitHub Light".to_string(),
        "Kanagawa".to_string(),
        "Kanagawa Dragon".to_string(),
        "Kanagawa Wave".to_string(),
        "rose-pine".to_string(),
        "rose-pine-moon".to_string(),
        "rose-pine-dawn".to_string(),
        "Everforest Dark".to_string(),
        "Everforest Light".to_string(),
        "Material".to_string(),
        "Material Dark".to_string(),
        "Material Lighter".to_string(),
        "Material Ocean".to_string(),
        "Palenight".to_string(),
        "Ayu Dark".to_string(),
        "Ayu Light".to_string(),
        "Ayu Mirage".to_string(),
        // Classic themes
        "Afterglow".to_string(),
        "Alabaster".to_string(),
        "Base16".to_string(),
        "Breeze".to_string(),
        "Chalk".to_string(),
        "Dark+".to_string(),
        "Horizon Dark".to_string(),
        "Horizon Bright".to_string(),
        "Horizon".to_string(),
        "Nightfly".to_string(),
        "Night Owl".to_string(),
        "Night Owlish Light".to_string(),
        "Oceanic-Next".to_string(),
        "Panda".to_string(),
        "Papercolor Dark".to_string(),
        "Papercolor Light".to_string(),
        "Snazzy".to_string(),
        "Synthwave".to_string(),
        "Ubuntu".to_string(),
        "Zenburn".to_string(),
        // Retro/fun themes
        "Cyberdyne".to_string(),
        "CyberPunk2077".to_string(),
        "DoomOne".to_string(),
        "Espresso".to_string(),
        "Flat".to_string(),
        "Floraverse".to_string(),
        "Grape".to_string(),
        "Hipster Green".to_string(),
        "IC_Green_PPL".to_string(),
        "JetBrains Darcula".to_string(),
        "Laser".to_string(),
        "Lavandula".to_string(),
        "Matrix".to_string(),
        "Miramare".to_string(),
        "Neon".to_string(),
        "Nightfly".to_string(),
        "Nova".to_string(),
        "Ocean".to_string(),
        "Operator Mono Dark".to_string(),
        "Outrun Dark".to_string(),
        "PaperColor Dark (base16)".to_string(),
        "PaperColor Light (base16)".to_string(),
        "Purplepeter".to_string(),
        "Rebecca".to_string(),
        "Sonokai".to_string(),
        "SpaceGray".to_string(),
        "Spacemacs".to_string(),
        "SynthWave84".to_string(),
        "Tango".to_string(),
        "Twilight".to_string(),
        "UltraDark".to_string(),
        "Violet Dark".to_string(),
        "Violet Light".to_string(),
        "Wez".to_string(),
        "Whimsy".to_string(),
        "Wryan".to_string(),
        "zenbones".to_string(),
        "zenbones_dark".to_string(),
        "zenbones_light".to_string(),
    ]
}

/// Get list of common programming fonts
/// Includes popular Nerd Fonts and monospace fonts
fn get_common_fonts() -> Vec<String> {
    vec![
        // Nerd Fonts (popular)
        "JetBrainsMono Nerd Font".to_string(),
        "FiraCode Nerd Font".to_string(),
        "Hack Nerd Font".to_string(),
        "CaskaydiaCove Nerd Font".to_string(),
        "CaskaydiaMono Nerd Font".to_string(),
        "Iosevka Nerd Font".to_string(),
        "IosevkaTerm Nerd Font".to_string(),
        "Iosevka Term".to_string(),
        "MesloLGS Nerd Font".to_string(),
        "MesloLGM Nerd Font".to_string(),
        "MesloLGL Nerd Font".to_string(),
        "SourceCodePro Nerd Font".to_string(),
        "UbuntuMono Nerd Font".to_string(),
        "RobotoMono Nerd Font".to_string(),
        "DejaVuSansMono Nerd Font".to_string(),
        "InconsolataGo Nerd Font".to_string(),
        "Inconsolata Nerd Font".to_string(),
        "VictorMono Nerd Font".to_string(),
        "DroidSansMono Nerd Font".to_string(),
        "Cousine Nerd Font".to_string(),
        "BitstreamVeraSansMono Nerd Font".to_string(),
        "CodeNewRoman Nerd Font".to_string(),
        "Agave Nerd Font".to_string(),
        "Anonymice Nerd Font".to_string(),
        "Arimo Nerd Font".to_string(),
        "AurulentSansMono Nerd Font".to_string(),
        "BigBlueTerminal Nerd Font".to_string(),
        "ComicShannsMono Nerd Font".to_string(),
        "Fantasque Sans Mono Nerd Font".to_string(),
        "FuraMono Nerd Font".to_string(),
        "Gohu Nerd Font".to_string(),
        "Go Mono Nerd Font".to_string(),
        "Hasklug Nerd Font".to_string(),
        "Hurmit Nerd Font".to_string(),
        "iA Writer Mono Nerd Font".to_string(),
        "IBMPlexMono Nerd Font".to_string(),
        "Lilex Nerd Font".to_string(),
        "Lekton Nerd Font".to_string(),
        "LiterationMono Nerd Font".to_string(),
        "M+ Nerd Font".to_string(),
        "Monofur Nerd Font".to_string(),
        "Monoid Nerd Font".to_string(),
        "Mononoki Nerd Font".to_string(),
        "Noto Mono Nerd Font".to_string(),
        "OpenDyslexicMono Nerd Font".to_string(),
        "Overpass Mono Nerd Font".to_string(),
        "ProggyClean Nerd Font".to_string(),
        "ProFont Nerd Font".to_string(),
        "ShareTechMono Nerd Font".to_string(),
        "SpaceMono Nerd Font".to_string(),
        "Terminess Nerd Font".to_string(),
        "Tinos Nerd Font".to_string(),
        "Ubuntu Nerd Font".to_string(),
        "0xProto Nerd Font".to_string(),
        "3270 Nerd Font".to_string(),
        "Zed Mono Nerd Font".to_string(),
        // Standard monospace fonts (non-Nerd)
        "JetBrains Mono".to_string(),
        "Fira Code".to_string(),
        "Cascadia Code".to_string(),
        "Cascadia Mono".to_string(),
        "Source Code Pro".to_string(),
        "Hack".to_string(),
        "Iosevka".to_string(),
        "Victor Mono".to_string(),
        "IBM Plex Mono".to_string(),
        "Inconsolata".to_string(),
        "Monaco".to_string(),
        "Menlo".to_string(),
        "SF Mono".to_string(),
        "Consolas".to_string(),
        "Courier New".to_string(),
        "DejaVu Sans Mono".to_string(),
        "Ubuntu Mono".to_string(),
        "Roboto Mono".to_string(),
        "Droid Sans Mono".to_string(),
        "Anonymous Pro".to_string(),
        "PT Mono".to_string(),
        "Noto Sans Mono".to_string(),
        "Space Mono".to_string(),
        "Input Mono".to_string(),
        "Operator Mono".to_string(),
        "Dank Mono".to_string(),
        "MonoLisa".to_string(),
        "Berkeley Mono".to_string(),
        "Monaspace Neon".to_string(),
        "Monaspace Argon".to_string(),
        "Monaspace Xenon".to_string(),
        "Monaspace Radon".to_string(),
        "Monaspace Krypton".to_string(),
        "Geist Mono".to_string(),
        "Comic Code".to_string(),
        "Maple Mono".to_string(),
        "Commit Mono".to_string(),
        "Intel One Mono".to_string(),
    ]
}
