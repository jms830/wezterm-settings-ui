// UI module - rendering logic

mod layout;
mod panels;
mod widgets;

use crate::app::{App, InputMode, Panel};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

/// Main draw function
pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title bar
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(f.area());

    draw_title_bar(f, chunks[0], app);
    draw_main_content(f, chunks[1], app);
    draw_status_bar(f, chunks[2], app);

    // Draw overlays
    match app.input_mode {
        InputMode::Help => draw_help_overlay(f, app),
        InputMode::Confirm => draw_confirm_dialog(f, app),
        _ => {}
    }
}

fn draw_title_bar(f: &mut Frame, area: Rect, app: &mut App) {
    let icon = "󰖲"; // WezTerm-like terminal icon
    let title = " WezTerm Settings";
    let modified = if app.has_changes { "  [modified]" } else { "" };
    
    let save_hint = "Ctrl+S: Save  ?: Help";
    let used_len = icon.len() + title.len() + modified.len() + save_hint.len() + 2;
    let padding = (area.width as usize).saturating_sub(used_len);
    
    let title_line = Line::from(vec![
        Span::styled(format!(" {}", icon), Style::default().fg(Color::Magenta)),
        Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(modified, Style::default().fg(Color::Yellow)),
        Span::raw(" ".repeat(padding.max(1))),
        Span::styled(save_hint, Style::default().fg(Color::DarkGray)),
    ]);

    f.render_widget(
        Paragraph::new(title_line).style(Style::default().bg(Color::Rgb(30, 30, 46))),
        area,
    );
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(16), // Sidebar
            Constraint::Min(0),     // Panel content
        ])
        .split(area);

    draw_sidebar(f, chunks[0], app);
    draw_panel(f, chunks[1], app);
}

