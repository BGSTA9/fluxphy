//! Branding and welcome message for FluxPhy

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// ASCII art logo for FluxPhy
pub const LOGO: &str = r#"
  _____ _            ____  _           
 |  ___| |_   ___  _|  _ \| |__  _   _ 
 | |_  | | | | \ \/ / |_) | '_ \| | | |
 |  _| | | |_| |>  <|  __/| | | | |_| |
 |_|   |_|\__,_/_/\_\_|   |_| |_|\__, |
                                 |___/ 
"#;

/// Get the logo as styled Lines
pub fn get_logo_lines() -> Vec<Line<'static>> {
    LOGO.lines()
        .map(|line| {
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ))
        })
        .collect()
}

/// Get a welcome message describing the tool's capabilities
pub fn get_welcome_message() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "⚡ The Physics of Data Transfer",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("FluxPhy treats your file transfers like a physics experiment,"),
        ]),
        Line::from(vec![
            Span::raw("measuring the 'flux' of data as it flows from source to destination."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("📊 ", Style::default().fg(Color::Green)),
            Span::raw("Real-time throughput visualization"),
        ]),
        Line::from(vec![
            Span::styled("⚛️ ", Style::default().fg(Color::Blue)),
            Span::raw("Physics-inspired metrics (entropy, stability, flow regime)"),
        ]),
        Line::from(vec![
            Span::styled("🔍 ", Style::default().fg(Color::Magenta)),
            Span::raw("Bottleneck detection (CPU, disk, memory)"),
        ]),
        Line::from(vec![
            Span::styled("📈 ", Style::default().fg(Color::Yellow)),
            Span::raw("Statistical analysis (variance, CV, predictions)"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Press any key to start...",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    ]
}
