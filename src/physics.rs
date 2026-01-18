//! Physics metrics calculation for FluxPhy
//! 
//! This module implements physics-inspired analysis of file transfer rates,
//! treating data flux as a physical process with measurable properties.

use serde::{Deserialize, Serialize};

/// Flow regime classification based on coefficient of variation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FlowRegime {
    /// CV < 0.05 - smooth, predictable transfer
    Laminar,
    /// 0.05 ≤ CV < 0.15 - mostly stable with minor fluctuations
    Transitional,
    /// 0.15 ≤ CV < 0.30 - significant fluctuations
    Turbulent,
    /// CV ≥ 0.30 - highly unpredictable
    Chaotic,
}

impl FlowRegime {
    /// Classify flow regime from coefficient of variation
    pub fn from_cv(cv: f64) -> Self {
        if cv < 0.05 {
            FlowRegime::Laminar
        } else if cv < 0.15 {
            FlowRegime::Transitional
        } else if cv < 0.30 {
            FlowRegime::Turbulent
        } else {
            FlowRegime::Chaotic
        }
    }

    /// Get a human-readable description of the flow regime
    pub fn description(&self) -> &'static str {
        match self {
            FlowRegime::Laminar => "Smooth, predictable transfer",
            FlowRegime::Transitional => "Mostly stable with minor fluctuations",
            FlowRegime::Turbulent => "Significant rate fluctuations",
            FlowRegime::Chaotic => "Highly unpredictable transfer",
        }
    }
}

impl std::fmt::Display for FlowRegime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowRegime::Laminar => write!(f, "Laminar"),
            FlowRegime::Transitional => write!(f, "Transitional"),
            FlowRegime::Turbulent => write!(f, "Turbulent"),
            FlowRegime::Chaotic => write!(f, "Chaotic"),
        }
    }
}

/// System bottleneck classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Bottleneck {
    DiskRead,
    DiskWrite,
    CPU,
    Memory,
    Network,
    /// Not enough data collected yet
    InsufficientData,
    /// Could not determine after analysis
    Unknown,
}

impl Bottleneck {
    /// Get a user-friendly description of what this bottleneck means
    pub fn describe(&self) -> &'static str {
        match self {
            Bottleneck::DiskRead => "Your disk is slow at reading files. Think of it like a slow librarian finding books.",
            Bottleneck::DiskWrite => "Your disk is slow at writing files. Like a printer that can't keep up.",
            Bottleneck::CPU => "Your processor is working too hard. The brain of your computer needs a break!",
            Bottleneck::Memory => "Your computer is running low on working memory. Too many things open at once.",
            Bottleneck::Network => "The network connection is the slowest part. Like a narrow road with traffic.",
            Bottleneck::InsufficientData => "Not enough measurements yet to determine the bottleneck.",
            Bottleneck::Unknown => "The system is well-balanced; no clear bottleneck detected.",
        }
    }
}

impl std::fmt::Display for Bottleneck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bottleneck::DiskRead => write!(f, "Disk Read"),
            Bottleneck::DiskWrite => write!(f, "Disk Write"),
            Bottleneck::CPU => write!(f, "CPU"),
            Bottleneck::Memory => write!(f, "Memory"),
            Bottleneck::Network => write!(f, "Network"),
            Bottleneck::InsufficientData => write!(f, "Analyzing..."),
            Bottleneck::Unknown => write!(f, "Balanced"),
        }
    }
}

/// Prediction accuracy classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Accuracy {
    /// < 5% error
    Excellent,
    /// 5-15% error
    Good,
    /// 15-30% error
    Fair,
    /// > 30% error
    Poor,
}

impl Accuracy {
    pub fn from_error_percentage(error: f64) -> Self {
        let error_abs = error.abs();
        if error_abs < 5.0 {
            Accuracy::Excellent
        } else if error_abs < 15.0 {
            Accuracy::Good
        } else if error_abs < 30.0 {
            Accuracy::Fair
        } else {
            Accuracy::Poor
        }
    }
}

impl std::fmt::Display for Accuracy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accuracy::Excellent => write!(f, "Excellent"),
            Accuracy::Good => write!(f, "Good"),
            Accuracy::Fair => write!(f, "Fair"),
            Accuracy::Poor => write!(f, "Poor"),
        }
    }
}

