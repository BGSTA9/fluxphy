//! Core file transfer engine for FluxPhy
//!
//! This module implements the instrumented file transfer logic with
//! rate sampling and progress tracking.

use crate::error::{FluxError, FluxResult};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use walkdir::WalkDir;

/// Progress update sent during transfer
#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    /// Total bytes copied so far
    pub bytes_copied: u64,
    /// Current instantaneous rate (MB/s)
    pub current_rate: f64,
    /// Elapsed time since start (seconds)
    pub elapsed_secs: f64,
    /// Current file being copied (for multi-file operations)
    pub current_file: Option<String>,
    /// File index in multi-file operations
    pub file_index: usize,
    /// Total file count
    pub total_files: usize,
    /// Mean transfer rate (MB/s)
    pub mean_rate: f64,
    /// Peak transfer rate (MB/s)
    pub peak_rate: f64,
}

/// Transfer state for external monitoring
#[derive(Debug, Clone)]
pub struct TransferState {
    /// Total size to transfer
    pub total_size: u64,
    /// Bytes transferred so far
    pub bytes_transferred: Arc<AtomicU64>,
    /// Is transfer paused
    pub paused: Arc<AtomicBool>,
    /// Is transfer cancelled
    pub cancelled: Arc<AtomicBool>,
    /// Rate history: (time, rate) pairs
    pub rate_history: Vec<(f64, f64)>,
    /// Start time
    pub start_time: Instant,
    /// Current file name
    pub current_file: String,
}

impl TransferState {
    pub fn new(total_size: u64) -> Self {
        Self {
            total_size,
            bytes_transferred: Arc::new(AtomicU64::new(0)),
            paused: Arc::new(AtomicBool::new(false)),
            cancelled: Arc::new(AtomicBool::new(false)),
            rate_history: Vec::with_capacity(1000),
            start_time: Instant::now(),
            current_file: String::new(),
        }
    }

    pub fn get_progress(&self) -> f64 {
        if self.total_size == 0 {
            return 100.0;
        }
        let transferred = self.bytes_transferred.load(Ordering::Relaxed);
        (transferred as f64 / self.total_size as f64) * 100.0
    }

    pub fn get_elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// FluxCopier performs instrumented file transfers
pub struct FluxCopier {
    /// Buffer size in bytes
    buffer_size: usize,
    /// Sample interval for rate collection
    sample_interval: Duration,
    /// Rate samples: (elapsed_time, rate_mb_s)
    rate_samples: Vec<(f64, f64)>,
    /// Total bytes copied across all files
    total_bytes_copied: u64,
    /// Start time of transfer
    start_time: Option<Instant>,
    /// Peak rate observed
    peak_rate: f64,
}

impl FluxCopier {
    /// Create a new FluxCopier with specified buffer size (in MB)
    pub fn new(buffer_size_mb: usize, sample_rate_ms: u64) -> Self {
        Self {
            buffer_size: buffer_size_mb * 1024 * 1024,
            sample_interval: Duration::from_millis(sample_rate_ms),
            rate_samples: Vec::with_capacity(1000),
            total_bytes_copied: 0,
            start_time: None,
            peak_rate: 0.0,
        }
    }

    /// Get collected rate samples
    pub fn get_rate_history(&self) -> &[(f64, f64)] {
        &self.rate_samples
    }

    /// Get rate samples as owned Vec for further processing
    pub fn take_rate_history(&mut self) -> Vec<(f64, f64)> {
        std::mem::take(&mut self.rate_samples)
    }

    /// Get just the rate values
    pub fn get_rates(&self) -> Vec<f64> {
        self.rate_samples.iter().map(|(_, r)| *r).collect()
    }

    /// Calculate total size of source files/directory
    pub fn calculate_total_size<P: AsRef<Path>>(sources: &[P], recursive: bool) -> FluxResult<u64> {
        let mut total = 0u64;

        for source in sources {
            let path = source.as_ref();
            if path.is_dir() {
                if recursive {
                    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                        if entry.file_type().is_file() {
                            total += entry.metadata().map(|m| m.len()).unwrap_or(0);
                        }
                    }
                } else {
                    return Err(FluxError::RecursiveRequired);
                }
            } else if path.is_file() {
                total += fs::metadata(path)?.len();
            } else {
                return Err(FluxError::SourceNotFound(path.display().to_string()));
            }
        }

