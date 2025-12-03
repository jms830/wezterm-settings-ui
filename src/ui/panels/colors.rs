// Colors settings panel

use crate::app::App;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Draw the colors settings panel
pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(2), // Section header
            Constraint::Length(8), // Core colors
            Constraint::Length(2), // Section header
            Constraint::Length(4), // ANSI colors
            Constraint::Length(2), // Section header  
            Constraint::Length(4), // Bright colors
            Constraint::Min(0),    // Remaining space
        ])
        .split(area);

    // Core Colors section
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(" Core Colors", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]),
    ]);
    f.render_widget(header, chunks[0]);

    let colors = &app.config.colors;
    let fields = vec![
        ("Foreground", &colors.foreground, 1),
        ("Background", &colors.background, 2),
        ("Cursor BG", &colors.cursor_bg, 3),
        ("Cursor FG", &colors.cursor_fg, 4),
        ("Cursor Border", &colors.cursor_border, 5),
        ("Selection BG", &colors.selection_bg, 6),
        ("Selection FG", &colors.selection_fg, 7),
    ];

    let core_lines: Vec<Line> = fields
        .iter()
        .map(|(label, value, idx)| {
            let is_selected = app.field_index == *idx;
            let (style, label_style) = if is_selected {
                (
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )
            } else {
                (
                    Style::default().fg(Color::Gray),
                    Style::default().fg(Color::White),
                )
            };
            
            let prefix = if is_selected { " ▸ " } else { "   " };
            let swatch = color_swatch(value);
            
            Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{:<14}", label), label_style),
                Span::raw(" "),
                Span::styled(format!("{:<9}", value), style),
                Span::raw(" "),
                swatch,
            ])
        })
        .collect();

    f.render_widget(Paragraph::new(core_lines), chunks[1]);

    // ANSI Colors section
    let ansi_header = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(" ANSI Colors (0-7)", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]),
    ]);
    f.render_widget(ansi_header, chunks[2]);

    let ansi_lines: Vec<Line> = colors
        .ansi
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let idx = 8 + i; // Offset for field index
            let is_selected = app.field_index == idx;
            let style = if is_selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let prefix = if is_selected { "> " } else { "  " };
            let swatch = color_swatch(color);
            
            Line::from(vec![
                Span::raw(prefix),
                Span::styled(format!("[{}]", i), style),
                Span::raw(" "),
                Span::styled(format!("{:<9}", color), style),
                Span::raw(" "),
                swatch,
            ])
        })
        .collect();

    // Display ANSI colors in 2 rows of 4
    let ansi_text: Vec<Line> = ansi_lines
        .chunks(4)
        .map(|chunk| {
            Line::from(
                chunk
                    .iter()
                    .flat_map(|l| l.spans.clone())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    f.render_widget(Paragraph::new(ansi_text), chunks[3]);

    // Bright Colors section
    let brights_header = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(" Bright Colors (8-15)", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]),
    ]);
    f.render_widget(brights_header, chunks[4]);

    let brights_lines: Vec<Line> = colors
        .brights
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let idx = 16 + i; // Offset for field index
            let is_selected = app.field_index == idx;
            let style = if is_selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let prefix = if is_selected { "> " } else { "  " };
            let swatch = color_swatch(color);
            
            Line::from(vec![
                Span::raw(prefix),
                Span::styled(format!("[{}]", i + 8), style),
                Span::raw(" "),
                Span::styled(format!("{:<9}", color), style),
                Span::raw(" "),
                swatch,
            ])
        })
        .collect();

    let brights_text: Vec<Line> = brights_lines
        .chunks(4)
        .map(|chunk| {
            Line::from(
                chunk
                    .iter()
                    .flat_map(|l| l.spans.clone())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    f.render_widget(Paragraph::new(brights_text), chunks[5]);
}

/// Create a color swatch span from a hex color
fn color_swatch(hex: &str) -> Span<'static> {
    if let Some(color) = parse_hex_color(hex) {
        Span::styled("██", Style::default().fg(color))
    } else {
        Span::styled("??", Style::default().fg(Color::Red))
    }
}

/// Parse a hex color string to a ratatui Color
fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    
    Some(Color::Rgb(r, g, b))
}
