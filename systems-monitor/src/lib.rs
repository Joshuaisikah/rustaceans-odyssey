// ─── systems-monitor ─────────────────────────────────────────────────────────
//
// A rolling-window system metrics monitor with threshold-based alerting.
//
// Crate layout
// ┌────────────────────────────────────────────────────────┐
// │ metrics/              Collected measurements           │
// │   cpu.rs           →  CpuMetrics                      │
// │   memory.rs        →  MemoryMetrics                   │
// │   process.rs       →  ProcessMetrics                  │
// │   snapshot.rs      →  Snapshot (point-in-time)        │
// ├────────────────────────────────────────────────────────┤
// │ alert.rs              Alert types + thresholds        │
// │ history.rs            MetricsHistory (rolling window) │
// └────────────────────────────────────────────────────────┘

pub mod alert;
pub mod history;
pub mod metrics;

pub use alert::{Alert, AlertKind, AlertThreshold};
pub use history::MetricsHistory;
pub use metrics::{CpuMetrics, MemoryMetrics, ProcessMetrics, Snapshot};
