use std::hash::Hash;
use std::time::{Duration, Instant};
use crate::config::CacheConfig;

// ─── LruCache ────────────────────────────────────────────────────────────────
//
// LRU (Least-Recently-Used) eviction cache with optional per-entry TTL.
//
// Design hints:
//   - Use a HashMap for O(1) lookups and a VecDeque (or LinkedList) to track
//     access order.  Alternatively, use the `linked_hash_map` pattern (two
//     HashMap entries sharing an index into a linked list).
//   - Each stored entry should hold the value, an optional expiry Instant,
//     and a position in the LRU order.
//   - get() must refresh the entry's position to the front ("most recent").
//   - set() must evict the LRU entry when capacity is exceeded.
//   - TTL expiry is checked lazily inside get().

pub struct LruCache<K, V> {
    // TODO: add your fields
    _p: std::marker::PhantomData<(K, V)>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    pub fn new(config: CacheConfig) -> Self {
        todo!()
    }

    /// Return the value for `key`, or None if missing or expired.
    /// On a hit, move the entry to the front of the LRU order.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        todo!()
    }

    /// Insert / overwrite using the config's default TTL.
    pub fn set(&mut self, key: K, value: V) {
        todo!()
    }

    /// Insert / overwrite with an explicit TTL override.
    pub fn set_with_ttl(&mut self, key: K, value: V, ttl: Option<Duration>) {
        todo!()
    }

    /// Remove an entry and return its value, or None if absent / expired.
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

    /// Remove all entries.
    pub fn clear(&mut self) {
        todo!()
    }

    /// Explicitly remove all entries that have passed their TTL deadline.
    pub fn evict_expired(&mut self) {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn cache(cap: usize) -> LruCache<&'static str, i32> {
        LruCache::new(CacheConfig::new(cap))
    }

    // A brand new cache is empty.
    #[test]
    fn test_new_cache_is_empty() {
        let c = cache(10);
        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
    }

    // capacity() returns the value given to the constructor.
    #[test]
    fn test_capacity_reflects_config() {
        let c = cache(42);
        assert_eq!(c.capacity(), 42);
    }

    // set() then get() returns the stored value.
    #[test]
    fn test_set_and_get() {
        let mut c = cache(10);
        c.set("foo", 42);
        assert_eq!(c.get(&"foo"), Some(&42));
    }

    // get() on a missing key returns None.
    #[test]
    fn test_get_missing_returns_none() {
        let mut c = cache(10);
        assert_eq!(c.get(&"missing"), None);
    }

    // Overwriting a key must not grow the cache.
    #[test]
    fn test_update_does_not_grow_len() {
        let mut c = cache(10);
        c.set("k", 1);
        c.set("k", 2);
        assert_eq!(c.len(), 1);
        assert_eq!(c.get(&"k"), Some(&2));
    }

    // When capacity is full, the least-recently-used entry is evicted.
    // Insert a, b, c → access a → insert d → b should be evicted.
    #[test]
    fn test_lru_eviction_after_access() {
        let mut c = cache(3);
        c.set("a", 1);
        c.set("b", 2);
        c.set("c", 3);
        c.get(&"a");       // refresh a; b is now LRU
        c.set("d", 4);
        assert_eq!(c.get(&"b"), None, "b must have been evicted");
        assert!(c.get(&"a").is_some());
        assert!(c.get(&"c").is_some());
        assert!(c.get(&"d").is_some());
    }

    // Without any get() calls, the insertion-order first entry is evicted.
    #[test]
    fn test_lru_evicts_insertion_order_when_untouched() {
        let mut c = cache(2);
        c.set("first",  1);
        c.set("second", 2);
        c.set("third",  3); // "first" must be evicted
        assert_eq!(c.get(&"first"), None);
        assert!(c.get(&"second").is_some());
        assert!(c.get(&"third").is_some());
    }

    // remove() returns the value and makes the key inaccessible.
    #[test]
    fn test_remove_existing_key() {
        let mut c = cache(10);
        c.set("x", 99);
        assert_eq!(c.remove(&"x"), Some(99));
        assert_eq!(c.get(&"x"), None);
        assert_eq!(c.len(), 0);
    }

    // remove() on a missing key returns None without panicking.
    #[test]
    fn test_remove_missing_key_returns_none() {
        let mut c = cache(10);
        assert_eq!(c.remove(&"ghost"), None);
    }

    // An entry with a short TTL is invisible after the deadline.
    #[test]
    fn test_ttl_entry_expires() {
        let mut c: LruCache<&str, &str> = LruCache::new(CacheConfig::new(10));
        c.set_with_ttl("temp", "val", Some(Duration::from_millis(50)));
        assert_eq!(c.get(&"temp"), Some(&"val"), "should exist before expiry");
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(c.get(&"temp"), None, "should be gone after expiry");
    }

    // An entry without TTL survives indefinitely.
    #[test]
    fn test_entry_without_ttl_does_not_expire() {
        let mut c = cache(10);
        c.set("perm", 7);
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(c.get(&"perm"), Some(&7));
    }

    // The default TTL from config is applied to set() calls.
    #[test]
    fn test_default_ttl_applied_by_set() {
        let mut c: LruCache<&str, i32> =
            LruCache::new(CacheConfig::with_ttl(10, Duration::from_millis(50)));
        c.set("k", 1);
        assert!(c.get(&"k").is_some());
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(c.get(&"k"), None);
    }

    // clear() removes every entry and reports the cache as empty.
    #[test]
    fn test_clear_empties_cache() {
        let mut c = cache(10);
        c.set("a", 1);
        c.set("b", 2);
        c.clear();
        assert!(c.is_empty());
    }

    // evict_expired() purges stale entries without touching live ones.
    #[test]
    fn test_evict_expired_leaves_live_entries() {
        let mut c: LruCache<&str, i32> = LruCache::new(CacheConfig::new(10));
        c.set_with_ttl("stale", 1, Some(Duration::from_millis(30)));
        c.set_with_ttl("alive", 2, Some(Duration::from_secs(60)));
        std::thread::sleep(Duration::from_millis(60));
        c.evict_expired();
        assert_eq!(c.get(&"stale"), None);
        assert_eq!(c.get(&"alive"), Some(&2));
    }
}
