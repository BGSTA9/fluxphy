//! Advanced Performance Modelling for FluxPhy
//!
//! Implements time-series analysis tools including rolling windows,
//! trend detection (linear regression), and statistical outlier detection.

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TimeSeriesModel {
    /// Number of samples to keep in history
    window_size: usize,
    /// History of rate samples (bytes/sec)
    history: VecDeque<f64>,
}

pub struct AnalysisResult {
    /// Trend slope (bytes/sec^2) - negative means slowing down
    pub trend_slope: f64,
    /// Is the current transfer accelerating, stable, or decelerating?
    pub trend_status: TrendStatus,
    /// Is the latest sample an outlier (> 2*sigma)?
    pub is_outlier: bool,
    /// Predicted rate for next second (simple extrapolation)
    pub predicted_next_rate: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrendStatus {
    Accelerating,
    Stable,
    Decelerating,
}

impl TimeSeriesModel {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            history: VecDeque::with_capacity(window_size),
        }
    }

    pub fn add_sample(&mut self, rate: f64) {
        if self.history.len() >= self.window_size {
            self.history.pop_front();
        }
        self.history.push_back(rate);
    }

    pub fn analyze(&self) -> Option<AnalysisResult> {
        if self.history.len() < 3 {
            return None;
        }

        let n = self.history.len() as f64;
        let rates: Vec<f64> = self.history.iter().cloned().collect();

        // 1. Calculate Mean and StdDev
        let mean = rates.iter().sum::<f64>() / n;
        let variance = rates.iter().map(|&r| (r - mean).powi(2)).sum::<f64>() / n;
        let std_dev = variance.sqrt();

        // 2. Linear Regression (Simple Ordinary Least Squares)
        // X = time (0, 1, 2...), Y = rate
        let sum_x: f64 = (0..self.history.len()).map(|i| i as f64).sum();
        let sum_y: f64 = rates.iter().sum();
        let sum_xy: f64 = rates.iter().enumerate().map(|(i, &r)| i as f64 * r).sum();
        let sum_xx: f64 = (0..self.history.len()).map(|i| (i as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x.powi(2));
        
        // 3. Trend Classification
        // If slope is significant relative to mean (e.g. > 1% change per sample)
        let relative_slope = slope / mean;
        let trend_status = if relative_slope > 0.01 {
            TrendStatus::Accelerating
        } else if relative_slope < -0.01 {
            TrendStatus::Decelerating
        } else {
            TrendStatus::Stable
        };

        // 4. Outlier Detection
        let latest = *self.history.back().unwrap();
        let is_outlier = (latest - mean).abs() > (2.0 * std_dev);

        // 5. Prediction (next sample at T=n)
        let intercept = (sum_y - slope * sum_x) / n;
        let predicted_next = slope * n + intercept;

        Some(AnalysisResult {
            trend_slope: slope,
            trend_status,
            is_outlier,
            predicted_next_rate: predicted_next.max(0.0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_acceleration() {
        let mut model = TimeSeriesModel::new(10);
        // Add increasing values: 100, 200, 300...
        for i in 1..=5 {
            model.add_sample(i as f64 * 100.0);
        }
        let result = model.analyze().unwrap();
        assert_eq!(result.trend_status, TrendStatus::Accelerating);
        assert!(result.trend_slope > 0.0);
    }

    #[test]
    fn test_outlier_detection() {
        let mut model = TimeSeriesModel::new(10);
        // Stable values
        for _ in 0..10 {
            model.add_sample(100.0);
        }
        // Sudden spike
        model.add_sample(1000.0);
        
        let result = model.analyze().unwrap();
        assert!(result.is_outlier);
    }
}
