// ─── distributed-cache: integration tests ────────────────────────────────────
//
// Black-box tests through the public API.
// Exercises CacheConfig → LruCache, eviction ordering, TTL, and EvictionPolicy.

use distributed_cache::{CacheConfig, EvictionPolicy, LruCache, LruPolicy};
use std::time::Duration;

// ── Basic cache operations ────────────────────────────────────────────────────

#[test]
fn test_fresh_cache_is_empty() {
    let c: LruCache<&str, i32> = LruCache::new(CacheConfig::new(10));
    assert!(c.is_empty());
    assert_eq!(c.len(), 0);
    assert_eq!(c.capacity(), 10);
}

#[test]
fn test_set_and_get_roundtrip() {
    let mut c = LruCache::new(CacheConfig::new(10));
    c.set("hello", 42i32);
    assert_eq!(c.get(&"hello"), Some(&42));
}

#[test]
fn test_missing_key_returns_none() {
    let mut c: LruCache<&str, i32> = LruCache::new(CacheConfig::new(10));
    assert_eq!(c.get(&"ghost"), None);
}

#[test]
fn test_overwrite_does_not_grow_len() {
    let mut c = LruCache::new(CacheConfig::new(10));
    c.set("k", 1i32);
    c.set("k", 2);
    assert_eq!(c.len(), 1);
    assert_eq!(c.get(&"k"), Some(&2));
}

// ── LRU eviction ─────────────────────────────────────────────────────────────

// Insert a, b, c into a capacity-3 cache.  Access a (making b the LRU).
// Insert d — b should be evicted.
#[test]
fn test_lru_eviction_respects_access_order() {
    let mut c = LruCache::new(CacheConfig::new(3));
    c.set("a", 1i32);
    c.set("b", 2);
    c.set("c", 3);
    c.get(&"a"); // refresh a; b is now LRU
    c.set("d", 4);

    assert!(c.get(&"b").is_none(), "b must be evicted (LRU)");
    assert!(c.get(&"a").is_some());
    assert!(c.get(&"c").is_some());
    assert!(c.get(&"d").is_some());
}

// Insert into a capacity-2 cache without any gets.  First inserted is evicted.
#[test]
fn test_lru_evicts_oldest_insertion_without_access() {
    let mut c = LruCache::new(CacheConfig::new(2));
    c.set("first",  1i32);
    c.set("second", 2);
    c.set("third",  3); // "first" evicted
    assert!(c.get(&"first").is_none());
    assert!(c.get(&"second").is_some());
    assert!(c.get(&"third").is_some());
}

// ── Remove ────────────────────────────────────────────────────────────────────

#[test]
fn test_remove_returns_value_and_shrinks_cache() {
    let mut c = LruCache::new(CacheConfig::new(10));
    c.set("x", 99i32);
    assert_eq!(c.remove(&"x"), Some(99));
    assert!(c.get(&"x").is_none());
    assert_eq!(c.len(), 0);
}

#[test]
fn test_remove_missing_key_returns_none() {
    let mut c: LruCache<&str, i32> = LruCache::new(CacheConfig::new(10));
    assert_eq!(c.remove(&"ghost"), None);
}

// ── Clear ─────────────────────────────────────────────────────────────────────

#[test]
fn test_clear_empties_cache() {
    let mut c = LruCache::new(CacheConfig::new(10));
    c.set("a", 1i32);
    c.set("b", 2);
    c.clear();
    assert!(c.is_empty());
    assert_eq!(c.len(), 0);
}

// ── TTL ───────────────────────────────────────────────────────────────────────

#[test]
fn test_entry_with_explicit_ttl_expires() {
    let mut c: LruCache<&str, &str> = LruCache::new(CacheConfig::new(10));
    c.set_with_ttl("tmp", "val", Some(Duration::from_millis(50)));
    assert_eq!(c.get(&"tmp"), Some(&"val"));
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(c.get(&"tmp"), None, "entry must be expired");
}

#[test]
fn test_entry_without_ttl_persists() {
    let mut c = LruCache::new(CacheConfig::new(10));
    c.set("perm", 7i32);
    std::thread::sleep(Duration::from_millis(20));
    assert_eq!(c.get(&"perm"), Some(&7));
}

#[test]
fn test_config_default_ttl_applied_on_set() {
    let mut c: LruCache<&str, i32> =
        LruCache::new(CacheConfig::with_ttl(10, Duration::from_millis(60)));
    c.set("k", 1);
    assert!(c.get(&"k").is_some());
    std::thread::sleep(Duration::from_millis(120));
    assert!(c.get(&"k").is_none(), "default TTL must have expired the entry");
}

#[test]
fn test_evict_expired_purges_stale_keeps_live() {
    let mut c: LruCache<&str, i32> = LruCache::new(CacheConfig::new(10));
    c.set_with_ttl("stale", 1, Some(Duration::from_millis(30)));
    c.set_with_ttl("live",  2, Some(Duration::from_secs(60)));
    std::thread::sleep(Duration::from_millis(60));
    c.evict_expired();
    assert!(c.get(&"stale").is_none(), "stale entry must be purged");
    assert_eq!(c.get(&"live"), Some(&2));
}

// ── LruPolicy standalone ──────────────────────────────────────────────────────

#[test]
fn test_lru_policy_new_is_empty() {
    let p = LruPolicy::<&str>::new();
    assert!(p.is_empty());
    assert_eq!(p.len(), 0);
}

#[test]
fn test_lru_policy_touch_and_evict() {
    let mut p = LruPolicy::<&str>::new();
    p.touch(&"a");
    p.touch(&"b");
    p.touch(&"c");
    // "a" is LRU (first touched, never refreshed)
    assert_eq!(p.evict(), Some("a"));
}

#[test]
fn test_lru_policy_touch_refreshes_to_mru() {
    let mut p = LruPolicy::<&str>::new();
    p.touch(&"a");
    p.touch(&"b");
    p.touch(&"a"); // refresh a; b is now LRU
    assert_eq!(p.evict(), Some("b"));
}

#[test]
fn test_lru_policy_remove_and_clear() {
    let mut p = LruPolicy::<&str>::new();
    p.touch(&"x");
    p.touch(&"y");
    p.remove(&"x");
    assert_eq!(p.len(), 1);
    p.clear();
    assert!(p.is_empty());
}

#[test]
fn test_lru_policy_evict_empty_returns_none() {
    let mut p = LruPolicy::<&str>::new();
    assert_eq!(p.evict(), None);
}
