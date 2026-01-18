//! TUI display layout and rendering for FluxPhy

use crate::analysis::{AnalysisResult, TimeSeriesModel, TrendStatus};
use crate::physics::{Bottleneck, FlowRegime, FluxStatistics, PhysicsMetrics, SystemConstraints};
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
    pub bottleneck: Bottleneck,
    /// System constraints for bottleneck analysis
    pub system_constraints: Option<SystemConstraints>,
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Memory usage percentage
    pub memory_usage: f32,
    /// Shannon entropy
    pub entropy: f64,
    /// System temperature (physics metric)
    pub system_temp: f64,
    /// Is paused
    pub paused: bool,
    /// Show help panel
    pub show_help: bool,
    /// File index (for multi-file)
    pub file_index: usize,
    /// Total file count
    pub total_files: usize,
    /// Path to generated dashboard (if any)
    pub dashboard_path: Option<std::path::PathBuf>,
    /// Transfer status for completion notification
    pub transfer_status: TransferStatus,
    /// Time-series analysis model
    pub time_series: TimeSeriesModel,
    /// Latest analysis result
    pub analysis_result: Option<AnalysisResult>,
}

/// Transfer completion status
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TransferStatus {
    #[default]
    InProgress,
    Success {
        duration: f64,
        size_bytes: u64,
        mean_rate: f64,
    },
    Failed {
        reason: String,
    },
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
            bottleneck: Bottleneck::Unknown,
            system_constraints: None,
            cpu_usage: 0.0,
            memory_usage: 0.0,
            entropy: 0.0,
            system_temp: 0.0,
            paused: false,
            show_help: false,
            file_index: 0,
            total_files: 1,
            dashboard_path: None,
            transfer_status: TransferStatus::InProgress,
            time_series: TimeSeriesModel::new(50), // Keep 50 samples history
            analysis_result: None,
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

            // Compute bottleneck from system constraints if available
            if let Some(ref constraints) = self.system_constraints {
                self.bottleneck = constraints.primary_bottleneck;
            } else if rates.len() >= 5 {
                // Compute bottleneck from available system metrics
                let constraints = SystemConstraints::analyze(
                    self.cpu_usage,
                    self.mean_rate, // Using mean rate as proxy for disk read
                    self.current_rate, // Using current rate as proxy for disk write
                    self.memory_usage,
                    self.current_rate,
                    self.peak_rate * 1.2,
                );
                self.bottleneck = constraints.primary_bottleneck;
                self.system_constraints = Some(constraints);
            }
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

    // Two-floor layout: Top floor (graphs) + Bottom floor (metrics)
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_chunks[1]);

    // Top floor: Throughput graph
    let max_rate = state.peak_rate.max(state.current_rate).max(1.0);
    render_flux_graph(frame, content_chunks[0], &state.rate_history, max_rate);

    // Bottom floor: 4 vertical sections for metrics
    let metrics_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),  // Transfer Info
            Constraint::Percentage(25),  // Rate Statistics
            Constraint::Percentage(25),  // Physics Metrics
            Constraint::Percentage(25),  // System Status
        ])
        .split(content_chunks[1]);

    render_transfer_info_panel(frame, metrics_chunks[0], state);
    render_rate_stats_panel(frame, metrics_chunks[1], state);
    render_physics_panel(frame, metrics_chunks[2], state);
    render_system_status_panel(frame, metrics_chunks[3], state);

    // Footer with controls
    render_footer(frame, main_chunks[2], state);

    // Help overlay (renders on top if show_help is true)
    if state.show_help {
        let help_area = centered_rect(60, 80, size);
        crate::ui::help::render_help_overlay(frame, help_area);
    }

    // Notification overlay for dashboard or completion
    if let Some(ref path) = state.dashboard_path {
        let notify_area = centered_rect(50, 20, size);
        render_notification(frame, notify_area, &format!(
            "📊 Dashboard saved!\n\n{}",
            path.display()
        ), Color::Cyan);
    } else if state.transfer_status != TransferStatus::InProgress {
        let notify_area = centered_rect(50, 30, size);
        match &state.transfer_status {
            TransferStatus::Success { duration, size_bytes, mean_rate } => {
                render_notification(frame, notify_area, &format!(
                    "✅ Transfer Complete!\n\nSize: {}\nDuration: {:.1}s\nMean Rate: {:.2} MB/s",
                    format_size(*size_bytes), duration, mean_rate
                ), Color::Green);
            }
            TransferStatus::Failed { reason } => {
                render_notification(frame, notify_area, &format!(
                    "❌ Transfer Failed!\n\n{}\n\nCheck logs for details.",
                    reason
                ), Color::Red);
            }
            _ => {}
        }
    }
}

