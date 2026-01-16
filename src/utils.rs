//! Utility functions for FluxPhy

/// Format bytes into human-readable size string
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format duration in seconds to HH:MM:SS
pub fn format_duration(seconds: f64) -> String {
    let total_secs = seconds as u64;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

/// Format rate in MB/s or GB/s depending on magnitude
pub fn format_rate(mb_per_sec: f64) -> String {
    if mb_per_sec >= 1000.0 {
        format!("{:.2} GB/s", mb_per_sec / 1024.0)
    } else {
        format!("{:.2} MB/s", mb_per_sec)
    }
}

/// Generate a unique transfer ID based on timestamp
pub fn generate_transfer_id() -> String {
    let now = chrono::Local::now();
    let random_suffix: u32 = rand_suffix();
    format!("{}_{:06x}", now.format("%Y%m%d_%H%M%S"), random_suffix)
}

fn rand_suffix() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (duration.subsec_nanos() ^ duration.as_secs() as u32) & 0xFFFFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0.0), "00:00:00");
        assert_eq!(format_duration(65.0), "00:01:05");
        assert_eq!(format_duration(3661.0), "01:01:01");
    }

    #[test]
    fn test_format_rate() {
        assert_eq!(format_rate(50.0), "50.00 MB/s");
        assert_eq!(format_rate(1024.0), "1.00 GB/s");
    }
}
