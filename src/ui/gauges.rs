//! Analog gauge rendering for temperature and pressure metrics
//!
//! Provides semi-circular "speedometer" style gauges with color gradients.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

/// Render a semi-circular analog gauge (speedometer style)
pub fn render_analog_gauge(value: f64, max_value: f64, min_color: Color, max_color: Color) -> Vec<Line<'static>> {
    let normalized = (value / max_value).clamp(0.0, 1.0);
    
    // ASCII semi-circular gauge
    let gauge_color = interpolate_color(normalized, min_color, max_color);
    
    // Simple gauge representation using arc characters
    let segments = 10;
    let filled_segments = (normalized * segments as f64).round() as usize;
    
    let mut gauge = String::new();
    for i in 0..segments {
        if i < filled_segments {
            gauge.push('▓');
        } else {
            gauge.push('░');
        }
    }
    
    let needle = if filled_segments == 0 {
        "|"
    } else if filled_segments >= segments {
        "|"
    } else {
        "↑"
    };
    
    vec![
        Line::from(vec![
            Span::raw("╭"),
            Span::styled(gauge, Style::default().fg(gauge_color)),
            Span::raw("╮"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("{:^10}", needle),
                Style::default().fg(gauge_color),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("{:.1}/{:.1}", value, max_value),
                Style::default().fg(Color::White),
            ),
        ]),
    ]
}

/// Interpolate between two colors based on value
fn interpolate_color(t: f64, color1: Color, color2: Color) -> Color {
    // Simple interpolation between predefined colors
    if t < 0.33 {
        color1 // Blue (cold)
    } else if t < 0.67 {
        Color::Yellow // Yellow (warm)
    } else {
        color2 // Red (hot)
    }
}

/// Render a simple horizontal gauge bar
pub fn render_horizontal_gauge(value: f64, width: usize, color: Color) -> Span<'static> {
    let normalized = (value / 100.0).clamp(0.0, 1.0);
    let filled = (normalized * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    
    let bar = format!("╢{}{}╟", "█".repeat(filled), "─".repeat(empty));
    Span::styled(bar, Style::default().fg(color))
}

/// Generate a turbulence/heatmap visualization using Braille patterns
pub fn render_turbulence_field(entropy: f64, cv: f64, width: usize, height: usize) -> Vec<String> {
    let mut field = Vec::new();
    
    // Generate "smoke" effect based on entropy and CV
    let intensity = (entropy * cv).clamp(0.0, 1.0);
    
    // Braille patterns for different densities
    let patterns = [' ', '⠂', '⠆', '⠖', '⠶', '⡶', '⣶', '⣾', '⣿'];
    
    for y in 0..height {
        let mut row = String::new();
        for x in 0..width {
            // Create pseudo-random pattern based on position and intensity
            let seed = (x + y * y) as f64 * 0.1;
            let noise = ((seed.sin() * 12.9898 + seed.cos() * 78.233) * 43758.5453).fract();
            let threshold = noise * intensity;
            
            let index = (threshold * patterns.len() as f64) as usize;
            row.push(patterns[index.min(patterns.len() - 1)]);
        }
        field.push(row);
    }
    
    field
}