/// Render a notification overlay
fn render_notification(frame: &mut Frame, area: Rect, message: &str, color: Color) {
    use ratatui::widgets::Clear;
    frame.render_widget(Clear, area);
    
    let lines: Vec<Line> = message.lines().map(|l| Line::from(l.to_string())).collect();
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Notification ")
                .title_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color)),
        )
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
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

/// Panel 1: Transfer Information
fn render_transfer_info_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let progress = state.progress();
    
    let info = vec![
        Line::from(vec![
            Span::styled("📁 Transfer", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("File: "),
            Span::styled(
                truncate_string(&state.current_file, 15),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Size: "),
            Span::styled(
                format_size(state.total_size),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Done: "),
            Span::styled(
                format_size(state.bytes_copied),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("▓ {:.1}%", progress),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Panel 2: Rate Statistics
fn render_rate_stats_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let info = vec![
        Line::from(vec![
            Span::styled("📊 Rates", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Current: "),
            Span::styled(
                format_rate(state.current_rate),
                if state.analysis_result.as_ref().map(|r| r.is_outlier).unwrap_or(false) {
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
                } else {
                    Style::default().fg(Color::Green)
                },
            ),
        ]),
        Line::from(vec![
            Span::raw("Mean: "),
            Span::styled(
                format_rate(state.mean_rate),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Peak: "),
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
    ];

    let paragraph = Paragraph::new(info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Panel 3: Physics Metrics
fn render_physics_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let info = vec![
        Line::from(vec![
            Span::styled("⚛️ Physics", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Flow: "),
            Span::styled(
                format!("{}", state.flow_regime),
                Style::default().fg(flow_regime_color(state.flow_regime)),
            ),
        ]),
        Line::from(vec![
            Span::raw("CV: "),
            Span::styled(
                format!("{:.4}", state.cv),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("σ: "),
            Span::styled(
                format!("{:.2} MB/s", state.std_dev),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Trend: "),
             match &state.analysis_result {
                Some(res) => {
                     let (text, color) = match res.trend_status {
                         TrendStatus::Accelerating => ("↗ Accel", Color::Green),
                         TrendStatus::Stable => ("→ Stable", Color::Cyan),
                         TrendStatus::Decelerating => ("↘ Decel", Color::Red),
                     };
                     Span::styled(text, Style::default().fg(color))
                },
                None => Span::raw("Analyzing..."),
            }
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Entropy: "),
            Span::styled(
                format!("{:.2} bits", state.entropy),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Temp: "),
            Span::styled(
                format!("{:.2}", state.system_temp),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Panel 4: System Status
fn render_system_status_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let info = vec![
        Line::from(vec![
            Span::styled("🖥️ System", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Bottleneck: "),
            Span::styled(
                format!("{}", state.bottleneck),
                Style::default().fg(bottleneck_color(state.bottleneck)),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("CPU: "),
            Span::styled(
                format!("{:.1}%", state.cpu_usage),
                Style::default().fg(if state.cpu_usage > 80.0 { Color::Red } else { Color::White }),
            ),
        ]),
        Line::from(vec![
            Span::raw("Memory: "),
            Span::styled(
                format!("{:.1}%", state.memory_usage),
                Style::default().fg(if state.memory_usage > 80.0 { Color::Red } else { Color::White }),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Stability: "),
            Span::styled(
                format!("{:.2}", state.thermal_stability),
                Style::default().fg(Color::Green),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame, area: Rect, state: &AppState) {
    let status = if state.paused { " [PAUSED] " } else { "" };

    let footer_text = format!(
        " [Q] Quit  [P] Pause  [R] Resume  [H] Help  [S] Dashboard{}  │  Flow: {}",
        status, state.flow_regime
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

fn bottleneck_color(bottleneck: Bottleneck) -> Color {
    match bottleneck {
        Bottleneck::InsufficientData => Color::DarkGray,
        Bottleneck::Unknown => Color::Green,
        Bottleneck::CPU => Color::Red,
        Bottleneck::Memory => Color::Red,
        Bottleneck::DiskRead => Color::Yellow,
        Bottleneck::DiskWrite => Color::Yellow,
        Bottleneck::Network => Color::Magenta,
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("...{}", &s[s.len() - max_len + 3..])
    }
}
