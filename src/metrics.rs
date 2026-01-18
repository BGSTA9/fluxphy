//! Metrics collection and JSON serialization for FluxPhy

use crate::physics::{
    Accuracy, Bottleneck, FlowRegime, FluxStatistics, PhysicsMetrics, PredictionMetrics,
    SystemConstraints,
};
use crate::utils::generate_transfer_id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Complete transfer metrics including all physics analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferMetrics {
    /// Unique transfer identifier
    pub transfer_id: String,
    /// ISO timestamp of transfer start
    pub timestamp: DateTime<Utc>,
    /// Source path(s)
    pub source: String,
    /// Destination path
    pub destination: String,
    /// Total file size in bytes
    pub file_size_bytes: u64,
    /// Total transfer time in seconds
    pub total_time_seconds: f64,
    /// Number of files transferred
    pub file_count: usize,
    /// Statistical analysis
    pub statistics: FluxStatistics,
    /// Physics-inspired metrics
    pub physics_metrics: PhysicsMetrics,
    /// System constraint analysis
    pub system_constraints: SystemConstraints,
    /// Prediction accuracy metrics
    pub prediction: PredictionMetrics,
    /// Complete rate history: (time, rate) pairs
    pub rate_history: Vec<(f64, f64)>,
}

impl TransferMetrics {
    /// Create new metrics from transfer data
    pub fn new(
        source: String,
        destination: String,
        file_size_bytes: u64,
        file_count: usize,
        total_time_seconds: f64,
        rate_history: Vec<(f64, f64)>,
        theoretical_max_rate: f64,
    ) -> Self {
        let rates: Vec<f64> = rate_history.iter().map(|(_, r)| *r).collect();
        let statistics = FluxStatistics::from_samples(&rates);

        let mut physics_metrics = PhysicsMetrics::calculate(&statistics, theoretical_max_rate);
        physics_metrics.shannon_entropy = PhysicsMetrics::calculate_entropy(&rates);

        // Get initial rate estimate from first 5 samples (~500ms)
        let initial_rate = if rates.len() >= 5 {
            rates[..5].iter().sum::<f64>() / 5.0
        } else if !rates.is_empty() {
            rates.iter().sum::<f64>() / rates.len() as f64
        } else {
            0.0
        };

        let prediction = PredictionMetrics::calculate(initial_rate, file_size_bytes, total_time_seconds);

        Self {
            transfer_id: generate_transfer_id(),
            timestamp: Utc::now(),
            source,
            destination,
            file_size_bytes,
            total_time_seconds,
            file_count,
            statistics,
            physics_metrics,
            system_constraints: SystemConstraints::default(),
            prediction,
            rate_history,
        }
    }

    /// Update system constraints based on sysinfo data
    pub fn update_system_constraints(&mut self, constraints: SystemConstraints) {
        self.system_constraints = constraints;
    }

    /// Save metrics to JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Generate default metrics filename with timestamp
    pub fn default_filename() -> PathBuf {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        PathBuf::from(format!("fluxphy_metrics_{}.json", timestamp))
    }

    /// Get a summary string for display
    pub fn summary(&self) -> String {
        format!(
            "Transfer Complete\n\
             ─────────────────\n\
             Size: {:.2} MB\n\
             Time: {:.2}s\n\
             Mean Rate: {:.2} MB/s\n\
             Flow Regime: {}\n\
             Thermal Stability: {:.2}\n\
             Prediction Accuracy: {}",
            self.file_size_bytes as f64 / (1024.0 * 1024.0),
            self.total_time_seconds,
            self.statistics.mean_rate,
            self.physics_metrics.flow_regime,
            self.physics_metrics.thermal_stability,
            self.prediction.accuracy_class
        )
    }

