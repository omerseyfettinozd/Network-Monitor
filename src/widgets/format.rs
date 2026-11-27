use crate::config::SpeedUnit;

/// Formats a byte rate (bytes/sec) into a human-readable speed string.
pub fn format_speed(bytes_per_sec: f64, unit: SpeedUnit) -> String {
    let safe = if bytes_per_sec.is_finite() && bytes_per_sec > 0.0 {
        bytes_per_sec
    } else {
        0.0
    };

    match unit {
        SpeedUnit::BytesPerSec => {
            const UNITS: [&str; 4] = ["B/s", "KB/s", "MB/s", "GB/s"];
            let mut value = safe;
            let mut idx = 0;
            while value >= 1024.0 && idx < UNITS.len() - 1 {
                value /= 1024.0;
                idx += 1;
            }
            if idx == 0 {
                format!("{value:.0} {}", UNITS[idx])
            } else if value >= 100.0 {
                format!("{value:.1} {}", UNITS[idx])
            } else {
                format!("{value:.2} {}", UNITS[idx])
            }
        }
        SpeedUnit::BitsPerSec => {
            const UNITS: [&str; 4] = ["bps", "Kbps", "Mbps", "Gbps"];
            let mut value = safe * 8.0;
            let mut idx = 0;
            while value >= 1000.0 && idx < UNITS.len() - 1 {
                value /= 1000.0;
                idx += 1;
            }
            if idx == 0 {
                format!("{value:.0} {}", UNITS[idx])
            } else if value >= 100.0 {
                format!("{value:.1} {}", UNITS[idx])
            } else {
                format!("{value:.2} {}", UNITS[idx])
            }
        }
    }
}

/// Formats total transferred bytes into a compact readable string.
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut idx = 0;
    while value >= 1024.0 && idx < UNITS.len() - 1 {
        value /= 1024.0;
        idx += 1;
    }
    if idx == 0 {
        format!("{bytes} B")
    } else if value >= 100.0 {
        format!("{value:.1} {}", UNITS[idx])
    } else {
        format!("{value:.2} {}", UNITS[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_byte_speeds() {
        assert_eq!(format_speed(0.0, SpeedUnit::BytesPerSec), "0 B/s");
        assert_eq!(format_speed(512.0, SpeedUnit::BytesPerSec), "512 B/s");
        assert_eq!(format_speed(1536.0, SpeedUnit::BytesPerSec), "1.50 KB/s");
        assert_eq!(
            format_speed(12.5 * 1024.0 * 1024.0, SpeedUnit::BytesPerSec),
            "12.50 MB/s"
        );
    }

    #[test]
    fn formats_bit_speeds() {
        assert_eq!(format_speed(125_000.0, SpeedUnit::BitsPerSec), "1.00 Mbps");
        assert_eq!(
            format_speed(125_000_000.0, SpeedUnit::BitsPerSec),
            "1.00 Gbps"
        );
    }

    #[test]
    fn formats_totals() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(5 * 1024 * 1024), "5.00 MB");
    }
}
