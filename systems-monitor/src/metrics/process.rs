// ─── ProcessMetrics ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ProcessMetrics {
    pub pid:          u32,
    pub name:         String,
    pub cpu_percent:  f64,
    pub memory_bytes: u64,
    pub threads:      u32,
}

impl ProcessMetrics {
    pub fn new(pid: u32, name: &str, cpu_percent: f64, memory_bytes: u64, threads: u32) -> Self {
        ProcessMetrics {
            pid,
            name: name.to_string(),
            cpu_percent,
            memory_bytes,
            threads,
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields_stored_correctly() {
        let p = ProcessMetrics::new(42, "server", 12.5, 1_000_000, 8);
        assert_eq!(p.pid,          42);
        assert_eq!(p.name,         "server");
        assert_eq!(p.cpu_percent,  12.5);
        assert_eq!(p.memory_bytes, 1_000_000);
        assert_eq!(p.threads,      8);
    }

    #[test]
    fn test_clone_is_independent() {
        let a = ProcessMetrics::new(1, "a", 10.0, 100, 1);
        let mut b = a.clone();
        b.cpu_percent = 99.0;
        assert_eq!(a.cpu_percent, 10.0);
    }
}