    /// Get analysis report for --analyze mode
    pub fn analysis_report(&self) -> String {
        let divider = "═".repeat(60);
        let thin_divider = "─".repeat(60);

        format!(
            r#"
{divider}
                    PHYSICS ANALYSIS REPORT
{divider}

Transfer Summary:
  Total Files: {}
  Total Size: {:.2} MB
  Duration: {:.2}s
  Mean Flux Rate: {:.2} MB/s

{thin_divider}
Statistical Analysis:
  μ (mean):     {:.2} MB/s
  σ (std dev):  {:.2} MB/s
  σ² (variance): {:.2} MB²/s²
  CV:           {:.4}

{thin_divider}
Physics Metrics:
  Flow Regime:        {}
  Thermal Stability:  {:.3}
  Flux Density:       {:.3}
  System Temperature: {:.2}
  Shannon Entropy:    {:.2} bits

{thin_divider}
System Constraints:
  Primary Bottleneck: {}
  CPU Usage:          {:.1}%
  Memory Pressure:    {:.1}%
  Efficiency:         {:.2}

{thin_divider}
Prediction Accuracy:
  Predicted Time: {:.2}s
  Actual Time:    {:.2}s
  Error:          {:.1}% ({})

{thin_divider}
Rate Distribution:
  Min: {:.2} MB/s
  Max: {:.2} MB/s
  Peak: {:.2} MB/s

{thin_divider}
Flow Analysis:
  {}

{divider}
"#,
            self.file_count,
            self.file_size_bytes as f64 / (1024.0 * 1024.0),
            self.total_time_seconds,
            self.statistics.mean_rate,
            self.statistics.mean_rate,
            self.statistics.std_dev,
            self.statistics.variance,
            self.statistics.coefficient_of_variation,
            self.physics_metrics.flow_regime,
            self.physics_metrics.thermal_stability,
            self.physics_metrics.flux_density,
            self.physics_metrics.system_temperature,
            self.physics_metrics.shannon_entropy,
            self.system_constraints.primary_bottleneck,
            self.system_constraints.cpu_usage,
            self.system_constraints.memory_pressure,
            self.system_constraints.efficiency,
            self.prediction.predicted_time,
            self.prediction.actual_time,
            self.prediction.error_percentage.abs(),
            self.prediction.accuracy_class,
            self.statistics.min_rate,
            self.statistics.max_rate,
            self.statistics.peak_rate,
            self.flow_analysis_text(),
            divider = divider,
            thin_divider = thin_divider,
        )
    }

    fn flow_analysis_text(&self) -> String {
        let regime_desc = match self.physics_metrics.flow_regime {
            FlowRegime::Laminar => {
                "The transfer exhibited highly stable laminar flow with minimal turbulence."
            }
            FlowRegime::Transitional => {
                "The transfer showed transitional flow with occasional fluctuations."
            }
            FlowRegime::Turbulent => {
                "The transfer experienced turbulent flow with significant rate variations."
            }
            FlowRegime::Chaotic => {
                "The transfer exhibited chaotic flow with highly unpredictable rate changes."
            }
        };

        let bottleneck_desc = match self.system_constraints.primary_bottleneck {
            Bottleneck::DiskRead => "disk read speed was the primary limiting factor",
            Bottleneck::DiskWrite => "disk write speed was the primary limiting factor",
            Bottleneck::CPU => "CPU was the primary limiting factor",
            Bottleneck::Memory => "memory pressure was the primary limiting factor",
            Bottleneck::Network => "network bandwidth was the primary limiting factor",
            Bottleneck::InsufficientData => "not enough data was collected to determine the bottleneck",
            Bottleneck::Unknown => "the system was well-balanced with no clear bottleneck",
        };

        format!(
            "{} The low coefficient of variation ({:.4}) indicates {}. \
             Bottleneck analysis suggests {}.",
            regime_desc,
            self.statistics.coefficient_of_variation,
            if self.statistics.coefficient_of_variation < 0.1 {
                "consistent I/O performance"
            } else {
                "variable I/O performance"
            },
            bottleneck_desc
        )
    }
}

impl Default for TransferMetrics {
    fn default() -> Self {
        Self {
            transfer_id: generate_transfer_id(),
            timestamp: Utc::now(),
            source: String::new(),
            destination: String::new(),
            file_size_bytes: 0,
            total_time_seconds: 0.0,
            file_count: 0,
            statistics: FluxStatistics::default(),
            physics_metrics: PhysicsMetrics::default(),
            system_constraints: SystemConstraints::default(),
            prediction: PredictionMetrics::default(),
            rate_history: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let rate_history = vec![
            (0.1, 100.0),
            (0.2, 105.0),
            (0.3, 98.0),
            (0.4, 102.0),
            (0.5, 101.0),
        ];

        let metrics = TransferMetrics::new(
            "/source/file.txt".to_string(),
            "/dest/file.txt".to_string(),
            1024 * 1024 * 100, // 100 MB
            1,
            1.0,
            rate_history,
            250.0,
        );

        assert!(!metrics.transfer_id.is_empty());
        assert!(metrics.statistics.mean_rate > 0.0);
    }

    #[test]
    fn test_metrics_serialization() {
        let metrics = TransferMetrics::default();
        let json = serde_json::to_string(&metrics).unwrap();
        assert!(json.contains("transfer_id"));
    }
}
