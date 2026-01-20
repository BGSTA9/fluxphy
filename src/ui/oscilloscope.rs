//! Oscilloscope waveform visualization for entropy and signal analysis
//!
//! Renders bi-directional waveforms centered on a zero-axis using
//! Unicode box-drawing characters.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Render an oscilloscope-style waveform visualization
pub fn render_oscilloscope(
    frame: &mut Frame,
    area: Rect,
    entropy_values: &[f64],
    color: Color,
) {
    if entropy_values.is_empty() || area.width < 4 || area.height < 3 {
        return;
    }

    let height = area.height as usize;
    let width = area.width as usize;
    let mid_line = height / 2;

    // Create waveform
    let waveform = generate_waveform(entropy_values, width, height);
    
    let mut lines = Vec::new();
    for (i, row) in waveform.iter().enumerate() {
        if i == mid_line {
            // Zero axis line
            lines.push(Line::from(vec![
                Span::styled(row, Style::default().fg(color)),
            ]));
        } else {
            lines.push(Line::from(Span::styled(row, Style::default().fg(color))));
        }
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

/// Generate waveform grid from entropy values
fn generate_waveform(values: &[f64], width: usize, height: usize) -> Vec<String> {
    let mut grid = vec![vec![' '; width]; height];
    let mid = height / 2;

    if values.is_empty() {
        // Draw zero line
        for row in grid.iter_mut() {
            row[0] = '│';
        }
        grid[mid] = vec!['─'; width];
        grid[mid][0] = '┼';
        return grid.iter().map(|row| row.iter().collect()).collect();
    }

    // Normalize values to fit in display
    let max_val = values.iter().fold(0.0f64, |a, &b| a.max(b.abs()));
    let scale = if max_val > 0.0 { 
        (mid as f64) / max_val 
    } else { 
        1.0 
    };

    // Sample values to fit width
    let step = if values.len() > width {
        values.len() as f64 / width as f64
    } else {
        1.0
    };

    // Draw waveform
    for x in 0..width.min(values.len()) {
        let idx = (x as f64 * step) as usize;
        if idx >= values.len() {
            break;
        }
        
        let value = values[idx];
        let normalized = value * scale;
        let y = (mid as i32 - normalized as i32).clamp(0, height as i32 - 1) as usize;
        
        // Draw point and connecting line
        if x > 0 {
            let prev_idx = ((x - 1) as f64 * step) as usize;
            if prev_idx < values.len() {
                let prev_value = values[prev_idx];
                let prev_y = (mid as i32 - (prev_value * scale) as i32)
                    .clamp(0, height as i32 - 1) as usize;
                
                // Connect points
                draw_line(&mut grid, x - 1, prev_y, x, y);
            }
        }
        
        grid[y][x] = '●';
    }

    // Draw axes
    for y in 0..height {
        grid[y][0] = if y == mid { '┼' } else { '│' };
    }
    for x in 0..width {
        if grid[mid][x] == ' ' {
            grid[mid][x] = '─';
        }
    }

    grid.iter().map(|row| row.iter().collect()).collect()
}

/// Draw a line between two points using simple vertical characters
fn draw_line(grid: &mut [Vec<char>], _x1: usize, y1: usize, x2: usize, y2: usize) {
    if y1 == y2 {
        return; // Horizontal line, skip
    }

    let y_start = y1.min(y2);
    let y_end = y1.max(y2);
    
    for y in y_start..=y_end {
        if y < grid.len() && x2 < grid[0].len() {
            if grid[y][x2] == ' ' {
                grid[y][x2] = '│';
            }
        }
    }
}

/// Generate entropy-based "jitter" signal for oscilloscope
pub fn generate_entropy_signal(entropy: f64, history_size: usize) -> Vec<f64> {
    // Generate synthetic entropy signal based on current entropy value
    // This creates a jittery waveform that increases with entropy
    let mut signal = Vec::with_capacity(history_size);
    
    for i in 0..history_size {
        let phase = i as f64 * 0.5;
        let noise = (entropy * 2.0 - 1.0) * ((phase * 0.7).sin() * 0.5 + (phase * 1.3).cos() * 0.3);
        signal.push(noise);
    }
    
    signal
}
