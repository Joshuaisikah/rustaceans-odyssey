use std::collections::VecDeque;
use crate::alert::{Alert, AlertThreshold};
use crate::metrics::Snapshot;

// ─── MetricsHistory ──────────────────────────────────────────────────────────
//
// Rolling window of metric snapshots with threshold-based alert detection.
//
// Design hints:
//   - Use a VecDeque<Snapshot> as the rolling buffer.
//   - record(): push_back the snapshot; if len > max_snapshots, pop_front.
//   - check_alerts(): inspect the latest snapshot against the threshold.
//   - average_cpu_usage(): mean of all stored cpu.usage_percent values.

pub struct MetricsHistory {
    // TODO: VecDeque<Snapshot>, max_snapshots, AlertThreshold
}

impl MetricsHistory {
    /// Create a history that stores at most `max_snapshots` entries and fires
    /// alerts when the latest snapshot exceeds `threshold`.
    pub fn new(max_snapshots: usize, threshold: AlertThreshold) -> Self {
        todo!()
    }

    /// Add a snapshot.  If the window is full the oldest entry is discarded.
    pub fn record(&mut self, snapshot: Snapshot) {
        todo!()
    }

    /// Most recent snapshot, or None if empty.
    pub fn latest(&self) -> Option<&Snapshot> {
        todo!()
    }

    /// Alerts triggered by the most recent snapshot.
    pub fn check_alerts(&self) -> Vec<Alert> {
        todo!()
    }

    /// Mean CPU usage across all stored snapshots (0.0 if empty).
    pub fn average_cpu_usage(&self) -> f64 {
        todo!()
    }

    /// Peak CPU usage seen across all stored snapshots (0.0 if empty).
    pub fn peak_cpu_usage(&self) -> f64 {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alert::AlertKind;
    use crate::metrics::{CpuMetrics, MemoryMetrics, Snapshot};

    fn snap(cpu_pct: f64, mem_used: u64, mem_total: u64) -> Snapshot {
        Snapshot::new(
            CpuMetrics { usage_percent: cpu_pct, core_count: 4 },
            MemoryMetrics { total_bytes: mem_total, used_bytes: mem_used, free_bytes: mem_total - mem_used },
            vec![],
        )
    }

    fn threshold(cpu: f64, mem: f64) -> AlertThreshold {
        AlertThreshold::new(cpu, mem)
    }

    // latest() on an empty history returns None.
    #[test]
    fn test_latest_empty_is_none() {
        let h = MetricsHistory::new(10, threshold(80.0, 80.0));
        assert!(h.latest().is_none());
        assert!(h.is_empty());
    }

    // record() and latest() work together.
    #[test]
    fn test_record_and_latest() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        h.record(snap(50.0, 500, 1000));
        assert_eq!(h.len(), 1);
        assert!(h.latest().is_some());
    }

    // The window is bounded at max_snapshots; oldest entry is dropped.
    #[test]
    fn test_history_is_bounded() {
        let mut h = MetricsHistory::new(3, threshold(80.0, 80.0));
        h.record(snap(10.0, 100, 1000));
        h.record(snap(20.0, 200, 1000));
        h.record(snap(30.0, 300, 1000));
        h.record(snap(40.0, 400, 1000)); // oldest (10%) dropped
        assert_eq!(h.len(), 3);
    }

    // No alerts when below thresholds.
    #[test]
    fn test_no_alert_below_threshold() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        h.record(snap(50.0, 500, 1000));
        assert!(h.check_alerts().is_empty());
    }

    // HighCpu alert fires when CPU exceeds threshold.
    #[test]
    fn test_high_cpu_alert_fires() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 90.0));
        h.record(snap(95.0, 500, 1000));
        let alerts = h.check_alerts();
        assert!(alerts.iter().any(|a| matches!(a.kind, AlertKind::HighCpu { .. })));
    }

    // HighMemory alert fires when memory exceeds threshold.
    #[test]
    fn test_high_memory_alert_fires() {
        let mut h = MetricsHistory::new(10, threshold(90.0, 80.0));
        h.record(snap(30.0, 900, 1000)); // 90% memory
        let alerts = h.check_alerts();
        assert!(alerts.iter().any(|a| matches!(a.kind, AlertKind::HighMemory { .. })));
    }

    // Both alerts fire simultaneously.
    #[test]
    fn test_both_alerts_fire_together() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        h.record(snap(90.0, 900, 1000));
        assert_eq!(h.check_alerts().len(), 2);
    }

    // average_cpu_usage is the mean of all snapshots.
    #[test]
    fn test_average_cpu_usage() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        h.record(snap(10.0, 0, 1000));
        h.record(snap(20.0, 0, 1000));
        h.record(snap(30.0, 0, 1000));
        assert!((h.average_cpu_usage() - 20.0).abs() < 0.001);
    }

    // average_cpu_usage on empty history is 0.0.
    #[test]
    fn test_average_cpu_empty_is_zero() {
        let h = MetricsHistory::new(10, threshold(80.0, 80.0));
        assert_eq!(h.average_cpu_usage(), 0.0);
    }

    // peak_cpu_usage is the maximum seen.
    #[test]
    fn test_peak_cpu_usage() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        h.record(snap(40.0, 0, 1000));
        h.record(snap(90.0, 0, 1000));
        h.record(snap(55.0, 0, 1000));
        assert!((h.peak_cpu_usage() - 90.0).abs() < 0.001);
    }

    // is_empty/len reflect recorded snapshots.
    #[test]
    fn test_is_empty_and_len() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        assert!(h.is_empty());
        h.record(snap(10.0, 0, 1000));
        assert!(!h.is_empty());
        assert_eq!(h.len(), 1);
    }
}
