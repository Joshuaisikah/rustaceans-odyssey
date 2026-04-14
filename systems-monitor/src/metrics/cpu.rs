// ─── CpuMetrics ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CpuMetrics {
    /// Aggregate usage across all cores (0.0–100.0).
    pub usage_percent: f64,
    pub core_count:    usize,
}

impl CpuMetrics {
    pub fn new(usage_percent: f64, core_count: usize) -> Self {
        CpuMetrics { usage_percent, core_count }
    }

    /// True when usage is above `threshold` percent.
    pub fn is_above(&self, threshold: f64) -> bool {
        self.usage_percent > threshold
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_above_threshold() {
        let c = CpuMetrics::new(85.0, 4);
        assert!(c.is_above(80.0));
        assert!(!c.is_above(90.0));
    }

    #[test]
    fn test_fields_stored_correctly() {
        let c = CpuMetrics::new(42.5, 8);
        assert_eq!(c.usage_percent, 42.5);
        assert_eq!(c.core_count, 8);
    }
}