fn draw_sidebar(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = Panel::all()
        .iter()
        .enumerate()
        .map(|(i, panel)| {
            let is_selected = i == app.sidebar_index;
            let is_focused = is_selected && app.field_index == 0;

            let (style, icon_style) = if is_focused {
                (
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    Style::default().fg(Color::Yellow),
                )
            } else if is_selected {
                (
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    Style::default().fg(Color::Cyan),
                )
            } else {
                (
                    Style::default().fg(Color::Gray),
                    Style::default().fg(Color::DarkGray),
                )
            };

            let prefix = if is_focused { "▸ " } else { "  " };
            
            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{} ", panel.icon()), icon_style),
                Span::styled(panel.name(), style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let border_style = if app.field_index == 0 {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let sidebar = List::new(items)
        .block(
            Block::default()
                .borders(Borders::RIGHT)
                .border_style(border_style)
                .title(" Settings ")
        );

    f.render_widget(sidebar, area);
}

fn draw_panel(f: &mut Frame, area: Rect, app: &mut App) {
    let title = format!(" {} {} ", app.current_panel.icon(), app.current_panel.name());
    
    let border_style = if app.field_index > 0 {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title)
        .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let inner = block.inner(area);
    f.render_widget(block, area);

    match app.current_panel {
        Panel::Themes => panels::themes::draw(f, inner, app),
        Panel::Colors => panels::colors::draw(f, inner, app),
        Panel::Fonts => panels::fonts::draw(f, inner, app),
        Panel::Window => panels::window::draw(f, inner, app),
        Panel::Cursor => panels::cursor::draw(f, inner, app),
        Panel::Gpu => panels::gpu::draw(f, inner, app),
        Panel::Keybindings => panels::keybindings::draw(f, inner, app),
    }
}

fn draw_status_bar(f: &mut Frame, area: Rect, app: &mut App) {
    let mode_indicator = match app.input_mode {
        InputMode::Normal => ("NORMAL", Color::Blue),
        InputMode::Editing => ("EDIT", Color::Green),
        InputMode::Help => ("HELP", Color::Yellow),
        InputMode::Confirm => ("CONFIRM", Color::Red),
    };

    let status = if let Some(ref msg) = app.status_message {
        vec![
            Span::styled(
                format!(" {} ", mode_indicator.0),
                Style::default().fg(Color::Black).bg(mode_indicator.1).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(msg.as_str(), Style::default().fg(Color::Green)),
        ]
    } else {
        let hints = match app.input_mode {
            InputMode::Normal => {
                if app.current_panel == Panel::Themes && app.field_index > 0 {
                    "j/k: Browse  /: Search  Enter: Apply  h: Back  q: Quit"
                } else {
                    "j/k: Navigate  l/Enter: Select  Tab: Next  ?: Help  q: Quit"
                }
            }
            InputMode::Editing => "Enter: Apply  Esc: Cancel",
            InputMode::Help => "Press any key to close",
            InputMode::Confirm => "y: Quit  n: Cancel  s: Save & Quit",
        };
        vec![
            Span::styled(
                format!(" {} ", mode_indicator.0),
                Style::default().fg(Color::Black).bg(mode_indicator.1).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(hints, Style::default().fg(Color::DarkGray)),
        ]
    };

    f.render_widget(
        Paragraph::new(Line::from(status)).style(Style::default().bg(Color::Rgb(30, 30, 46))),
        area,
    );
}

fn draw_help_overlay(f: &mut Frame, _app: &App) {
    let area = centered_rect(55, 75, f.area());
    
    let help_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Navigation", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(Span::styled("  ─────────────────────────────", Style::default().fg(Color::DarkGray))),
        Line::from(vec![
            Span::styled("  j / ↓      ", Style::default().fg(Color::Yellow)),
            Span::styled("Move down", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  k / ↑      ", Style::default().fg(Color::Yellow)),
            Span::styled("Move up", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  h / ←      ", Style::default().fg(Color::Yellow)),
            Span::styled("Back to sidebar", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  l / → / ⏎  ", Style::default().fg(Color::Yellow)),
            Span::styled("Enter / Edit", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Tab        ", Style::default().fg(Color::Yellow)),
            Span::styled("Next field", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Shift+Tab  ", Style::default().fg(Color::Yellow)),
            Span::styled("Previous field", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Actions", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(Span::styled("  ─────────────────────────────", Style::default().fg(Color::DarkGray))),
        Line::from(vec![
            Span::styled("  Ctrl+S     ", Style::default().fg(Color::Yellow)),
            Span::styled("Save config", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  ?          ", Style::default().fg(Color::Yellow)),
            Span::styled("Toggle help", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  q / Esc    ", Style::default().fg(Color::Yellow)),
            Span::styled("Quit", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Themes Panel", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(Span::styled("  ─────────────────────────────", Style::default().fg(Color::DarkGray))),
        Line::from(vec![
            Span::styled("  /          ", Style::default().fg(Color::Yellow)),
            Span::styled("Search themes", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Enter      ", Style::default().fg(Color::Yellow)),
            Span::styled("Apply selected theme", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Editing", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(Span::styled("  ─────────────────────────────", Style::default().fg(Color::DarkGray))),
        Line::from(vec![
            Span::styled("  ⏎          ", Style::default().fg(Color::Yellow)),
            Span::styled("Apply changes", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Esc        ", Style::default().fg(Color::Yellow)),
            Span::styled("Cancel edit", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(help_lines)
        .block(
            Block::default()
                .title(" Help ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .style(Style::default().bg(Color::Rgb(30, 30, 46))),
        );

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn draw_confirm_dialog(f: &mut Frame, _app: &App) {
    let area = centered_rect(45, 30, f.area());

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ⚠ ", Style::default().fg(Color::Yellow)),
            Span::styled("You have unsaved changes.", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  (y) ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled("Quit without saving", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  (s) ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("Save and quit", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  (n) ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::styled("Cancel", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Unsaved Changes ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .style(Style::default().bg(Color::Rgb(30, 30, 46))),
        );

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
