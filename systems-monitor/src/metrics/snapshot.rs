use std::time::Instant;
use super::{CpuMetrics, MemoryMetrics, ProcessMetrics};

// ─── Snapshot — point-in-time system reading ──────────────────────────────────

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub timestamp: Instant,
    pub cpu:       CpuMetrics,
    pub memory:    MemoryMetrics,
    pub processes: Vec<ProcessMetrics>,
}

impl Snapshot {
    pub fn new(cpu: CpuMetrics, memory: MemoryMetrics, processes: Vec<ProcessMetrics>) -> Self {
        Snapshot { timestamp: Instant::now(), cpu, memory, processes }
    }

    /// Total memory consumed across all tracked processes.
    pub fn total_process_memory(&self) -> u64 {
        todo!()
    }

    /// The process with the highest CPU usage, or None if empty.
    pub fn top_cpu_process(&self) -> Option<&ProcessMetrics> {
        todo!()
    }

    /// The process with the highest memory usage, or None if empty.
    pub fn top_memory_process(&self) -> Option<&ProcessMetrics> {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn snap(cpu_pct: f64, procs: Vec<ProcessMetrics>) -> Snapshot {
        Snapshot::new(
            CpuMetrics::new(cpu_pct, 4),
            MemoryMetrics::new(1000, 500),
            procs,
        )
    }

    fn proc(pid: u32, cpu: f64, mem: u64) -> ProcessMetrics {
        ProcessMetrics::new(pid, "p", cpu, mem, 1)
    }

    #[test]
    fn test_total_process_memory() {
        let s = snap(50.0, vec![proc(1, 10.0, 100), proc(2, 20.0, 200)]);
        assert_eq!(s.total_process_memory(), 300);
    }

    #[test]
    fn test_total_process_memory_empty() {
        let s = snap(0.0, vec![]);
        assert_eq!(s.total_process_memory(), 0);
    }

    #[test]
    fn test_top_cpu_process() {
        let s = snap(80.0, vec![proc(1, 5.0, 0), proc(2, 80.0, 0), proc(3, 30.0, 0)]);
        assert_eq!(s.top_cpu_process().unwrap().pid, 2);
    }

    #[test]
    fn test_top_cpu_process_none_when_empty() {
        let s = snap(0.0, vec![]);
        assert!(s.top_cpu_process().is_none());
    }

    #[test]
    fn test_top_memory_process() {
        let s = snap(50.0, vec![proc(1, 0.0, 50), proc(2, 0.0, 500), proc(3, 0.0, 100)]);
        assert_eq!(s.top_memory_process().unwrap().pid, 2);
    }
}
