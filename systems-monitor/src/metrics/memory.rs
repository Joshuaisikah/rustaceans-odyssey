// ─── MemoryMetrics ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub used_bytes:  u64,
    pub free_bytes:  u64,
}

impl MemoryMetrics {
    pub fn new(total_bytes: u64, used_bytes: u64) -> Self {
        MemoryMetrics {
            total_bytes,
            used_bytes,
            free_bytes: total_bytes.saturating_sub(used_bytes),
        }
    }

    /// Percentage of total memory in use (0.0–100.0).
    /// Returns 0.0 when total_bytes == 0.
    pub fn usage_percent(&self) -> f64 {
        todo!()
    }

    pub fn is_above(&self, threshold_pct: f64) -> bool {
        self.usage_percent() > threshold_pct
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_percent_25() {
        let m = MemoryMetrics::new(1_000_000, 250_000);
        assert!((m.usage_percent() - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_usage_percent_zero_total_returns_zero() {
        let m = MemoryMetrics { total_bytes: 0, used_bytes: 0, free_bytes: 0 };
        assert_eq!(m.usage_percent(), 0.0);
    }

    #[test]
    fn test_usage_percent_full() {
        let m = MemoryMetrics::new(1000, 1000);
        assert!((m.usage_percent() - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_free_bytes_computed_correctly() {
        let m = MemoryMetrics::new(1000, 300);
        assert_eq!(m.free_bytes, 700);
    }

    #[test]
    fn test_is_above() {
        let m = MemoryMetrics::new(1000, 900); // 90%
        assert!(m.is_above(80.0));
        assert!(!m.is_above(95.0));
    }
}
