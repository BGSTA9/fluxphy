//! Sparkline rendering using Braille patterns and Unicode blocks
//!
//! This module provides advanced sparkline visualizations for the FluxPhy TUI,
//! using Unicode Braille block characters (U+2800-U+28FF) for high-resolution
//! mini-charts.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render a Braille-pattern sparkline
/// 
/// Uses Unicode Braille characters to achieve 2×4 resolution per character cell
pub fn render_braille_sparkline(
    frame: &mut Frame,
    area: Rect,
    data: &[(f64, f64)],
    max_value: f64,
    color: Color,
    title: &str,
) {
    if data.is_empty() || area.width < 2 || area.height < 2 {
        return;
    }

    let sparkline_text = generate_braille_sparkline(data, max_value, area.width as usize - 2);
    
    let content = vec![Line::from(Span::styled(sparkline_text, Style::default().fg(color)))];
    
    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::NONE),
        );
    
    frame.render_widget(paragraph, area);
}

/// Generate Braille sparkline string from data points
fn generate_braille_sparkline(data: &[(f64, f64)], max_value: f64, width: usize) -> String {
    if data.is_empty() || max_value <= 0.0 {
        return "─".repeat(width);
    }

    // Sample data to fit width
    let step = if data.len() > width {
        data.len() / width
    } else {
        1
    };

    let mut result = String::new();
    
    for i in 0..width.min(data.len() / step.max(1)) {
        let idx = i * step;
        if idx < data.len() {
            let value = data[idx].1;
            let normalized = (value / max_value).clamp(0.0, 1.0);
            let char = braille_char_for_value(normalized);
            result.push(char);
        } else {
            result.push('⠀'); // Empty Braille
        }
    }

    result
}

/// Map normalized value (0.0-1.0) to Braille character height
fn braille_char_for_value(normalized: f64) -> char {
    // Braille patterns for vertical bars (approximation)
    // Using dots 1,2,3,4 on left side for height levels
    let chars = ['⠀', '⠁', '⠃', '⠇', '⠏', '⠟', '⠿', '⡿', '⣿'];
    let index = (normalized * (chars.len() - 1) as f64).round() as usize;
    chars[index.min(chars.len() - 1)]
}

/// Render horizontal comparison bars
pub fn render_comparison_bars(
    frame: &mut Frame,
    area: Rect,
    labels: &[&str],
    values: &[f64],
    max_value: f64,
    color: Color,
) {
    if labels.is_empty() || values.is_empty() || area.height < labels.len() as u16 {
        return;
    }

    let mut lines = Vec::new();
    
    for (i, (label, &value)) in labels.iter().zip(values.iter()).enumerate() {
        if i >= area.height as usize {
            break;
        }
        
        let normalized = if max_value > 0.0 {
            (value / max_value).clamp(0.0, 1.0)
        } else {
            0.0
        };
        
        let bar_width = ((area.width as f64 - 10.0) * normalized) as usize;
        let bar = "▓".repeat(bar_width);
        let empty = "░".repeat((area.width as usize).saturating_sub(bar_width + 10));
        
        lines.push(Line::from(vec![
            Span::raw(format!("{:>5}: ", label)),
            Span::styled(bar, Style::default().fg(color)),
            Span::styled(empty, Style::default().fg(Color::DarkGray)),
        ]));
    }
    
    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

/// Render vertical histogram bars
pub fn render_histogram(
    frame: &mut Frame,
    area: Rect,
    values: &[f64],
    max_value: f64,
    color: Color,
) {
    if values.is_empty() || area.width < 2 {
        return;
    }

    let height = area.height.saturating_sub(1) as usize;
    let width = area.width as usize;
    
    // Sample values to fit width
    let step = if values.len() > width {
        values.len() / width
    } else {
        1
    };

    let mut bars = String::new();
    
    for i in 0..width.min(values.len() / step.max(1)) {
        let idx = i * step;
        if idx < values.len() {
            let value = values[idx];
            let normalized = if max_value > 0.0 {
                (value / max_value).clamp(0.0, 1.0)
            } else {
                0.0
            };
            
            let bar_height = (normalized * height as f64).round() as usize;
            let char = if bar_height == 0 {
                '▁'
            } else if bar_height >= height {
                '█'
            } else {
                // Use block characters for different heights
                ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█']
                    [(bar_height * 8 / height.max(1)).min(7)]
            };
            bars.push(char);
        }
    }

    let content = vec![Line::from(Span::styled(bars, Style::default().fg(color)))];
    let paragraph = Paragraph::new(content);
    frame.render_widget(paragraph, area);
}

/// Render a circular progress indicator (ASCII pie chart)
pub fn render_circular_progress(progress: f64) -> Vec<String> {
    let percentage = (progress * 100.0).clamp(0.0, 100.0);
    let filled_eighths = ((percentage / 100.0) * 8.0).round() as usize;
    
    // Simple ASCII pie chart using Unicode pie symbols
    let pie_chars = ['○', '◔', '◑', '◕', '●'];
    let index = (filled_eighths * pie_chars.len() / 8).min(pie_chars.len() - 1);
    
    vec![
        format!("   {}   ", pie_chars[index]),
        format!(" {:.1}% ", percentage),
    ]
}

/// Render a segmented horizontal bar (for CPU/RAM)
pub fn render_segmented_bar(value: f64, max_width: usize, color: Color) -> Span<'static> {
    let normalized = (value / 100.0).clamp(0.0, 1.0);
    let filled = (normalized * max_width as f64).round() as usize;
    let empty = max_width.saturating_sub(filled);
    
    let bar = format!("{}{}", "▓".repeat(filled), "░".repeat(empty));
    Span::styled(bar, Style::default().fg(color))
}
