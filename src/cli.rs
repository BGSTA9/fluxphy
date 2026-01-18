//! CLI argument parsing for FluxPhy

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "fluxphy")]
#[command(author = "Argo Navis Research Laboratory")]
#[command(version = "0.1.0")]
#[command(about = "A file transfer tool with deep instrumentation into the physics of data flux", long_about = None)]
pub struct Cli {
    /// Display system info (neofetch style)
    #[arg(long)]
    pub fetch: bool,

    /// Source file(s) and destination path (last argument is destination)
    #[arg(num_args = 0..)]
    pub paths: Vec<PathBuf>,

    /// Copy directories recursively
    #[arg(short, long)]
    pub recursive: bool,

    /// Quiet mode - no TUI, minimal output
    #[arg(short, long)]
    pub quiet: bool,

    /// Enable verbose physics analysis
    #[arg(long)]
    pub physics_verbose: bool,

    /// Enable detailed analysis and reporting
    #[arg(short, long)]
    pub analyze: bool,

    /// Custom metrics output file (default: fluxphy_metrics_<timestamp>.json)
    #[arg(long, value_name = "FILE")]
    pub metrics_file: Option<PathBuf>,

    /// Verify file integrity with checksum after copy
    #[arg(long)]
    pub verify: bool,

    /// Buffer size in MB (default: 8)
    #[arg(long, default_value = "8")]
    pub buffer_size: usize,

    /// Sample rate in milliseconds (default: 100)
    #[arg(long, default_value = "100")]
    pub sample_rate: u64,

    /// Overwrite existing files without prompting
    #[arg(short, long)]
    pub force: bool,

    /// Color output mode
    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorMode,
}

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum ColorMode {
    #[default]
    Auto,
    Always,
    Never,
}

impl Cli {
    /// Get source paths (all paths except the last one)
    pub fn sources(&self) -> &[PathBuf] {
        &self.paths[..self.paths.len() - 1]
    }

    /// Get destination path (the last path)
    pub fn destination(&self) -> &PathBuf {
        self.paths.last().expect("At least 2 paths required")
    }
}
