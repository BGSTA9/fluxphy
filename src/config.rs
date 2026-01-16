//! Configuration file support for FluxPhy

use crate::error::{FluxError, FluxResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub ui: UiConfig,
    pub analysis: AnalysisConfig,
    pub behavior: BehaviorConfig,
    pub advanced: AdvancedConfig,
}

/// General transfer settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// Buffer size in MB
    pub buffer_size: usize,
    /// Sample rate in milliseconds
    pub sample_rate: u64,
    /// Always save metrics after transfer
    pub save_metrics: bool,
    /// Default metrics directory
    pub metrics_dir: String,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiConfig {
    /// Color theme: "default", "nord", "dracula", "solarized"
    pub theme: String,
    /// Show real-time graph by default
    pub show_graph: bool,
    /// Graph update interval in milliseconds
    pub graph_update_interval: u64,
}

/// Analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AnalysisConfig {
    /// Enable physics analysis by default
    pub analyze: bool,
    /// Verbose physics output
    pub physics_verbose: bool,
    /// Automatically detect bottlenecks
    pub detect_bottlenecks: bool,
}

/// Behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BehaviorConfig {
    /// Overwrite files without prompting
    pub force: bool,
    /// Verify checksums after copy
    pub verify: bool,
    /// Maximum concurrent file copies for directories
    pub max_concurrent: usize,
}

/// Advanced configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AdvancedConfig {
    /// Use direct I/O for large files
    pub use_direct_io: bool,
    /// Enable adaptive buffer sizing
    pub adaptive_buffer: bool,
    /// Log level
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            ui: UiConfig::default(),
            analysis: AnalysisConfig::default(),
            behavior: BehaviorConfig::default(),
            advanced: AdvancedConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            buffer_size: 8,
            sample_rate: 100,
            save_metrics: true,
            metrics_dir: "~/.local/share/fluxphy/metrics".to_string(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            show_graph: true,
            graph_update_interval: 100,
        }
    }
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            analyze: false,
            physics_verbose: false,
            detect_bottlenecks: true,
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            force: false,
            verify: false,
            max_concurrent: 4,
        }
    }
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            use_direct_io: false,
            adaptive_buffer: true,
            log_level: "info".to_string(),
        }
    }
}

impl Config {
    /// Load configuration from default path
    pub fn load() -> FluxResult<Self> {
        let config_path = Self::default_path()?;

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            toml::from_str(&contents).map_err(FluxError::from)
        } else {
            Ok(Self::default())
        }
    }

    /// Get the default configuration file path
    pub fn default_path() -> FluxResult<PathBuf> {
        dirs::config_dir()
            .map(|p| p.join("fluxphy").join("config.toml"))
            .ok_or(FluxError::ConfigNotFound)
    }

    /// Save configuration to file
    pub fn save(&self) -> FluxResult<()> {
        let config_path = Self::default_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| FluxError::ConfigError(e.to_string()))?;
        fs::write(config_path, contents)?;
        Ok(())
    }

    /// Create a default config file
    pub fn create_default() -> FluxResult<PathBuf> {
        let config = Self::default();
        config.save()?;
        Self::default_path()
    }
}
