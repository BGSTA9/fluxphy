//! Real-time line graph plotting for FluxPhy

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame,
};

/// Render the flux rate graph
pub fn render_flux_graph(
    frame: &mut Frame,
    area: Rect,
    rate_history: &[(f64, f64)],
    max_rate: f64,
) {
    // Convert to owned data for the chart
    let data: Vec<(f64, f64)> = rate_history.to_vec();

    let datasets = vec![Dataset::default()
        .name("Flux Rate")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&data)];

    let max_time = rate_history.last().map(|(t, _)| *t).unwrap_or(10.0);
    let display_max_rate = if max_rate > 0.0 { max_rate * 1.1 } else { 100.0 };

    // Create labels as Line<'static>
    let x_labels: Vec<Line<'static>> = vec![
        Line::from("0s"),
        Line::from(format!("{:.1}s", max_time / 2.0)),
        Line::from(format!("{:.1}s", max_time)),
    ];

    let y_labels: Vec<Line<'static>> = vec![
        Line::from("0"),
        Line::from(format!("{:.0}", display_max_rate / 2.0)),
        Line::from(format!("{:.0}", display_max_rate)),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(" Flux Rate R(t) ")
                .title_style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .x_axis(
            Axis::default()
                .title("Time (s)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, max_time.max(1.0)])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Rate (MB/s)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, display_max_rate])
                .labels(y_labels),
        );

    frame.render_widget(chart, area);
}

/// Simple ASCII sparkline for quiet mode output
pub fn sparkline(data: &[f64], width: usize) -> String {
    if data.is_empty() {
        return " ".repeat(width);
    }

    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let range = max - min;

    // Spark characters from low to high
    const SPARKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    // Sample data to fit width
    let step = (data.len() as f64 / width as f64).max(1.0);
    let mut result = String::with_capacity(width);

    for i in 0..width {
        let idx = (i as f64 * step) as usize;
        if idx < data.len() {
            let normalized = if range > 0.0 {
                ((data[idx] - min) / range * 7.0) as usize
            } else {
                4
            };
            result.push(SPARKS[normalized.min(7)]);
        } else {
            result.push(' ');
        }
    }

    result
}
