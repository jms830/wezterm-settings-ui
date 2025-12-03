// Themes panel - browse and select WezTerm color schemes

use crate::app::{App, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

pub fn draw(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Theme list
            Constraint::Percentage(40), // Preview
        ])
        .split(area);

    // Left side: search + theme list
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search/filter box
            Constraint::Min(0),    // Theme list
            Constraint::Length(2), // Current theme info
        ])
        .split(chunks[0]);

    draw_search_box(f, left_chunks[0], app);
    draw_theme_list(f, left_chunks[1], app);
    draw_current_theme(f, left_chunks[2], app);

    // Right side: preview
    draw_theme_preview(f, chunks[1], app);
}

fn draw_search_box(f: &mut Frame, area: Rect, app: &App) {
    let is_searching = app.input_mode == InputMode::Editing && app.field_index > 0;

    let search_text = if is_searching {
        format!("{}|", app.input_buffer)
    } else if app.theme_filter.is_empty() {
        "Press / to search themes...".to_string()
    } else {
        format!("{} (press / to edit)", app.theme_filter)
    };

    let (style, border_style) = if is_searching {
        (
            Style::default().fg(Color::Yellow),
            Style::default().fg(Color::Yellow),
        )
    } else if app.theme_filter.is_empty() {
        (
            Style::default().fg(Color::DarkGray),
            Style::default().fg(Color::DarkGray),
        )
    } else {
        (
            Style::default().fg(Color::White),
            Style::default().fg(Color::DarkGray),
        )
    };

    let title = if is_searching {
        " Search (Enter to apply, Esc to cancel) "
    } else {
        " Search "
    };

    let search = Paragraph::new(search_text)
        .style(style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        );

    f.render_widget(search, area);
}

fn draw_theme_list(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = app
        .filtered_themes
        .iter()
        .enumerate()
        .map(|(i, theme)| {
            let is_selected = i == app.theme_index && app.field_index > 0;
            let is_current = app.config.color_scheme.as_ref() == Some(theme);

            let prefix = if is_current { " " } else { "  " };
            let suffix = if is_current { " (current)" } else { "" };

            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if is_current {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(theme.clone(), style),
                Span::styled(suffix, if is_selected { style } else { Style::default().fg(Color::DarkGray) }),
            ]);

            ListItem::new(line)
        })
        .collect();

    let theme_count = format!(" Themes ({}) ", app.filtered_themes.len());
    let in_panel = app.field_index > 0;

    let border_style = if in_panel {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(theme_count),
        )
        .highlight_style(Style::default())
        .highlight_symbol("");

    // Render stateful list with scrolling
    f.render_stateful_widget(list, area, &mut app.theme_list_state);

    // Add scrollbar
    if app.filtered_themes.len() > area.height as usize - 2 {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state = ScrollbarState::new(app.filtered_themes.len())
            .position(app.theme_index);

        f.render_stateful_widget(
            scrollbar,
            area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 0 }),
            &mut scrollbar_state,
        );
    }
}

fn draw_current_theme(f: &mut Frame, area: Rect, app: &App) {
    let current = app
        .config
        .color_scheme
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("(none)");

    let info = Line::from(vec![
        Span::styled(" Current: ", Style::default().fg(Color::DarkGray)),
        Span::styled(current, Style::default().fg(Color::Cyan)),
        Span::styled(
            "  |  Enter: Apply  /: Search",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    f.render_widget(Paragraph::new(info), area);
}

fn draw_theme_preview(f: &mut Frame, area: Rect, app: &App) {
    let selected_theme = app.filtered_themes.get(app.theme_index);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" Preview ");

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(theme_name) = selected_theme {
        if let Some(colors) = get_theme_colors(theme_name) {
            draw_color_preview(f, inner, theme_name, &colors);
        } else {
            // No color data available
            let text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    format!("  {}", theme_name),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Preview not available",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Press Enter to apply",
                    Style::default().fg(Color::DarkGray),
                )),
            ];
            f.render_widget(Paragraph::new(text), inner);
        }
    }
}

