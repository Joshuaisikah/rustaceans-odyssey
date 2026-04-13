use std::collections::VecDeque;
use std::time::{Duration, Instant};

// ─── Metric types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CpuMetrics {
    pub usage_percent: f64, // 0.0 – 100.0
    pub core_count: usize,
}

#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
}

impl MemoryMetrics {
    /// Percentage of total memory that is in use (0.0–100.0).
    pub fn usage_percent(&self) -> f64 {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessMetrics {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f64,
    pub memory_bytes: u64,
    pub threads: u32,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub timestamp: Instant,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub processes: Vec<ProcessMetrics>,
}

// ─── Alerting ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AlertThreshold {
    pub cpu_percent: f64,
    pub memory_percent: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertKind {
    HighCpu { actual: u64 },      // rounded to nearest integer for equality
    HighMemory { actual: u64 },
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub kind: AlertKind,
    pub message: String,
}

// ─── History ─────────────────────────────────────────────────────────────────

/// Rolling window of metric snapshots with alert detection.
pub struct MetricsHistory {
    // TODO: add your fields
}

impl MetricsHistory {
    /// Create a history that stores at most `max_snapshots` entries and fires
    /// alerts when the latest snapshot exceeds `threshold`.
    pub fn new(max_snapshots: usize, threshold: AlertThreshold) -> Self {
        todo!()
    }

    /// Add a snapshot. If the window is full the oldest entry is discarded.
    pub fn record(&mut self, snapshot: Snapshot) {
        todo!()
    }

    /// Return a reference to the most recent snapshot, or `None` if empty.
    pub fn latest(&self) -> Option<&Snapshot> {
        todo!()
    }

    /// Return any alerts triggered by the most recent snapshot.
    pub fn check_alerts(&self) -> Vec<Alert> {
        todo!()
    }

    /// Mean CPU usage across all stored snapshots.
    pub fn average_cpu_usage(&self) -> f64 {
        todo!()
    }

    /// Number of snapshots currently held.
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

    fn cpu(pct: f64) -> CpuMetrics {
        CpuMetrics { usage_percent: pct, core_count: 4 }
    }

    fn mem(total: u64, used: u64) -> MemoryMetrics {
        MemoryMetrics { total_bytes: total, used_bytes: used, free_bytes: total - used }
    }

    fn snap(cpu_pct: f64, mem_used: u64, mem_total: u64) -> Snapshot {
        Snapshot {
            timestamp: Instant::now(),
            cpu: cpu(cpu_pct),
            memory: mem(mem_total, mem_used),
            processes: vec![],
        }
    }

    fn threshold(cpu: f64, mem: f64) -> AlertThreshold {
        AlertThreshold { cpu_percent: cpu, memory_percent: mem }
    }

    // usage_percent = used / total * 100
    #[test]
    fn test_memory_usage_percent() {
        let m = mem(1_000_000, 250_000);
        assert!((m.usage_percent() - 25.0).abs() < 0.001);
    }

    // usage_percent on zero-total memory must not panic (returns 0.0).
    #[test]
    fn test_memory_usage_percent_zero_total() {
        let m = MemoryMetrics { total_bytes: 0, used_bytes: 0, free_bytes: 0 };
        assert_eq!(m.usage_percent(), 0.0);
    }

    // latest() returns None on an empty history.
    #[test]
    fn test_latest_on_empty_history_is_none() {
        let h = MetricsHistory::new(10, threshold(80.0, 80.0));
        assert!(h.latest().is_none());
    }

    // record() adds a snapshot; latest() reflects it.
    #[test]
    fn test_record_and_latest() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        h.record(snap(50.0, 500, 1000));
        assert!(h.latest().is_some());
        assert_eq!(h.len(), 1);
    }

    // History is bounded: oldest entry is discarded when the window is full.
    #[test]
    fn test_history_bounded_by_max_snapshots() {
        let mut h = MetricsHistory::new(3, threshold(80.0, 80.0));
        h.record(snap(10.0, 100, 1000));
        h.record(snap(20.0, 200, 1000));
        h.record(snap(30.0, 300, 1000));
        h.record(snap(40.0, 400, 1000)); // oldest (10%) dropped
        assert_eq!(h.len(), 3);
    }

    // No alert when metrics are below thresholds.
    #[test]
    fn test_no_alert_below_threshold() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        h.record(snap(50.0, 500, 1000));
        assert!(h.check_alerts().is_empty());
    }

    // CPU alert fires when CPU usage exceeds the threshold.
    #[test]
    fn test_high_cpu_alert_fires() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 90.0));
        h.record(snap(95.0, 500, 1000));
        let alerts = h.check_alerts();
        let cpu_alert = alerts.iter().any(|a| matches!(a.kind, AlertKind::HighCpu { .. }));
        assert!(cpu_alert, "expected a HighCpu alert");
    }

    // Memory alert fires when memory usage exceeds the threshold.
    #[test]
    fn test_high_memory_alert_fires() {
        let mut h = MetricsHistory::new(10, threshold(90.0, 80.0));
        h.record(snap(30.0, 900, 1000)); // 90% memory used
        let alerts = h.check_alerts();
        let mem_alert = alerts.iter().any(|a| matches!(a.kind, AlertKind::HighMemory { .. }));
        assert!(mem_alert, "expected a HighMemory alert");
    }

    // Both alerts can fire simultaneously.
    #[test]
    fn test_both_alerts_can_fire_together() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        h.record(snap(90.0, 900, 1000));
        assert_eq!(h.check_alerts().len(), 2);
    }

    // average_cpu_usage is the mean over all stored snapshots.
    #[test]
    fn test_average_cpu_usage() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        h.record(snap(10.0, 0, 1000));
        h.record(snap(20.0, 0, 1000));
        h.record(snap(30.0, 0, 1000));
        assert!((h.average_cpu_usage() - 20.0).abs() < 0.001);
    }

    // average_cpu_usage on empty history returns 0.0 without panicking.
    #[test]
    fn test_average_cpu_on_empty_history() {
        let h = MetricsHistory::new(10, threshold(80.0, 80.0));
        assert_eq!(h.average_cpu_usage(), 0.0);
    }

    // is_empty reflects whether any snapshots have been recorded.
    #[test]
    fn test_is_empty() {
        let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
        assert!(h.is_empty());
        h.record(snap(10.0, 0, 1000));
        assert!(!h.is_empty());
    }
}
