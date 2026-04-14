// ─── Alerting ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AlertThreshold {
    /// Fire a HighCpu alert when CPU usage exceeds this percentage.
    pub cpu_percent:    f64,
    /// Fire a HighMemory alert when memory usage exceeds this percentage.
    pub memory_percent: f64,
}

impl AlertThreshold {
    pub fn new(cpu_percent: f64, memory_percent: f64) -> Self {
        AlertThreshold { cpu_percent, memory_percent }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertKind {
    HighCpu    { actual: u64 }, // actual rounded to nearest integer
    HighMemory { actual: u64 },
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub kind:    AlertKind,
    pub message: String,
}

impl Alert {
    pub fn high_cpu(actual_pct: f64) -> Self {
        Alert {
            kind:    AlertKind::HighCpu { actual: actual_pct.round() as u64 },
            message: format!("High CPU usage: {:.1}%", actual_pct),
        }
    }

    pub fn high_memory(actual_pct: f64) -> Self {
        Alert {
            kind:    AlertKind::HighMemory { actual: actual_pct.round() as u64 },
            message: format!("High memory usage: {:.1}%", actual_pct),
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // high_cpu alert contains the rounded percentage.
    #[test]
    fn test_high_cpu_alert_kind() {
        let a = Alert::high_cpu(95.4);
        assert!(matches!(a.kind, AlertKind::HighCpu { actual: 95 }));
        assert!(a.message.contains("95"));
    }

    // high_memory alert contains the rounded percentage.
    #[test]
    fn test_high_memory_alert_kind() {
        let a = Alert::high_memory(88.7);
        assert!(matches!(a.kind, AlertKind::HighMemory { actual: 89 }));
        assert!(a.message.contains("88.7"));
    }

    // AlertThreshold stores the given values.
    #[test]
    fn test_threshold_values() {
        let t = AlertThreshold::new(80.0, 90.0);
        assert_eq!(t.cpu_percent,    80.0);
        assert_eq!(t.memory_percent, 90.0);
    }
}
