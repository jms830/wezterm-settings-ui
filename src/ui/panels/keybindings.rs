// Keybindings settings panel

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Draw the keybindings settings panel
pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let kb = &app.config.keybindings;
    
    // Split into two columns for better organization
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    
    // Left column: Settings & Custom Commands
    draw_left_column(f, chunks[0], app, kb);
    
    // Right column: Key binding categories summary
    draw_right_column(f, chunks[1], app, kb);
}

fn draw_left_column(f: &mut Frame, area: Rect, app: &App, kb: &wezterm_settings_gui_lib::models::KeyBindingsConfig) {
    let mut lines: Vec<Line> = vec![];
    
    // Custom Commands Section
    lines.push(Line::from(Span::styled(
        "Custom Command Palette Entries",
        Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
    )));
    lines.push(Line::from(""));
    
    // Settings-TUI
    let settings_tui_enabled = kb.custom_commands.settings_tui;
    add_toggle_line(&mut lines, 1, app.field_index, "Settings-TUI", settings_tui_enabled,
        "Opens settings TUI from command palette");
    
    // Rename Tab
    let rename_tab_enabled = kb.custom_commands.rename_tab;
    add_toggle_line(&mut lines, 2, app.field_index, "Rename Tab", rename_tab_enabled,
        "Opens rename prompt from command palette");
    
    lines.push(Line::from(""));
    
    // Mouse Bindings Section
    lines.push(Line::from(Span::styled(
        "Mouse Bindings",
        Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
    )));
    lines.push(Line::from(""));
    
    add_toggle_line(&mut lines, 3, app.field_index, "Ctrl+Click Open Link", kb.mouse.ctrl_click_open_link,
        "Ctrl+Click opens URLs under cursor");
    
    add_toggle_line(&mut lines, 4, app.field_index, "Right-Click Cmd Palette", kb.mouse.right_click_command_palette,
        "Right-click opens command palette");
    
    lines.push(Line::from(""));
    
    // General Settings Section
    lines.push(Line::from(Span::styled(
        "General Settings",
        Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
    )));
    lines.push(Line::from(""));
    
    add_toggle_line(&mut lines, 5, app.field_index, "Disable Default Keys", kb.disable_defaults,
        "Disable WezTerm's default keybindings");
    
    let leader_desc = format!("Leader: {} + {} ({}ms)", kb.leader.mods, kb.leader.key, kb.leader.timeout_ms);
    lines.push(Line::from(""));
    
    // Leader key toggle (manually constructed to avoid lifetime issues)
    let is_selected = app.field_index == 6;
    let style = if is_selected {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let prefix = if is_selected { "> " } else { "  " };
    let status = if kb.leader.enabled { "[ON] " } else { "[OFF]" };
    let status_style = if kb.leader.enabled {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Red)
    };
    
    lines.push(Line::from(vec![
        Span::raw(prefix),
        Span::styled(status, status_style),
        Span::styled(" Leader Key", style),
    ]));
    
    lines.push(Line::from(vec![
        Span::raw("       "),
        Span::styled(leader_desc, Style::default().fg(Color::DarkGray)),
    ]));
    
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    
    // Instructions
    lines.push(Line::from(Span::styled(
        "Press Enter to toggle  |  Ctrl+S to save",
        Style::default().fg(Color::DarkGray),
    )));
    
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(Color::DarkGray));
    
    f.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_right_column(f: &mut Frame, area: Rect, _app: &App, kb: &wezterm_settings_gui_lib::models::KeyBindingsConfig) {
    let mut lines: Vec<Line> = vec![];
    
    // Summary of key binding categories
    lines.push(Line::from(Span::styled(
        "Keybinding Categories",
        Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
    )));
    lines.push(Line::from(""));
    
    // Misc bindings
    lines.push(category_header("Misc/Utility"));
    add_binding_info(&mut lines, "Copy Mode", &kb.misc.copy_mode);
    add_binding_info(&mut lines, "Command Palette", &kb.misc.command_palette);
    add_binding_info(&mut lines, "Search", &kb.misc.search);
    add_binding_info(&mut lines, "Toggle Fullscreen", &kb.misc.toggle_fullscreen);
    lines.push(Line::from(""));
    
    // Tab bindings
    lines.push(category_header("Tabs"));
    add_binding_info(&mut lines, "New Tab", &kb.tabs.spawn_tab);
    add_binding_info(&mut lines, "Close Tab", &kb.tabs.close_tab);
    add_binding_info(&mut lines, "Next/Prev Tab", &kb.tabs.next_tab);
    add_binding_info(&mut lines, "Rename Tab", &kb.tabs.rename_tab);
    lines.push(Line::from(""));
    
    // Pane bindings
    lines.push(category_header("Panes"));
    add_binding_info(&mut lines, "Split Vertical", &kb.panes.split_vertical);
    add_binding_info(&mut lines, "Split Horizontal", &kb.panes.split_horizontal);
    add_binding_info(&mut lines, "Close Pane", &kb.panes.close_pane);
    add_binding_info(&mut lines, "Navigate", &kb.panes.nav_up);
    lines.push(Line::from(""));
    
    // Window bindings
    lines.push(category_header("Window"));
    add_binding_info(&mut lines, "New Window", &kb.windows.spawn_window);
    add_binding_info(&mut lines, "Maximize", &kb.windows.maximize_window);
    lines.push(Line::from(""));
    
    // Backdrop bindings
    lines.push(category_header("Backdrops"));
    add_binding_info(&mut lines, "Random", &kb.backdrops.random);
    add_binding_info(&mut lines, "Cycle", &kb.backdrops.cycle_forward);
    add_binding_info(&mut lines, "Select", &kb.backdrops.select);
    add_binding_info(&mut lines, "Toggle Focus", &kb.backdrops.toggle_focus);
    lines.push(Line::from(""));
    
    // Copy/Paste
    lines.push(category_header("Copy/Paste"));
    add_binding_info(&mut lines, "Copy", &kb.copy_paste.copy);
    add_binding_info(&mut lines, "Paste", &kb.copy_paste.paste);
    
    f.render_widget(Paragraph::new(lines), area);
}

fn add_toggle_line<'a>(lines: &mut Vec<Line<'a>>, field_idx: usize, current_idx: usize, name: &'a str, enabled: bool, description: &'a str) {
    let is_selected = current_idx == field_idx;
    let style = if is_selected {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let prefix = if is_selected { "> " } else { "  " };
    let status = if enabled { "[ON] " } else { "[OFF]" };
    let status_style = if enabled {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Red)
    };
    
    lines.push(Line::from(vec![
        Span::raw(prefix),
        Span::styled(status, status_style),
        Span::styled(format!(" {}", name), style),
    ]));
    
    lines.push(Line::from(vec![
        Span::raw("       "),
        Span::styled(description.to_string(), Style::default().fg(Color::DarkGray)),
    ]));
}

fn category_header(name: &str) -> Line<'static> {
    Line::from(Span::styled(
        format!("{}", name),
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ))
}

fn add_binding_info(lines: &mut Vec<Line>, name: &str, binding: &wezterm_settings_gui_lib::models::KeyBinding) {
    let status_color = if binding.enabled { Color::Green } else { Color::DarkGray };
    let key_display = if binding.mods == "NONE" {
        binding.key.clone()
    } else {
        format!("{}+{}", binding.mods, binding.key)
    };
    
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("{:<18}", name), Style::default().fg(status_color)),
        Span::styled(key_display, Style::default().fg(Color::Yellow)),
    ]));
}
