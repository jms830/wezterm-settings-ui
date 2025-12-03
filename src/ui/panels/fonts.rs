// Fonts settings panel

use crate::app::{App, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Draw the fonts settings panel
pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let fonts = &app.config.fonts;
    
    // Split area: left for current settings, right for font selector
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);
    
    // Left side: current settings and other font options
    draw_settings(f, chunks[0], app, fonts);
    
    // Right side: font selector list
    draw_font_selector(f, chunks[1], app);
}

fn draw_settings(f: &mut Frame, area: Rect, app: &App, fonts: &wezterm_settings_gui_lib::models::FontConfig) {
    let fields = vec![
        ("Current Font", fonts.family.clone(), 1),
        ("Font Size", format!("{}", fonts.size), 2),
        ("Weight", fonts.weight.as_ref().map(|w| format!("{:?}", w)).unwrap_or_else(|| "Default".to_string()), 3),
        ("Load Target", fonts.freetype_load_target.as_ref().map(|t| format!("{:?}", t)).unwrap_or_else(|| "Normal".to_string()), 4),
        ("Render Target", fonts.freetype_render_target.as_ref().map(|t| format!("{:?}", t)).unwrap_or_else(|| "Normal".to_string()), 5),
    ];

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            "Settings",
            Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )),
        Line::from(""),
    ];

    for (label, value, idx) in &fields {
        let is_selected = app.field_index == *idx && app.field_index <= 5;
        let style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        
        let prefix = if is_selected { "> " } else { "  " };
        
        lines.push(Line::from(vec![
            Span::raw(prefix),
            Span::styled(format!("{:<14}", label), style),
            Span::raw(" "),
            Span::styled(value.clone(), style),
        ]));
    }

    // Add hint
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press 'l' or Right to select font →",
        Style::default().fg(Color::DarkGray),
    )));

    f.render_widget(Paragraph::new(lines), area);
}

fn draw_font_selector(f: &mut Frame, area: Rect, app: &App) {
    let is_in_selector = app.field_index > 5;
    
    // Search bar at top
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);
    
    // Search input
    let search_style = if app.input_mode == InputMode::Editing && is_in_selector {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    
    let search_text = if app.input_mode == InputMode::Editing && is_in_selector {
        format!("Search: {}_", app.input_buffer)
    } else if !app.font_filter.is_empty() {
        format!("Search: {} (press '/' to edit)", app.font_filter)
    } else {
        "Press '/' to search fonts".to_string()
    };
    
    let search_block = Block::default()
        .borders(Borders::ALL)
        .border_style(search_style)
        .title("Font Search");
    
    f.render_widget(
        Paragraph::new(search_text).block(search_block),
        chunks[0],
    );
    
    // Font list
    let list_style = if is_in_selector {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    
    let items: Vec<ListItem> = app
        .filtered_fonts
        .iter()
        .map(|font| {
            let is_current = font == &app.config.fonts.family;
            let style = if is_current {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let prefix = if is_current { "● " } else { "  " };
            ListItem::new(format!("{}{}", prefix, font)).style(style)
        })
        .collect();
    
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(list_style)
                .title(format!("Fonts ({} available)", app.filtered_fonts.len())),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");
    
    f.render_stateful_widget(list, chunks[1], &mut app.font_list_state.clone());
}