        Ok(total)
    }

    /// Copy a single file with progress callback
    pub fn copy_file<P: AsRef<Path>>(
        &mut self,
        source: P,
        dest: P,
        progress_tx: Option<&mpsc::UnboundedSender<ProgressUpdate>>,
        paused: Option<Arc<AtomicBool>>,
    ) -> FluxResult<u64> {
        let source = source.as_ref();
        let dest = dest.as_ref();

        // Validate source
        if !source.exists() {
            return Err(FluxError::SourceNotFound(source.display().to_string()));
        }

        // Handle destination directory
        let final_dest = if dest.is_dir() {
            dest.join(source.file_name().unwrap_or_default())
        } else {
            dest.to_path_buf()
        };

        // Create parent directories if needed
        if let Some(parent) = final_dest.parent() {
            fs::create_dir_all(parent)?;
        }

        let file_size = fs::metadata(source)?.len();
        let src_file = File::open(source)?;
        let dst_file = File::create(&final_dest)?;

        let mut reader = BufReader::with_capacity(self.buffer_size, src_file);
        let mut writer = BufWriter::with_capacity(self.buffer_size, dst_file);

        let mut buffer = vec![0u8; self.buffer_size];
        let mut bytes_copied = 0u64;
        let mut last_sample_bytes = 0u64;

        let start = if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
            self.start_time.unwrap()
        } else {
            self.start_time.unwrap()
        };

        let mut last_sample_time = Instant::now();

        loop {
            // Check pause state and sleep while paused
            if let Some(ref pause_flag) = paused {
                while pause_flag.load(Ordering::Relaxed) {
                    std::thread::sleep(Duration::from_millis(100));
                }
            }

            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            writer.write_all(&buffer[..bytes_read])?;
            bytes_copied += bytes_read as u64;
            self.total_bytes_copied += bytes_read as u64;

            let now = Instant::now();
            if now.duration_since(last_sample_time) >= self.sample_interval {
                let elapsed = now.duration_since(start).as_secs_f64();
                let interval_secs = now.duration_since(last_sample_time).as_secs_f64();
                let bytes_this_interval = bytes_copied - last_sample_bytes;

                // Instantaneous rate for this sampling interval (MB/s)
                let rate = if interval_secs > 0.0 {
                    bytes_this_interval as f64 / interval_secs / (1024.0 * 1024.0)
                } else {
                    0.0
                };

                self.rate_samples.push((elapsed, rate));

                if rate > self.peak_rate {
                    self.peak_rate = rate;
                }

                let mean_rate = if elapsed > 0.0 {
                    self.total_bytes_copied as f64 / elapsed / (1024.0 * 1024.0)
                } else {
                    0.0
                };

                // Send progress update
                if let Some(tx) = progress_tx {
                    let _ = tx.send(ProgressUpdate {
                        bytes_copied: self.total_bytes_copied,
                        current_rate: rate,
                        elapsed_secs: elapsed,
                        current_file: Some(source.file_name().unwrap_or_default().to_string_lossy().to_string()),
                        file_index: 0,
                        total_files: 1,
                        mean_rate,
                        peak_rate: self.peak_rate,
                    });
                }

                last_sample_time = now;
                last_sample_bytes = bytes_copied;
            }
        }