fn draw_color_preview(f: &mut Frame, area: Rect, name: &str, colors: &ThemeColors) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Theme name
            Constraint::Length(3), // Terminal preview
            Constraint::Length(1), // Spacer
            Constraint::Length(2), // ANSI colors row 1
            Constraint::Length(2), // ANSI colors row 2
            Constraint::Min(0),    // Remaining
        ])
        .split(area);

    // Theme name
    let title = Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(Paragraph::new(vec![Line::from(""), title]), chunks[0]);

    // Terminal preview (bg + fg sample)
    let bg = parse_hex_color(&colors.background).unwrap_or(Color::Black);
    let fg = parse_hex_color(&colors.foreground).unwrap_or(Color::White);
    let cursor = parse_hex_color(&colors.cursor).unwrap_or(Color::White);

    let preview_text = vec![
        Line::from(vec![
            Span::styled("  $ ", Style::default().fg(Color::Green).bg(bg)),
            Span::styled("echo ", Style::default().fg(fg).bg(bg)),
            Span::styled("\"Hello, World!\"", Style::default().fg(Color::Yellow).bg(bg)),
            Span::styled("█", Style::default().fg(cursor).bg(bg)),
            Span::raw("  "),
        ]),
        Line::from(vec![
            Span::styled("  Hello, World!", Style::default().fg(fg).bg(bg)),
            Span::styled("          ", Style::default().bg(bg)),
        ]),
    ];
    f.render_widget(
        Paragraph::new(preview_text).block(Block::default()),
        chunks[1],
    );

    // ANSI colors - first 8 (normal)
    let ansi_line1: Vec<Span> = std::iter::once(Span::raw("  "))
        .chain(colors.ansi[0..8].iter().map(|c| {
            let color = parse_hex_color(c).unwrap_or(Color::White);
            Span::styled("  ", Style::default().bg(color))
        }))
        .collect();
    f.render_widget(Paragraph::new(Line::from(ansi_line1)), chunks[3]);

    // ANSI colors - bright 8
    let ansi_line2: Vec<Span> = std::iter::once(Span::raw("  "))
        .chain(colors.brights[0..8].iter().map(|c| {
            let color = parse_hex_color(c).unwrap_or(Color::White);
            Span::styled("  ", Style::default().bg(color))
        }))
        .collect();
    f.render_widget(Paragraph::new(Line::from(ansi_line2)), chunks[4]);
}

/// Parse hex color string to ratatui Color
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

/// Theme color palette
struct ThemeColors {
    foreground: String,
    background: String,
    cursor: String,
    ansi: [String; 8],
    brights: [String; 8],
}

