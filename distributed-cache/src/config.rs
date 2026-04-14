use std::time::Duration;

// ─── CacheConfig ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries the cache will hold.
    pub capacity: usize,
    /// Default TTL applied to entries inserted via `set()`.
    /// `None` means entries never expire unless explicitly removed or evicted.
    pub default_ttl: Option<Duration>,
}

impl CacheConfig {
    pub fn new(capacity: usize) -> Self {
        CacheConfig { capacity, default_ttl: None }
    }

    pub fn with_ttl(capacity: usize, ttl: Duration) -> Self {
        CacheConfig { capacity, default_ttl: Some(ttl) }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // new() sets no TTL.
    #[test]
    fn test_new_has_no_ttl() {
        let c = CacheConfig::new(10);
        assert_eq!(c.capacity, 10);
        assert!(c.default_ttl.is_none());
    }

    // with_ttl() stores the TTL.
    #[test]
    fn test_with_ttl_stores_duration() {
        let ttl = Duration::from_secs(60);
        let c = CacheConfig::with_ttl(5, ttl);
        assert_eq!(c.capacity, 5);
        assert_eq!(c.default_ttl, Some(ttl));
    }
}
