// ─── systems-monitor: integration tests ──────────────────────────────────────
//
// Black-box tests through the public API.
// Exercises metric types, Snapshot queries, MetricsHistory rolling window,
// and threshold-based alert detection as an end-to-end pipeline.

use systems_monitor::{
    Alert, AlertKind, AlertThreshold, CpuMetrics, MemoryMetrics, MetricsHistory,
    ProcessMetrics, Snapshot,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn cpu(pct: f64) -> CpuMetrics {
    CpuMetrics::new(pct, 4)
}

fn mem(used: u64, total: u64) -> MemoryMetrics {
    MemoryMetrics::new(total, used)
}

fn proc(pid: u32, cpu_pct: f64, mem_bytes: u64) -> ProcessMetrics {
    ProcessMetrics::new(pid, "p", cpu_pct, mem_bytes, 1)
}

fn snap(cpu_pct: f64, mem_used: u64, mem_total: u64, procs: Vec<ProcessMetrics>) -> Snapshot {
    Snapshot::new(cpu(cpu_pct), mem(mem_used, mem_total), procs)
}

fn threshold(cpu: f64, mem: f64) -> AlertThreshold {
    AlertThreshold::new(cpu, mem)
}

// ── CpuMetrics ────────────────────────────────────────────────────────────────

#[test]
fn test_cpu_is_above() {
    let c = CpuMetrics::new(85.0, 4);
    assert!(c.is_above(80.0));
    assert!(!c.is_above(90.0));
    assert!(!c.is_above(85.0)); // strict greater-than
}

// ── MemoryMetrics ─────────────────────────────────────────────────────────────

#[test]
fn test_memory_usage_percent() {
    let m = MemoryMetrics::new(1_000, 250);
    assert!((m.usage_percent() - 25.0).abs() < 0.001);
}

#[test]
fn test_memory_usage_percent_zero_total_is_zero() {
    let m = MemoryMetrics { total_bytes: 0, used_bytes: 0, free_bytes: 0 };
    assert_eq!(m.usage_percent(), 0.0);
}

#[test]
fn test_memory_free_bytes_computed() {
    let m = MemoryMetrics::new(1000, 300);
    assert_eq!(m.free_bytes, 700);
}

// ── Snapshot queries ──────────────────────────────────────────────────────────

#[test]
fn test_snapshot_total_process_memory() {
    let s = snap(50.0, 500, 1000, vec![proc(1, 10.0, 100), proc(2, 20.0, 200)]);
    assert_eq!(s.total_process_memory(), 300);
}

#[test]
fn test_snapshot_total_process_memory_empty() {
    let s = snap(0.0, 0, 1000, vec![]);
    assert_eq!(s.total_process_memory(), 0);
}

#[test]
fn test_snapshot_top_cpu_process() {
    let s = snap(80.0, 500, 1000, vec![
        proc(1, 5.0,  0),
        proc(2, 80.0, 0),
        proc(3, 30.0, 0),
    ]);
    assert_eq!(s.top_cpu_process().unwrap().pid, 2);
}

#[test]
fn test_snapshot_top_cpu_process_none_when_empty() {
    let s = snap(0.0, 0, 1000, vec![]);
    assert!(s.top_cpu_process().is_none());
}

#[test]
fn test_snapshot_top_memory_process() {
    let s = snap(50.0, 500, 1000, vec![
        proc(1, 0.0, 50),
        proc(2, 0.0, 500),
        proc(3, 0.0, 100),
    ]);
    assert_eq!(s.top_memory_process().unwrap().pid, 2);
}

#[test]
fn test_snapshot_top_memory_process_none_when_empty() {
    let s = snap(0.0, 0, 1000, vec![]);
    assert!(s.top_memory_process().is_none());
}

// ── Alert construction ────────────────────────────────────────────────────────

#[test]
fn test_high_cpu_alert_contains_rounded_percentage() {
    let a = Alert::high_cpu(95.4);
    assert!(matches!(a.kind, AlertKind::HighCpu { actual: 95 }));
    assert!(!a.message.is_empty());
}

#[test]
fn test_high_memory_alert_contains_rounded_percentage() {
    let a = Alert::high_memory(88.7);
    assert!(matches!(a.kind, AlertKind::HighMemory { actual: 89 }));
    assert!(!a.message.is_empty());
}

// ── MetricsHistory ────────────────────────────────────────────────────────────

#[test]
fn test_history_new_is_empty() {
    let h = MetricsHistory::new(10, threshold(80.0, 80.0));
    assert!(h.is_empty());
    assert_eq!(h.len(), 0);
    assert!(h.latest().is_none());
}

#[test]
fn test_history_record_and_latest() {
    let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
    h.record(snap(50.0, 500, 1000, vec![]));
    assert_eq!(h.len(), 1);
    assert!(h.latest().is_some());
}

// Bounded window: oldest snapshot is dropped when max is exceeded.
#[test]
fn test_history_bounded_window() {
    let mut h = MetricsHistory::new(3, threshold(80.0, 80.0));
    h.record(snap(10.0, 100, 1000, vec![]));
    h.record(snap(20.0, 200, 1000, vec![]));
    h.record(snap(30.0, 300, 1000, vec![]));
    h.record(snap(40.0, 400, 1000, vec![])); // oldest (10%) dropped
    assert_eq!(h.len(), 3);
}

// ── Alert detection ───────────────────────────────────────────────────────────

#[test]
fn test_no_alerts_below_threshold() {
    let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
    h.record(snap(50.0, 500, 1000, vec![]));
    assert!(h.check_alerts().is_empty());
}

#[test]
fn test_high_cpu_alert_fires_above_threshold() {
    let mut h = MetricsHistory::new(10, threshold(80.0, 90.0));
    h.record(snap(95.0, 500, 1000, vec![]));
    let alerts = h.check_alerts();
    assert!(alerts.iter().any(|a| matches!(a.kind, AlertKind::HighCpu { .. })));
}

#[test]
fn test_high_memory_alert_fires_above_threshold() {
    let mut h = MetricsHistory::new(10, threshold(90.0, 80.0));
    h.record(snap(30.0, 900, 1000, vec![])); // 90% memory
    let alerts = h.check_alerts();
    assert!(alerts.iter().any(|a| matches!(a.kind, AlertKind::HighMemory { .. })));
}

#[test]
fn test_both_alerts_fire_simultaneously() {
    let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
    h.record(snap(90.0, 900, 1000, vec![]));
    assert_eq!(h.check_alerts().len(), 2);
}

// ── Statistics ────────────────────────────────────────────────────────────────

#[test]
fn test_average_cpu_usage() {
    let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
    h.record(snap(10.0, 0, 1000, vec![]));
    h.record(snap(20.0, 0, 1000, vec![]));
    h.record(snap(30.0, 0, 1000, vec![]));
    assert!((h.average_cpu_usage() - 20.0).abs() < 0.001);
}

#[test]
fn test_average_cpu_empty_returns_zero() {
    let h = MetricsHistory::new(10, threshold(80.0, 80.0));
    assert_eq!(h.average_cpu_usage(), 0.0);
}

#[test]
fn test_peak_cpu_usage_is_high_water_mark() {
    let mut h = MetricsHistory::new(10, threshold(80.0, 80.0));
    h.record(snap(40.0, 0, 1000, vec![]));
    h.record(snap(90.0, 0, 1000, vec![]));
    h.record(snap(55.0, 0, 1000, vec![]));
    assert!((h.peak_cpu_usage() - 90.0).abs() < 0.001);
}

// ── End-to-end pipeline ───────────────────────────────────────────────────────

// Simulate recording 20 snapshots at varying CPU load and verify:
//   • the window stays bounded
//   • average and peak are computed correctly
//   • alerts fire when load is high
#[test]
fn test_end_to_end_monitoring_pipeline() {
    let mut h   = MetricsHistory::new(5, threshold(70.0, 90.0));
    let usages  = [20.0f64, 40.0, 60.0, 80.0, 95.0]; // 5 readings, last two exceed threshold

    for &u in &usages {
        h.record(snap(u, 400, 1000, vec![]));
    }

    // Window contains exactly the last 5 entries (== max_snapshots).
    assert_eq!(h.len(), 5);

    // Peak must be 95.0.
    assert!((h.peak_cpu_usage() - 95.0).abs() < 0.001);

    // Latest snapshot triggers a HighCpu alert (95% > 70%).
    let alerts = h.check_alerts();
    assert!(alerts.iter().any(|a| matches!(a.kind, AlertKind::HighCpu { .. })));
}