        writer.flush()?;
        Ok(file_size)
    }

    /// Copy directory recursively
    pub fn copy_directory<P: AsRef<Path>>(
        &mut self,
        source: P,
        dest: P,
        progress_tx: Option<&mpsc::UnboundedSender<ProgressUpdate>>,
        paused: Option<Arc<AtomicBool>>,
    ) -> FluxResult<(usize, u64)> {
        let source = source.as_ref();
        let dest = dest.as_ref();

        if !source.is_dir() {
            return Err(FluxError::InvalidPath(
                "Source must be a directory".to_string(),
            ));
        }

        // Collect all files first
        let files: Vec<PathBuf> = WalkDir::new(source)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .collect();

        let total_files = files.len();
        let mut total_bytes = 0u64;

        for (idx, file_path) in files.iter().enumerate() {
            // Calculate relative path
            let relative = file_path.strip_prefix(source).unwrap_or(file_path);
            let dest_path = dest.join(relative);

            // Update progress with file info
            if let Some(tx) = progress_tx {
                // Approximate mean/peak for directory intermediate updates
                let elapsed = self.start_time.map(|s| s.elapsed().as_secs_f64()).unwrap_or(0.0);
                let current_rate = self.rate_samples.last().map(|(_, r)| *r).unwrap_or(0.0);
                let mean_rate = if elapsed > 0.0 {
                     self.total_bytes_copied as f64 / elapsed / (1024.0 * 1024.0)
                } else {
                    0.0
                };
                
                let _ = tx.send(ProgressUpdate {
                    bytes_copied: self.total_bytes_copied,
                    current_rate,
                    elapsed_secs: elapsed,
                    current_file: Some(file_path.file_name().unwrap_or_default().to_string_lossy().to_string()),
                    file_index: idx,
                    total_files,
                    mean_rate,
                    peak_rate: self.peak_rate,
                });
            }

            let bytes = self.copy_file(file_path, &dest_path, progress_tx, paused.clone())?;
            total_bytes += bytes;
        }

        Ok((total_files, total_bytes))
    }

    /// Compute SHA-256 checksum of a file
    pub fn compute_checksum<P: AsRef<Path>>(path: P) -> FluxResult<String> {
        let file = File::open(path.as_ref())?;
        let mut reader = BufReader::with_capacity(8 * 1024 * 1024, file);
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8 * 1024 * 1024];

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Verify source and destination have matching checksums
    pub fn verify_copy<P: AsRef<Path>>(source: P, dest: P) -> FluxResult<bool> {
        let source_hash = Self::compute_checksum(&source)?;
        let dest_hash = Self::compute_checksum(&dest)?;

        if source_hash == dest_hash {
            Ok(true)
        } else {
            Err(FluxError::ChecksumMismatch {
                expected: source_hash,
                actual: dest_hash,
            })
        }
    }

    /// Get the elapsed time since transfer started
    pub fn get_elapsed(&self) -> Duration {
        self.start_time
            .map(|s| s.elapsed())
            .unwrap_or(Duration::ZERO)
    }

    /// Get total bytes copied
    pub fn get_total_bytes_copied(&self) -> u64 {
        self.total_bytes_copied
    }

    /// Calculate initial rate estimate from first N samples
    pub fn get_initial_rate_estimate(&self, num_samples: usize) -> f64 {
        if self.rate_samples.is_empty() {
            return 0.0;
        }
        let samples_to_use = self.rate_samples.len().min(num_samples);
        let sum: f64 = self.rate_samples[..samples_to_use].iter().map(|(_, r)| r).sum();
        sum / samples_to_use as f64
    }
}

impl Default for FluxCopier {
    fn default() -> Self {
        Self::new(8, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_copy_file() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source.txt");
        let dest = temp.path().join("dest.txt");

        fs::write(&source, b"Hello, FluxPhy!").unwrap();

        let mut copier = FluxCopier::default();
        copier.copy_file(&source, &dest, None, None).unwrap();

        assert_eq!(fs::read(&source).unwrap(), fs::read(&dest).unwrap());
    }

    #[test]
    fn test_checksum() {
        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.txt");
        fs::write(&file, b"Test content").unwrap();

        let hash = FluxCopier::compute_checksum(&file).unwrap();
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA-256 = 64 hex chars
    }

    #[test]
    fn test_verify_copy() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source.txt");
        let dest = temp.path().join("dest.txt");

        fs::write(&source, b"Same content").unwrap();
        fs::write(&dest, b"Same content").unwrap();

        assert!(FluxCopier::verify_copy(&source, &dest).unwrap());
    }
}
