use std::hash::Hash;
use std::time::Duration;

// ─── Types you need to implement ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub capacity: usize,
    pub default_ttl: Option<Duration>,
}

/// An LRU cache with optional per-entry TTL.
pub struct LruCache<K, V> {
    // TODO: add your fields
    _p: std::marker::PhantomData<(K, V)>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    pub fn new(config: CacheConfig) -> Self {
        todo!()
    }

    /// Return the value for `key`, or `None` if missing / expired.
    /// A successful hit should refresh the entry's LRU position.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        todo!()
    }

    /// Insert / update using the config's default TTL.
    pub fn set(&mut self, key: K, value: V) {
        todo!()
    }

    /// Insert / update with an explicit TTL override.
    pub fn set_with_ttl(&mut self, key: K, value: V, ttl: Option<Duration>) {
        todo!()
    }

    /// Remove the entry and return its value.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn cache(cap: usize) -> LruCache<&'static str, i32> {
        LruCache::new(CacheConfig { capacity: cap, default_ttl: None })
    }

    // A value that was just set must be retrievable.
    #[test]
    fn test_set_and_get() {
        let mut c = cache(10);
        c.set("foo", 42);
        assert_eq!(c.get(&"foo"), Some(&42));
    }

    // A key that was never inserted returns None.
    #[test]
    fn test_get_missing_returns_none() {
        let mut c = cache(10);
        assert_eq!(c.get(&"missing"), None);
    }

    // Overwriting an existing key must not grow the cache.
    #[test]
    fn test_update_does_not_grow_cache() {
        let mut c = cache(10);
        c.set("k", 1);
        c.set("k", 2);
        assert_eq!(c.len(), 1);
        assert_eq!(c.get(&"k"), Some(&2));
    }

    // Once capacity is full the least-recently-used entry is evicted.
    // Access order: insert a, b, c → get "a" → insert "d".
    // Expected eviction: "b" (oldest untouched after "a" was refreshed).
    #[test]
    fn test_lru_eviction_after_access() {
        let mut c = cache(3);
        c.set("a", 1);
        c.set("b", 2);
        c.set("c", 3);
        c.get(&"a"); // refresh "a"; "b" is now LRU
        c.set("d", 4);
        assert_eq!(c.len(), 3);
        assert_eq!(c.get(&"b"), None, "b should have been evicted");
        assert!(c.get(&"a").is_some());
        assert!(c.get(&"c").is_some());
        assert!(c.get(&"d").is_some());
    }

    // The very first inserted key is LRU when no accesses follow.
    #[test]
    fn test_lru_evicts_insertion_order_when_untouched() {
        let mut c = cache(2);
        c.set("first", 1);
        c.set("second", 2);
        c.set("third", 3); // "first" evicted
        assert_eq!(c.get(&"first"), None);
        assert!(c.get(&"second").is_some());
        assert!(c.get(&"third").is_some());
    }

    // remove() returns the stored value and makes the key inaccessible.
    #[test]
    fn test_remove_existing_key() {
        let mut c = cache(10);
        c.set("x", 99);
        assert_eq!(c.remove(&"x"), Some(99));
        assert_eq!(c.get(&"x"), None);
        assert_eq!(c.len(), 0);
    }

    // remove() on a missing key must return None without panicking.
    #[test]
    fn test_remove_missing_key_returns_none() {
        let mut c = cache(10);
        assert_eq!(c.remove(&"ghost"), None);
    }

    // An entry with a short TTL must be invisible after the deadline.
    #[test]
    fn test_ttl_entry_expires() {
        let mut c: LruCache<&str, &str> =
            LruCache::new(CacheConfig { capacity: 10, default_ttl: None });
        c.set_with_ttl("temp", "val", Some(Duration::from_millis(50)));
        assert_eq!(c.get(&"temp"), Some(&"val"), "should exist before expiry");
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(c.get(&"temp"), None, "should be gone after expiry");
    }

    // An entry without TTL must survive indefinitely.
    #[test]
    fn test_entry_without_ttl_does_not_expire() {
        let mut c = cache(10);
        c.set("perm", 7);
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(c.get(&"perm"), Some(&7));
    }

    // The default TTL from config applies to set() calls.
    #[test]
    fn test_default_ttl_applied_via_set() {
        let mut c: LruCache<&str, i32> = LruCache::new(CacheConfig {
            capacity: 10,
            default_ttl: Some(Duration::from_millis(50)),
        });
        c.set("k", 1);
        assert!(c.get(&"k").is_some());
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(c.get(&"k"), None);
    }

    // clear() removes every entry.
    #[test]
    fn test_clear_empties_the_cache() {
        let mut c = cache(10);
        c.set("a", 1);
        c.set("b", 2);
        c.clear();
        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
    }

    // capacity() reflects the limit given to the constructor.
    #[test]
    fn test_capacity_reflects_config() {
        let c = cache(42);
        assert_eq!(c.capacity(), 42);
    }

    // A brand-new cache must report itself as empty.
    #[test]
    fn test_new_cache_is_empty() {
        let c = cache(10);
        assert!(c.is_empty());
    }
}
