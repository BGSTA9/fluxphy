//! FluxPhy - Physics of Flux File Transfer Tool
//!
//! A cross-platform TUI/CLI file copy tool that provides deep instrumentation
//! into the "physics" of data flux.

mod analysis;
mod cli;
mod config;

mod dashboard;
mod error;
mod flux;
mod metrics;
mod physics;
mod provenance;
mod ui;
mod utils;
mod validation;
mod welcome;

use crate::cli::Cli;
use crate::config::Config;
use crate::error::{FluxError, FluxResult};
use crate::validation::Validator;
use crate::flux::{FluxCopier, ProgressUpdate};
use crate::metrics::TransferMetrics;
use crate::physics::SystemConstraints;
use crate::ui::{render_ui, AppState};
use crate::utils::{format_duration, format_rate, format_size};

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use indicatif::{ProgressBar, ProgressStyle};
use ratatui::prelude::*;
use std::io::{self, stdout};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::System;
use tokio::sync::mpsc;

/// Run the TUI version
async fn run_tui(
    cli: &Cli,
    total_size: u64,
    mut progress_rx: mpsc::UnboundedReceiver<ProgressUpdate>,
    cancelled: Arc<AtomicBool>,
) -> FluxResult<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::default();
    state.total_size = total_size;
    state.current_file = cli.sources()[0]
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let start_time = Instant::now();
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    // System info for bottleneck detection
    let mut sys = System::new();
    let mut last_sysinfo_update = Instant::now();

    loop {
        // Check for cancellation
        if cancelled.load(Ordering::Relaxed) {
            break;
        }

        // Refresh sysinfo periodically (every 500ms to avoid overhead)
        if last_sysinfo_update.elapsed() >= Duration::from_millis(500) {
            sys.refresh_cpu_usage();
            sys.refresh_memory();
            state.cpu_usage = sys.global_cpu_usage();
            state.memory_usage = (sys.used_memory() as f64 / sys.total_memory().max(1) as f64 * 100.0) as f32;
            last_sysinfo_update = Instant::now();
        }

        // Draw UI
        terminal.draw(|f| render_ui(f, &state))?;

        // Handle events with timeout
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            cancelled.store(true, Ordering::Relaxed);
                            break;
                        }
                        KeyCode::Char('p') | KeyCode::Char('P') => {
                            state.paused = true;
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            state.paused = false;
                        }
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            // Generate HTML dashboard
                            if let Ok(path) = crate::dashboard::generate_dashboard(&state) {
                                state.dashboard_path = Some(path);
                            }
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') => {
                            state.show_help = !state.show_help;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Process progress updates
        while let Ok(update) = progress_rx.try_recv() {
            state.bytes_copied = update.bytes_copied;
            if update.current_rate > 0.0 {
                state.time_series.add_sample(update.current_rate);
                state.analysis_result = state.time_series.analyze();
            }

            state.current_rate = update.current_rate;
            state.mean_rate = update.mean_rate;
            state.peak_rate = update.peak_rate;
            
            // Update physics metrics
            // We need to pass the history to physics metrics update if we want real-time regime change
            // For now, let's keep the simplified CV calculation in flux.rs or here.
            // But we DO have rate_history in state.
            
            state.rate_history.push((update.elapsed_secs, update.current_rate)); 

            if let Some(file) = update.current_file {
                state.current_file = file;
            }
            state.file_index = update.file_index;
            state.total_files = update.total_files;
            state.update_stats();

            // Check if transfer complete
            if state.bytes_copied >= state.total_size {
                // Set completion status
                state.transfer_status = crate::ui::TransferStatus::Success {
                    duration: state.elapsed_secs,
                    size_bytes: state.total_size,
                    mean_rate: state.mean_rate,
                };
                // Wait a moment to show completion
                tokio::time::sleep(Duration::from_millis(1500)).await;
                break;
            }
        }

        if last_tick.elapsed() >= tick_rate {
            state.elapsed_secs = start_time.elapsed().as_secs_f64();
            last_tick = Instant::now();
        }

        // Check if complete
        if state.bytes_copied >= state.total_size {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

/// Run the quiet (non-TUI) version with simple progress bar
async fn run_quiet(
    cli: &Cli,
    total_size: u64,
    mut progress_rx: mpsc::UnboundedReceiver<ProgressUpdate>,
) -> FluxResult<()> {
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    println!(
        "Copying: {} → {}",
        cli.sources()[0].display(),
        cli.destination().display()
    );

    while let Some(update) = progress_rx.recv().await {
        pb.set_position(update.bytes_copied);
        if update.bytes_copied >= total_size {
            break;
        }
    }

    pb.finish_with_message("Complete");
    Ok(())
}

/// Perform the actual file transfer
async fn perform_transfer(
    cli: &Cli,
    progress_tx: mpsc::UnboundedSender<ProgressUpdate>,
) -> FluxResult<(FluxCopier, u64, usize)> {
    let mut copier = FluxCopier::new(cli.buffer_size, cli.sample_rate);
    let sources = cli.sources();
    let dest = cli.destination();

    let mut total_bytes = 0u64;
    let mut file_count = 0usize;

    for source in sources {
        if source.is_dir() {
            if !cli.recursive {
                return Err(FluxError::RecursiveRequired);
            }
            let (count, bytes) = copier.copy_directory(source, dest, Some(&progress_tx))?;
            file_count += count;
            total_bytes += bytes;
        } else {
            // Optional: Run validation before transfer
            // For now, we just log/warn if invalid, but proceed (or could skip)
            let validator = validation::MagicBytesValidator;
            if let Ok(validation::ValidationResult::Invalid(reason)) = validator.validate(source) {
                eprintln!("Warning: Skipping invalid file {}: {}", source.display(), reason);
                continue;
            }

            let bytes = copier.copy_file(source, dest, Some(&progress_tx))?;
            file_count += 1;
            total_bytes += bytes;
        }
    }

    // Verify if requested
    if cli.verify {
        for source in sources {
            let dest_path = if dest.is_dir() {
                dest.join(source.file_name().unwrap_or_default())
            } else {
                dest.to_path_buf()
            };
            FluxCopier::verify_copy(source, &dest_path)?;
        }
    }

    Ok((copier, total_bytes, file_count))
}

fn print_summary(metrics: &TransferMetrics) {
    println!();
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║              FluxPhy Transfer Complete                    ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("File: {}", metrics.source);
    println!("Size: {}", format_size(metrics.file_size_bytes));
    println!("Time: {}", format_duration(metrics.total_time_seconds));
    println!("Mean Rate: {}", format_rate(metrics.statistics.mean_rate));
    println!("Flow Regime: {}", metrics.physics_metrics.flow_regime);
    println!(
        "Thermal Stability: {:.2}",
        metrics.physics_metrics.thermal_stability
    );
    println!();
}

#[tokio::main]
async fn main() -> FluxResult<()> {
    let cli = Cli::parse();

    // Load config (use defaults if not present)
    let _config = Config::load().unwrap_or_default();

    // Validate paths
    for source in cli.sources() {
        if !source.exists() {
            return Err(FluxError::SourceNotFound(source.display().to_string()));
        }
    }

    // Calculate total size first
    let total_size = FluxCopier::calculate_total_size(cli.sources(), cli.recursive)?;

    // Create progress channel
    let (progress_tx, progress_rx) = mpsc::unbounded_channel();
    let cancelled = Arc::new(AtomicBool::new(false));
    let cancelled_clone = cancelled.clone();

    // Spawn transfer task
    let cli_clone = cli.clone();
    let transfer_handle = tokio::spawn(async move {
        perform_transfer(&cli_clone, progress_tx).await
    });

    // Run UI
    if cli.quiet {
        run_quiet(&cli, total_size, progress_rx).await?;
    } else {
        // Display welcome screen before TUI
        welcome::display_welcome_with_delay(1500);
        run_tui(&cli, total_size, progress_rx, cancelled_clone).await?;
    }

    // Wait for transfer to complete
    let (mut copier, total_bytes, file_count) = transfer_handle.await.map_err(|e| {
        FluxError::IoError(io::Error::new(io::ErrorKind::Other, e.to_string()))
    })??;

    // Generate metrics
    let elapsed = copier.get_elapsed().as_secs_f64();
    let rate_history = copier.take_rate_history();

    // Get system info for constraints
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu_usage = sys.global_cpu_usage();
    let memory_usage = (sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0) as f32;

    let source_str = cli.sources()[0].display().to_string();
    let dest_str = cli.destination().display().to_string();

    let mut metrics = TransferMetrics::new(
        source_str.clone(),
        dest_str.clone(),
        total_size,
        file_count,
        elapsed,
        rate_history,
        500.0, // Theoretical max (should be configurable)
    );

    let constraints = SystemConstraints::analyze(
        cpu_usage,
        0.0, // disk read proxy
        metrics.statistics.mean_rate, // disk write proxy
        memory_usage,
        metrics.statistics.mean_rate,
        500.0,
    );
    metrics.update_system_constraints(constraints);

    // --- PROVENANCE GENERATION (Phase 7) ---
    let mut prov = provenance::ProvRecord::new();
    let agent = provenance::capture_agent();
    let agent_id = agent.id.clone();
    prov.graph.push(provenance::ProvElement::Agent(agent));

    // Source Entity
    let source_id = format!("urn:file:{}", uuid::Uuid::new_v4());
    prov.graph.push(provenance::ProvElement::Entity(provenance::ProvEntity {
        id: source_id.clone(),
        was_attributed_to: agent_id.clone(),
        path: source_str,
        size: total_size, // Assuming single file size proxy
        checksum: None, // TODO: Add real checksum
    }));

    // Destination Entity
    let dest_id = format!("urn:file:{}", uuid::Uuid::new_v4());
    prov.graph.push(provenance::ProvElement::Entity(provenance::ProvEntity {
        id: dest_id.clone(),
        was_attributed_to: agent_id.clone(),
        path: dest_str,
        size: total_bytes,
        checksum: None,
    }));

    // Activity
    let activity_id = provenance::generate_activity_id();
    prov.graph.push(provenance::ProvElement::Activity(provenance::ProvActivity {
        id: activity_id,
        start_time: chrono::Local::now().to_rfc3339(),
        end_time: Some(chrono::Local::now().to_rfc3339()),
        was_associated_with: vec![agent_id],
        used: vec![source_id],
        generated: vec![dest_id],
        mean_rate: Some(metrics.statistics.mean_rate),
        flow_regime: Some(format!("{:?}", metrics.physics_metrics.flow_regime)),
    }));

    // Save provenance file
    if let Ok(dest_path) = std::fs::canonicalize(cli.destination()) {
         let prov_path = if dest_path.is_dir() {
             dest_path.join("provenance.json")
         } else {
             // If dest is file, put prov in same dir
             dest_path.parent().unwrap_or(Path::new(".")).join("provenance.json")
         };
         
         if let Err(e) = prov.save(&prov_path) {
             eprintln!("Warning: Failed to save provenance record: {}", e);
         } else if !cli.quiet {
             println!("Provenance record saved to: {}", prov_path.display());
         }
    }

    // Print summary
    print_summary(&metrics);

    // Show analysis if requested
    if cli.analyze {
        println!("{}", metrics.analysis_report());
    }

    // Save metrics
    let metrics_path = cli
        .metrics_file
        .clone()
        .unwrap_or_else(TransferMetrics::default_filename);
    metrics.save_to_file(&metrics_path)?;
    println!("Metrics saved to: {}", metrics_path.display());

    Ok(())
}

impl Clone for Cli {
    fn clone(&self) -> Self {
        Self {
            paths: self.paths.clone(),
            recursive: self.recursive,
            quiet: self.quiet,
            physics_verbose: self.physics_verbose,
            analyze: self.analyze,
            metrics_file: self.metrics_file.clone(),
            verify: self.verify,
            buffer_size: self.buffer_size,
            sample_rate: self.sample_rate,
            force: self.force,
            color: self.color,
        }
    }
}
