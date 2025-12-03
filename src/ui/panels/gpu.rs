// GPU settings panel

use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Draw the GPU settings panel
pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let gpu = &app.config.gpu;
    
    let fields = vec![
        ("Frontend", format!("{:?}", gpu.front_end), 1),
        ("Power Preference", format!("{:?}", gpu.webgpu_power_preference), 2),
        ("Max FPS", format!("{}", gpu.max_fps), 3),
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
