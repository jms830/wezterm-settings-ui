// Window settings panel

use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Draw the window settings panel
pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    let window = &app.config.window;
    
    let fields = vec![
        ("Opacity", format!("{:.2}", window.window_background_opacity), 1),
        ("Padding Left", format!("{}", window.window_padding.left), 2),
        ("Padding Right", format!("{}", window.window_padding.right), 3),
        ("Padding Top", format!("{}", window.window_padding.top), 4),
        ("Padding Bottom", format!("{}", window.window_padding.bottom), 5),
        ("Decorations", format!("{:?}", window.window_decorations), 6),
        ("Tab Bar", if window.enable_tab_bar { "Enabled" } else { "Disabled" }.to_string(), 7),
        ("Hide Tab If One", if window.hide_tab_bar_if_only_one_tab { "Yes" } else { "No" }.to_string(), 8),
        ("Fancy Tab Bar", if window.use_fancy_tab_bar { "Yes" } else { "No" }.to_string(), 9),
        ("Tab Max Width", format!("{}", window.tab_max_width), 10),
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