/// Basic statistical analysis of flux rates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluxStatistics {
    /// Mean transfer rate (MB/s): μ = Σ R(t) / n
    pub mean_rate: f64,
    /// Variance (MB²/s²): σ² = Σ(R(t) - μ)² / n
    pub variance: f64,
    /// Standard deviation (MB/s): σ
    pub std_dev: f64,
    /// Coefficient of variation: CV = σ / μ
    pub coefficient_of_variation: f64,
    /// Minimum observed rate
    pub min_rate: f64,
    /// Maximum observed rate
    pub max_rate: f64,
    /// Peak rate (same as max, but for clarity)
    pub peak_rate: f64,
    /// Number of samples collected
    pub sample_count: usize,
}

impl FluxStatistics {
    /// Calculate statistics from rate samples
    pub fn from_samples(samples: &[f64]) -> Self {
        if samples.is_empty() {
            return Self::default();
        }

        let n = samples.len() as f64;
        let sum: f64 = samples.iter().sum();
        let mean = sum / n;

        let variance = if samples.len() > 1 {
            samples.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / n
        } else {
            0.0
        };

        let std_dev = variance.sqrt();
        let cv = if mean > 0.0 { std_dev / mean } else { 0.0 };

        let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        Self {
            mean_rate: mean,
            variance,
            std_dev,
            coefficient_of_variation: cv,
            min_rate: min,
            max_rate: max,
            peak_rate: max,
            sample_count: samples.len(),
        }
    }
}

impl Default for FluxStatistics {
    fn default() -> Self {
        Self {
            mean_rate: 0.0,
            variance: 0.0,
            std_dev: 0.0,
            coefficient_of_variation: 0.0,
            min_rate: 0.0,
            max_rate: 0.0,
            peak_rate: 0.0,
            sample_count: 0,
        }
    }
}

/// Physics-inspired metrics derived from transfer behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsMetrics {
    /// Flux density: ρ = R / R_max (0.0 to 1.0)
    pub flux_density: f64,
    /// Thermal stability: S = 1 - CV (0.0 to 1.0)
    pub thermal_stability: f64,
    /// System "temperature": T ∝ σ² (higher = more chaotic)
    pub system_temperature: f64,
    /// Shannon entropy of rate distribution (bits)
    pub shannon_entropy: f64,
    /// Flow regime classification
    pub flow_regime: FlowRegime,
}

impl PhysicsMetrics {
    /// Calculate physics metrics from statistics and theoretical maximum
    pub fn calculate(stats: &FluxStatistics, theoretical_max: f64) -> Self {
        let flux_density = if theoretical_max > 0.0 {
            (stats.mean_rate / theoretical_max).min(1.0)
        } else {
            0.0
        };

        let thermal_stability = (1.0 - stats.coefficient_of_variation).max(0.0).min(1.0);
        let system_temperature = stats.variance;
        let flow_regime = FlowRegime::from_cv(stats.coefficient_of_variation);

        Self {
            flux_density,
            thermal_stability,
            system_temperature,
            shannon_entropy: 0.0, // Calculated separately
            flow_regime,
        }
    }

    /// Calculate Shannon entropy from rate samples
    pub fn calculate_entropy(samples: &[f64]) -> f64 {
        if samples.len() < 2 {
            return 0.0;
        }

        // Bin the rates into histogram buckets
        let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = max - min;

        if range == 0.0 {
            return 0.0; // All values identical = zero entropy
        }

        const NUM_BINS: usize = 16;
        let mut bins = [0usize; NUM_BINS];
        let bin_width = range / NUM_BINS as f64;

        for &sample in samples {
            let bin_idx = ((sample - min) / bin_width).floor() as usize;
            let bin_idx = bin_idx.min(NUM_BINS - 1);
            bins[bin_idx] += 1;
        }

        // Calculate Shannon entropy: H = -Σ p(r) log₂ p(r)
        let n = samples.len() as f64;
        let mut entropy = 0.0;

        for &count in &bins {
            if count > 0 {
                let p = count as f64 / n;
                entropy -= p * p.log2();
            }
        }

        entropy
    }
}

impl Default for PhysicsMetrics {
    fn default() -> Self {
        Self {
            flux_density: 0.0,
            thermal_stability: 1.0,
            system_temperature: 0.0,
            shannon_entropy: 0.0,
            flow_regime: FlowRegime::Laminar,
        }
    }
}

/// System constraint analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConstraints {
    /// Identified primary bottleneck
    pub primary_bottleneck: Bottleneck,
    /// CPU usage percentage (0-100)
    pub cpu_usage: f32,
    /// Disk I/O wait percentage
    pub disk_io_wait: f32,
    /// Memory pressure percentage
    pub memory_pressure: f32,
    /// Overall efficiency (0.0 to 1.0)
    pub efficiency: f64,
}

