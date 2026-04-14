// ─── systems-monitor — integration demo ──────────────────────────────────────
//
// Run with:  cargo run -p systems-monitor

use systems_monitor::{
    Alert, AlertKind, AlertThreshold, CpuMetrics, MemoryMetrics,
    MetricsHistory, ProcessMetrics, Snapshot,
};

fn main() {
    println!("=== systems-monitor integration demo ===\n");

    demo_snapshot_metrics();
    demo_rolling_history();
    demo_alerting();

    println!("\nAll demos completed.");
}

// ── Demo 1: Snapshot ──────────────────────────────────────────────────────────
fn demo_snapshot_metrics() {
    println!("[ Demo 1 ] Snapshot metrics");

    let procs = vec![
        ProcessMetrics { pid: 1, name: "kernel".into(), cpu_percent: 5.0,  memory_bytes: 50_000,  threads: 8 },
        ProcessMetrics { pid: 2, name: "server".into(), cpu_percent: 45.0, memory_bytes: 200_000, threads: 4 },
        ProcessMetrics { pid: 3, name: "agent".into(),  cpu_percent: 12.0, memory_bytes: 30_000,  threads: 2 },
    ];

    let snap = Snapshot::new(
        CpuMetrics    { usage_percent: 62.0, core_count: 8 },
        MemoryMetrics { total_bytes: 8_000_000, used_bytes: 5_000_000, free_bytes: 3_000_000 },
        procs,
    );

    let mem_pct = snap.memory.usage_percent();
    assert!((mem_pct - 62.5).abs() < 0.1);
    println!("  memory usage: {mem_pct:.1}%  ✓");

    let top = snap.top_cpu_process().unwrap();
    assert_eq!(top.name, "server");
    println!("  top CPU process: {} ({:.1}%)  ✓", top.name, top.cpu_percent);

    let total_mem = snap.total_process_memory();
    assert_eq!(total_mem, 280_000);
    println!("  total process memory: {total_mem} bytes  ✓");
}

// ── Demo 2: Rolling history window ───────────────────────────────────────────
fn demo_rolling_history() {
    println!("[ Demo 2 ] Rolling history (max 4 snapshots)");

    let threshold = AlertThreshold::new(90.0, 90.0); // high threshold → no alerts
    let mut history = MetricsHistory::new(4, threshold);

    for (cpu, mem) in [(10.0, 200), (20.0, 300), (30.0, 400), (40.0, 500), (50.0, 600)] {
        history.record(Snapshot::new(
            CpuMetrics    { usage_percent: cpu, core_count: 4 },
            MemoryMetrics { total_bytes: 1000, used_bytes: mem, free_bytes: 1000 - mem },
            vec![],
        ));
    }

    assert_eq!(history.len(), 4, "oldest should have been dropped");
    println!("  window len = {} (oldest dropped)  ✓", history.len());

    let avg = history.average_cpu_usage();
    println!("  average CPU: {avg:.1}%  ✓");

    let peak = history.peak_cpu_usage();
    println!("  peak CPU:    {peak:.1}%  ✓");
}

// ── Demo 3: Alert firing ──────────────────────────────────────────────────────
fn demo_alerting() {
    println!("[ Demo 3 ] Alert detection");

    let threshold = AlertThreshold::new(80.0, 80.0);
    let mut history = MetricsHistory::new(10, threshold);

    // Normal snapshot — no alerts.
    history.record(Snapshot::new(
        CpuMetrics    { usage_percent: 50.0, core_count: 4 },
        MemoryMetrics { total_bytes: 1000, used_bytes: 500, free_bytes: 500 },
        vec![],
    ));
    assert!(history.check_alerts().is_empty());
    println!("  normal snapshot → no alerts  ✓");

    // Spike snapshot — both thresholds exceeded.
    history.record(Snapshot::new(
        CpuMetrics    { usage_percent: 95.0, core_count: 4 },
        MemoryMetrics { total_bytes: 1000, used_bytes: 950, free_bytes: 50 },
        vec![],
    ));

    let alerts = history.check_alerts();
    assert_eq!(alerts.len(), 2, "expected HighCpu + HighMemory");

    for alert in &alerts {
        match &alert.kind {
            AlertKind::HighCpu    { actual } => println!("  ALERT: HighCpu    {actual}%  ✓"),
            AlertKind::HighMemory { actual } => println!("  ALERT: HighMemory {actual}%  ✓"),
        }
    }
}
