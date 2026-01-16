//! TUI display layout and rendering for FluxPhy

use crate::physics::{FlowRegime, FluxStatistics, PhysicsMetrics};
use crate::ui::plot::render_flux_graph;
use crate::utils::{format_duration, format_rate, format_size};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

/// Application state for the TUI
pub struct AppState {
    /// Current file being copied
    pub current_file: String,
    /// Total file size
    pub total_size: u64,
    /// Bytes copied so far
    pub bytes_copied: u64,
    /// Current transfer rate (MB/s)
    pub current_rate: f64,
    /// Mean transfer rate (MB/s)
    pub mean_rate: f64,
    /// Peak transfer rate (MB/s)
    pub peak_rate: f64,
    /// Elapsed time in seconds
    pub elapsed_secs: f64,
    /// Rate history for plotting
    pub rate_history: Vec<(f64, f64)>,
    /// Flow regime
    pub flow_regime: FlowRegime,
    /// Thermal stability
    pub thermal_stability: f64,
    /// Flux density
    pub flux_density: f64,
    /// Variance
    pub variance: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Coefficient of variation
    pub cv: f64,
    /// Current bottleneck
    pub bottleneck: String,
    /// System temperature
    pub system_temp: f64,
    /// Shannon entropy
    pub entropy: f64,
    /// Is paused
    pub paused: bool,
    /// File index (for multi-file)
    pub file_index: usize,
    /// Total file count
    pub total_files: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_file: String::new(),
            total_size: 0,
            bytes_copied: 0,
            current_rate: 0.0,
            mean_rate: 0.0,
            peak_rate: 0.0,
            elapsed_secs: 0.0,
            rate_history: Vec::new(),
            flow_regime: FlowRegime::Laminar,
            thermal_stability: 1.0,
            flux_density: 0.0,
            variance: 0.0,
            std_dev: 0.0,
            cv: 0.0,
            bottleneck: "Unknown".to_string(),
            system_temp: 0.0,
            entropy: 0.0,
            paused: false,
            file_index: 0,
            total_files: 1,
        }
    }
}

impl AppState {
    /// Update statistics from rate history
    pub fn update_stats(&mut self) {
        let rates: Vec<f64> = self.rate_history.iter().map(|(_, r)| *r).collect();
        if !rates.is_empty() {
            let stats = FluxStatistics::from_samples(&rates);
            self.mean_rate = stats.mean_rate;
            self.peak_rate = stats.peak_rate;
            self.variance = stats.variance;
            self.std_dev = stats.std_dev;
            self.cv = stats.coefficient_of_variation;
            self.flow_regime = FlowRegime::from_cv(self.cv);

            let physics = PhysicsMetrics::calculate(&stats, self.peak_rate * 1.2);
            self.thermal_stability = physics.thermal_stability;
            self.flux_density = physics.flux_density;
            self.system_temp = physics.system_temperature;
            self.entropy = PhysicsMetrics::calculate_entropy(&rates);
        }
    }

    /// Get progress percentage
    pub fn progress(&self) -> f64 {
        if self.total_size == 0 {
            return 0.0;
        }
        (self.bytes_copied as f64 / self.total_size as f64) * 100.0
    }

    /// Calculate ETA
    pub fn eta(&self) -> f64 {
        if self.mean_rate <= 0.0 || self.bytes_copied >= self.total_size {
            return 0.0;
        }
        let remaining_bytes = self.total_size - self.bytes_copied;
        let remaining_mb = remaining_bytes as f64 / (1024.0 * 1024.0);
        remaining_mb / self.mean_rate
    }
}

/// Render the complete TUI layout
pub fn render_ui(frame: &mut Frame, state: &AppState) {
    let size = frame.area();

    // Main layout: header + content + footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Content
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    // Render header
    render_header(frame, main_chunks[0], state);

    // Content: 50% metrics panel, 50% graph
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_chunks[1]);

    // Left panel: metrics
    render_metrics_panel(frame, content_chunks[0], state);

    // Right panel: graph
    let max_rate = state.peak_rate.max(state.current_rate).max(1.0);
    render_flux_graph(frame, content_chunks[1], &state.rate_history, max_rate);

    // Footer with controls
    render_footer(frame, main_chunks[2], state);
}

fn render_header(frame: &mut Frame, area: Rect, state: &AppState) {
    let title = if state.total_files > 1 {
        format!(
            " FluxPhy Transfer Status [{}/{}] ",
            state.file_index + 1,
            state.total_files
        )
    } else {
        " FluxPhy Transfer Status ".to_string()
    };

    let header = Block::default()
        .title(title)
        .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(header, area);
}

fn render_metrics_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Transfer info
            Constraint::Length(2),  // Progress bar
            Constraint::Min(8),     // Physics metrics
        ])
        .margin(1)
        .split(area);

    // Transfer info section
    let transfer_info = vec![
        Line::from(vec![
            Span::styled("╔════════════════════╗", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("║ Transfer Metrics   ║", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("╚════════════════════╝", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("File: "),
            Span::styled(
                truncate_string(&state.current_file, 25),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Size: "),
            Span::styled(
                format_size(state.total_size),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let transfer_para = Paragraph::new(transfer_info);
    frame.render_widget(transfer_para, inner[0]);

    // Progress bar
    let progress = state.progress();
    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .percent(progress as u16)
        .label(format!("{:.1}%", progress));

    frame.render_widget(gauge, inner[1]);

    // Physics metrics section
    let physics_info = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("Flux Rate: "),
            Span::styled(
                format_rate(state.current_rate),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("Mean Rate: "),
            Span::styled(
                format_rate(state.mean_rate),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Peak Rate: "),
            Span::styled(
                format_rate(state.peak_rate),
                Style::default().fg(Color::Magenta),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Elapsed: "),
            Span::styled(
                format_duration(state.elapsed_secs),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("ETA: "),
            Span::styled(
                format_duration(state.eta()),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("╔════════════════════╗", Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("║ Physics Metrics    ║", Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("╚════════════════════╝", Style::default().fg(Color::Blue)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Variance: "),
            Span::styled(
                format!("{:.2} MB²/s²", state.variance),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Std Dev: "),
            Span::styled(
                format!("{:.2} MB/s", state.std_dev),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("CV: "),
            Span::styled(
                format!("{:.4} ({})", state.cv, state.flow_regime),
                Style::default().fg(flow_regime_color(state.flow_regime)),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Bottleneck: "),
            Span::styled(&state.bottleneck, Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("System Temp: "),
            Span::styled(
                format!("{:.3}", state.system_temp),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Entropy: "),
            Span::styled(
                format!("{:.2} bits", state.entropy),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let physics_para = Paragraph::new(physics_info).wrap(Wrap { trim: true });
    frame.render_widget(physics_para, inner[2]);

    // Outer block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    frame.render_widget(block, area);
}

fn render_footer(frame: &mut Frame, area: Rect, state: &AppState) {
    let status = if state.paused { " [PAUSED] " } else { "" };

    let footer_text = format!(
        " [Q] Quit  [P] Pause  [R] Resume  [S] Save Metrics{}  │  Flow: {} │ Stability: {:.2}",
        status, state.flow_regime, state.thermal_stability
    );

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    frame.render_widget(footer, area);
}

fn flow_regime_color(regime: FlowRegime) -> Color {
    match regime {
        FlowRegime::Laminar => Color::Green,
        FlowRegime::Transitional => Color::Yellow,
        FlowRegime::Turbulent => Color::Red,
        FlowRegime::Chaotic => Color::Magenta,
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("...{}", &s[s.len() - max_len + 3..])
    }
}