/// Get colors for a known theme
fn get_theme_colors(name: &str) -> Option<ThemeColors> {
    match name {
        "Catppuccin Mocha" => Some(ThemeColors {
            foreground: "#cdd6f4".into(),
            background: "#1e1e2e".into(),
            cursor: "#f5e0dc".into(),
            ansi: ["#45475a", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#f5c2e7", "#94e2d5", "#bac2de"].map(String::from),
            brights: ["#585b70", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#f5c2e7", "#94e2d5", "#a6adc8"].map(String::from),
        }),
        "Catppuccin Macchiato" => Some(ThemeColors {
            foreground: "#cad3f5".into(),
            background: "#24273a".into(),
            cursor: "#f4dbd6".into(),
            ansi: ["#494d64", "#ed8796", "#a6da95", "#eed49f", "#8aadf4", "#f5bde6", "#8bd5ca", "#b8c0e0"].map(String::from),
            brights: ["#5b6078", "#ed8796", "#a6da95", "#eed49f", "#8aadf4", "#f5bde6", "#8bd5ca", "#a5adcb"].map(String::from),
        }),
        "Catppuccin Frappe" => Some(ThemeColors {
            foreground: "#c6d0f5".into(),
            background: "#303446".into(),
            cursor: "#f2d5cf".into(),
            ansi: ["#51576d", "#e78284", "#a6d189", "#e5c890", "#8caaee", "#f4b8e4", "#81c8be", "#b5bfe2"].map(String::from),
            brights: ["#626880", "#e78284", "#a6d189", "#e5c890", "#8caaee", "#f4b8e4", "#81c8be", "#a5adce"].map(String::from),
        }),
        "Catppuccin Latte" => Some(ThemeColors {
            foreground: "#4c4f69".into(),
            background: "#eff1f5".into(),
            cursor: "#dc8a78".into(),
            ansi: ["#5c5f77", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#ea76cb", "#179299", "#acb0be"].map(String::from),
            brights: ["#6c6f85", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#ea76cb", "#179299", "#bcc0cc"].map(String::from),
        }),
        "Dracula" => Some(ThemeColors {
            foreground: "#f8f8f2".into(),
            background: "#282a36".into(),
            cursor: "#f8f8f2".into(),
            ansi: ["#21222c", "#ff5555", "#50fa7b", "#f1fa8c", "#bd93f9", "#ff79c6", "#8be9fd", "#f8f8f2"].map(String::from),
            brights: ["#6272a4", "#ff6e6e", "#69ff94", "#ffffa5", "#d6acff", "#ff92df", "#a4ffff", "#ffffff"].map(String::from),
        }),
        "Gruvbox Dark" | "Gruvbox dark, medium (base16)" => Some(ThemeColors {
            foreground: "#ebdbb2".into(),
            background: "#282828".into(),
            cursor: "#ebdbb2".into(),
            ansi: ["#282828", "#cc241d", "#98971a", "#d79921", "#458588", "#b16286", "#689d6a", "#a89984"].map(String::from),
            brights: ["#928374", "#fb4934", "#b8bb26", "#fabd2f", "#83a598", "#d3869b", "#8ec07c", "#ebdbb2"].map(String::from),
        }),
        "Gruvbox Light" => Some(ThemeColors {
            foreground: "#3c3836".into(),
            background: "#fbf1c7".into(),
            cursor: "#3c3836".into(),
            ansi: ["#fbf1c7", "#cc241d", "#98971a", "#d79921", "#458588", "#b16286", "#689d6a", "#7c6f64"].map(String::from),
            brights: ["#928374", "#9d0006", "#79740e", "#b57614", "#076678", "#8f3f71", "#427b58", "#3c3836"].map(String::from),
        }),
        "Nord" | "Nord (Gogh)" => Some(ThemeColors {
            foreground: "#d8dee9".into(),
            background: "#2e3440".into(),
            cursor: "#d8dee9".into(),
            ansi: ["#3b4252", "#bf616a", "#a3be8c", "#ebcb8b", "#81a1c1", "#b48ead", "#88c0d0", "#e5e9f0"].map(String::from),
            brights: ["#4c566a", "#bf616a", "#a3be8c", "#ebcb8b", "#81a1c1", "#b48ead", "#8fbcbb", "#eceff4"].map(String::from),
        }),
        "Tokyo Night" | "Tokyo Night Storm" | "tokyonight" | "tokyonight_night" | "tokyonight_storm" => Some(ThemeColors {
            foreground: "#c0caf5".into(),
            background: "#1a1b26".into(),
            cursor: "#c0caf5".into(),
            ansi: ["#15161e", "#f7768e", "#9ece6a", "#e0af68", "#7aa2f7", "#bb9af7", "#7dcfff", "#a9b1d6"].map(String::from),
            brights: ["#414868", "#f7768e", "#9ece6a", "#e0af68", "#7aa2f7", "#bb9af7", "#7dcfff", "#c0caf5"].map(String::from),
        }),
        "Solarized Dark" | "Solarized Dark Higher Contrast" => Some(ThemeColors {
            foreground: "#839496".into(),
            background: "#002b36".into(),
            cursor: "#839496".into(),
            ansi: ["#073642", "#dc322f", "#859900", "#b58900", "#268bd2", "#d33682", "#2aa198", "#eee8d5"].map(String::from),
            brights: ["#002b36", "#cb4b16", "#586e75", "#657b83", "#839496", "#6c71c4", "#93a1a1", "#fdf6e3"].map(String::from),
        }),
        "Solarized Light" => Some(ThemeColors {
            foreground: "#657b83".into(),
            background: "#fdf6e3".into(),
            cursor: "#657b83".into(),
            ansi: ["#073642", "#dc322f", "#859900", "#b58900", "#268bd2", "#d33682", "#2aa198", "#eee8d5"].map(String::from),
            brights: ["#002b36", "#cb4b16", "#586e75", "#657b83", "#839496", "#6c71c4", "#93a1a1", "#fdf6e3"].map(String::from),
        }),
        "One Dark" | "OneDark" => Some(ThemeColors {
            foreground: "#abb2bf".into(),
            background: "#282c34".into(),
            cursor: "#528bff".into(),
            ansi: ["#282c34", "#e06c75", "#98c379", "#e5c07b", "#61afef", "#c678dd", "#56b6c2", "#abb2bf"].map(String::from),
            brights: ["#545862", "#e06c75", "#98c379", "#e5c07b", "#61afef", "#c678dd", "#56b6c2", "#c8ccd4"].map(String::from),
        }),
        "Monokai" | "Monokai Pro" | "Monokai Remastered" => Some(ThemeColors {
            foreground: "#f8f8f2".into(),
            background: "#272822".into(),
            cursor: "#f8f8f2".into(),
            ansi: ["#272822", "#f92672", "#a6e22e", "#f4bf75", "#66d9ef", "#ae81ff", "#a1efe4", "#f8f8f2"].map(String::from),
            brights: ["#75715e", "#f92672", "#a6e22e", "#f4bf75", "#66d9ef", "#ae81ff", "#a1efe4", "#f9f8f5"].map(String::from),
        }),
        "Kanagawa" | "Kanagawa Wave" | "Kanagawa Dragon" => Some(ThemeColors {
            foreground: "#dcd7ba".into(),
            background: "#1f1f28".into(),
            cursor: "#c8c093".into(),
            ansi: ["#16161d", "#c34043", "#76946a", "#c0a36e", "#7e9cd8", "#957fb8", "#6a9589", "#c8c093"].map(String::from),
            brights: ["#727169", "#e82424", "#98bb6c", "#e6c384", "#7fb4ca", "#938aa9", "#7aa89f", "#dcd7ba"].map(String::from),
        }),
        "rose-pine" | "rose-pine-moon" => Some(ThemeColors {
            foreground: "#e0def4".into(),
            background: "#191724".into(),
            cursor: "#524f67".into(),
            ansi: ["#26233a", "#eb6f92", "#31748f", "#f6c177", "#9ccfd8", "#c4a7e7", "#ebbcba", "#e0def4"].map(String::from),
            brights: ["#6e6a86", "#eb6f92", "#31748f", "#f6c177", "#9ccfd8", "#c4a7e7", "#ebbcba", "#e0def4"].map(String::from),
        }),
        "rose-pine-dawn" => Some(ThemeColors {
            foreground: "#575279".into(),
            background: "#faf4ed".into(),
            cursor: "#9893a5".into(),
            ansi: ["#f2e9de", "#b4637a", "#286983", "#ea9d34", "#56949f", "#907aa9", "#d7827e", "#575279"].map(String::from),
            brights: ["#9893a5", "#b4637a", "#286983", "#ea9d34", "#56949f", "#907aa9", "#d7827e", "#575279"].map(String::from),
        }),
        "GitHub Dark" => Some(ThemeColors {
            foreground: "#c9d1d9".into(),
            background: "#0d1117".into(),
            cursor: "#c9d1d9".into(),
            ansi: ["#484f58", "#ff7b72", "#3fb950", "#d29922", "#58a6ff", "#bc8cff", "#39c5cf", "#b1bac4"].map(String::from),
            brights: ["#6e7681", "#ffa198", "#56d364", "#e3b341", "#79c0ff", "#d2a8ff", "#56d4dd", "#f0f6fc"].map(String::from),
        }),
        "GitHub Light" => Some(ThemeColors {
            foreground: "#24292f".into(),
            background: "#ffffff".into(),
            cursor: "#044289".into(),
            ansi: ["#24292f", "#cf222e", "#116329", "#4d2d00", "#0969da", "#8250df", "#1b7c83", "#6e7781"].map(String::from),
            brights: ["#57606a", "#a40e26", "#1a7f37", "#633c01", "#218bff", "#a475f9", "#3192aa", "#8c959f"].map(String::from),
        }),
        "Everforest Dark" => Some(ThemeColors {
            foreground: "#d3c6aa".into(),
            background: "#2d353b".into(),
            cursor: "#d3c6aa".into(),
            ansi: ["#475258", "#e67e80", "#a7c080", "#dbbc7f", "#7fbbb3", "#d699b6", "#83c092", "#d3c6aa"].map(String::from),
            brights: ["#475258", "#e67e80", "#a7c080", "#dbbc7f", "#7fbbb3", "#d699b6", "#83c092", "#d3c6aa"].map(String::from),
        }),
        "Material Dark" | "Material" => Some(ThemeColors {
            foreground: "#eeffff".into(),
            background: "#263238".into(),
            cursor: "#ffcc00".into(),
            ansi: ["#263238", "#ff5370", "#c3e88d", "#ffcb6b", "#82aaff", "#c792ea", "#89ddff", "#eeffff"].map(String::from),
            brights: ["#546e7a", "#ff5370", "#c3e88d", "#ffcb6b", "#82aaff", "#c792ea", "#89ddff", "#ffffff"].map(String::from),
        }),
        "Ayu Dark" => Some(ThemeColors {
            foreground: "#e6e1cf".into(),
            background: "#0f1419".into(),
            cursor: "#f29718".into(),
            ansi: ["#000000", "#ff3333", "#b8cc52", "#e7c547", "#36a3d9", "#f07178", "#95e6cb", "#ffffff"].map(String::from),
            brights: ["#323232", "#ff6565", "#eafe84", "#fff779", "#68d5ff", "#ffa3aa", "#c7fffd", "#ffffff"].map(String::from),
        }),
        "Ayu Light" => Some(ThemeColors {
            foreground: "#5c6773".into(),
            background: "#fafafa".into(),
            cursor: "#ff6a00".into(),
            ansi: ["#000000", "#ff3333", "#86b300", "#f29718", "#41a6d9", "#f07178", "#4dbf99", "#ffffff"].map(String::from),
            brights: ["#323232", "#ff6565", "#b8e532", "#ffc94a", "#73d8ff", "#ffa3aa", "#7ff1cb", "#ffffff"].map(String::from),
        }),
        "Nightfly" => Some(ThemeColors {
            foreground: "#c3ccdc".into(),
            background: "#011627".into(),
            cursor: "#80a4c2".into(),
            ansi: ["#1d3b53", "#fc514e", "#a1cd5e", "#e7d37a", "#82aaff", "#c792ea", "#7fdbca", "#a1aab8"].map(String::from),
            brights: ["#7c8f8f", "#ff5874", "#21c7a8", "#ecc48d", "#82aaff", "#ae81ff", "#7fdbca", "#d6deeb"].map(String::from),
        }),
        "Night Owl" => Some(ThemeColors {
            foreground: "#d6deeb".into(),
            background: "#011627".into(),
            cursor: "#80a4c2".into(),
            ansi: ["#011627", "#ef5350", "#22da6e", "#addb67", "#82aaff", "#c792ea", "#21c7a8", "#ffffff"].map(String::from),
            brights: ["#575656", "#ef5350", "#22da6e", "#ffeb95", "#82aaff", "#c792ea", "#7fdbca", "#ffffff"].map(String::from),
        }),
        "Zenburn" => Some(ThemeColors {
            foreground: "#dcdccc".into(),
            background: "#3f3f3f".into(),
            cursor: "#dcdccc".into(),
            ansi: ["#1e2320", "#d78787", "#60b48a", "#dfaf8f", "#506070", "#dc8cc3", "#8cd0d3", "#dcdccc"].map(String::from),
            brights: ["#709080", "#dca3a3", "#c3bf9f", "#f0dfaf", "#94bff3", "#ec93d3", "#93e0e3", "#ffffff"].map(String::from),
        }),
        _ => None,
    }
}
