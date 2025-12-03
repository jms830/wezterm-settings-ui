// Cursor settings panel

use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Draw the cursor settings panel
pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let cursor = &app.config.cursor;
    
    let fields = vec![
        ("Cursor Style", format!("{:?}", cursor.default_cursor_style), 1),
        ("Blink Rate (ms)", format!("{}", cursor.cursor_blink_rate), 2),
        ("Blink Ease In", format!("{:?}", cursor.cursor_blink_ease_in), 3),
        ("Blink Ease Out", format!("{:?}", cursor.cursor_blink_ease_out), 4),
        ("Animation FPS", format!("{}", cursor.animation_fps), 5),
    ];

    let lines: Vec<Line> = fields
        .iter()
        .map(|(label, value, idx)| {
            let is_selected = app.field_index == *idx;
            let style = if is_selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let prefix = if is_selected { "> " } else { "  " };
            
            Line::from(vec![
                Span::raw(prefix),
                Span::styled(format!("{:<18}", label), style),
                Span::raw(" "),
                Span::styled(value.clone(), style),
            ])
        })
        .collect();

    f.render_widget(Paragraph::new(lines), area);
}
