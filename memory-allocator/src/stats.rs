// ─── AllocStats ───────────────────────────────────────────────────────────────
//
// Tracks allocation counters.  Embed this in your allocator to observe usage.

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AllocStats {
    pub total_allocations:   usize,
    pub total_deallocations: usize,
    pub bytes_allocated:     usize,
    pub bytes_freed:         usize,
    pub peak_usage:          usize,
    pub current_usage:       usize,
}

impl AllocStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an allocation of `size` bytes.
    pub fn record_alloc(&mut self, size: usize) {
        todo!()
        // Hint:
        //   total_allocations += 1
        //   bytes_allocated   += size
        //   current_usage     += size
        //   peak_usage = peak_usage.max(current_usage)
    }

    /// Record a deallocation of `size` bytes.
    pub fn record_free(&mut self, size: usize) {
        todo!()
        // Hint:
        //   total_deallocations += 1
        //   bytes_freed         += size
        //   current_usage        = current_usage.saturating_sub(size)
    }

    /// True when current_usage == 0 (no live allocations).
    pub fn is_balanced(&self) -> bool {
        self.current_usage == 0
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Fresh stats are all zero.
    #[test]
    fn test_new_stats_are_zero() {
        let s = AllocStats::new();
        assert_eq!(s, AllocStats::default());
    }

    // record_alloc increments counters and tracks peak.
    #[test]
    fn test_record_alloc_increments_counters() {
        let mut s = AllocStats::new();
        s.record_alloc(100);
        assert_eq!(s.total_allocations, 1);
        assert_eq!(s.bytes_allocated,   100);
        assert_eq!(s.current_usage,     100);
        assert_eq!(s.peak_usage,        100);
    }

    // record_free decrements current_usage.
    #[test]
    fn test_record_free_decrements_current_usage() {
        let mut s = AllocStats::new();
        s.record_alloc(200);
        s.record_free(100);
        assert_eq!(s.total_deallocations, 1);
        assert_eq!(s.bytes_freed,         100);
        assert_eq!(s.current_usage,       100);
    }

    // peak_usage is the maximum current_usage ever seen.
    #[test]
    fn test_peak_usage_is_high_water_mark() {
        let mut s = AllocStats::new();
        s.record_alloc(500);
        s.record_free(300);
        s.record_alloc(100); // current = 300, peak stays 500
        assert_eq!(s.peak_usage, 500);
    }

    // current_usage does not underflow (saturating sub).
    #[test]
    fn test_current_usage_saturates_at_zero() {
        let mut s = AllocStats::new();
        s.record_alloc(10);
        s.record_free(50); // free more than allocated
        assert_eq!(s.current_usage, 0);
    }

    // is_balanced is true only when current_usage == 0.
    #[test]
    fn test_is_balanced() {
        let mut s = AllocStats::new();
        assert!(s.is_balanced());
        s.record_alloc(64);
        assert!(!s.is_balanced());
        s.record_free(64);
        assert!(s.is_balanced());
    }

    // Multiple alloc/free pairs are tracked independently.
    #[test]
    fn test_multiple_alloc_free_pairs() {
        let mut s = AllocStats::new();
        s.record_alloc(10);
        s.record_alloc(20);
        s.record_free(10);
        assert_eq!(s.total_allocations,   2);
        assert_eq!(s.total_deallocations, 1);
        assert_eq!(s.current_usage,       20);
    }
}