impl SystemConstraints {
    /// Analyze system constraints to identify bottleneck
    pub fn analyze(
        cpu_usage: f32,
        disk_read_rate: f64,
        disk_write_rate: f64,
        memory_usage: f32,
        actual_rate: f64,
        theoretical_max: f64,
    ) -> Self {
        // Determine primary bottleneck based on resource usage
        let primary_bottleneck = if cpu_usage > 90.0 {
            Bottleneck::CPU
        } else if memory_usage > 90.0 {
            Bottleneck::Memory
        } else if disk_write_rate > disk_read_rate * 0.8 {
            Bottleneck::DiskWrite
        } else if disk_read_rate > 0.0 {
            Bottleneck::DiskRead
        } else {
            Bottleneck::Unknown
        };

        let efficiency = if theoretical_max > 0.0 {
            (actual_rate / theoretical_max).min(1.0)
        } else {
            0.0
        };

        Self {
            primary_bottleneck,
            cpu_usage,
            disk_io_wait: 0.0, // Would need OS-specific implementation
            memory_pressure: memory_usage,
            efficiency,
        }
    }
}

impl Default for SystemConstraints {
    fn default() -> Self {
        Self {
            primary_bottleneck: Bottleneck::Unknown,
            cpu_usage: 0.0,
            disk_io_wait: 0.0,
            memory_pressure: 0.0,
            efficiency: 0.0,
        }
    }
}

/// Prediction accuracy metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionMetrics {
    /// Initial rate estimate from first ~0.5-1s (MB/s)
    pub initial_rate_estimate: f64,
    /// Predicted total time (seconds)
    pub predicted_time: f64,
    /// Actual total time (seconds)
    pub actual_time: f64,
    /// Percentage error
    pub error_percentage: f64,
    /// Accuracy classification
    pub accuracy_class: Accuracy,
}

impl PredictionMetrics {
    /// Calculate prediction accuracy
    pub fn calculate(initial_rate: f64, file_size_bytes: u64, actual_time: f64) -> Self {
        let predicted_time = if initial_rate > 0.0 {
            (file_size_bytes as f64 / (1024.0 * 1024.0)) / initial_rate
        } else {
            0.0
        };

        let error_percentage = if actual_time > 0.0 {
            ((predicted_time - actual_time) / actual_time) * 100.0
        } else {
            0.0
        };

        Self {
            initial_rate_estimate: initial_rate,
            predicted_time,
            actual_time,
            error_percentage,
            accuracy_class: Accuracy::from_error_percentage(error_percentage),
        }
    }
}

impl Default for PredictionMetrics {
    fn default() -> Self {
        Self {
            initial_rate_estimate: 0.0,
            predicted_time: 0.0,
            actual_time: 0.0,
            error_percentage: 0.0,
            accuracy_class: Accuracy::Excellent,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_regime_classification() {
        assert_eq!(FlowRegime::from_cv(0.03), FlowRegime::Laminar);
        assert_eq!(FlowRegime::from_cv(0.10), FlowRegime::Transitional);
        assert_eq!(FlowRegime::from_cv(0.20), FlowRegime::Turbulent);
        assert_eq!(FlowRegime::from_cv(0.35), FlowRegime::Chaotic);
    }

    #[test]
    fn test_flux_statistics() {
        let samples = vec![100.0, 105.0, 98.0, 102.0, 101.0];
        let stats = FluxStatistics::from_samples(&samples);

        assert!((stats.mean_rate - 101.2).abs() < 0.1);
        assert!(stats.std_dev > 0.0);
        assert_eq!(stats.sample_count, 5);
    }

    #[test]
    fn test_entropy_calculation() {
        // All same values = zero entropy
        let uniform = vec![100.0, 100.0, 100.0, 100.0];
        assert_eq!(PhysicsMetrics::calculate_entropy(&uniform), 0.0);

        // Varied values = positive entropy
        let varied = vec![50.0, 100.0, 150.0, 200.0];
        assert!(PhysicsMetrics::calculate_entropy(&varied) > 0.0);
    }

    #[test]
    fn test_accuracy_classification() {
        assert_eq!(Accuracy::from_error_percentage(2.0), Accuracy::Excellent);
        assert_eq!(Accuracy::from_error_percentage(10.0), Accuracy::Good);
        assert_eq!(Accuracy::from_error_percentage(20.0), Accuracy::Fair);
        assert_eq!(Accuracy::from_error_percentage(40.0), Accuracy::Poor);
    }
}
