// ─── distributed-cache — integration demo ────────────────────────────────────
//
// Run with:  cargo run -p distributed-cache

use distributed_cache::{CacheConfig, LruCache};
use std::time::Duration;

fn main() {
    println!("=== distributed-cache integration demo ===\n");

    demo_basic_get_set();
    demo_lru_eviction();
    demo_ttl_expiry();
    demo_remove_and_clear();

    println!("\nAll demos completed.");
}

fn demo_basic_get_set() {
    println!("[ Demo 1 ] basic get/set");

    let mut c: LruCache<&str, i32> = LruCache::new(CacheConfig::new(10));
    c.set("x", 100);
    c.set("y", 200);

    assert_eq!(c.get(&"x"), Some(&100));
    assert_eq!(c.get(&"y"), Some(&200));
    assert_eq!(c.get(&"z"), None);
    assert_eq!(c.len(), 2);

    // Overwrite
    c.set("x", 999);
    assert_eq!(c.get(&"x"), Some(&999));
    assert_eq!(c.len(), 2, "overwrite must not grow len");

    println!("  get/set/overwrite  ✓");
}

fn demo_lru_eviction() {
    println!("[ Demo 2 ] LRU eviction");

    let mut c: LruCache<&str, i32> = LruCache::new(CacheConfig::new(3));
    c.set("a", 1);
    c.set("b", 2);
    c.set("c", 3);
    c.get(&"a"); // refresh a — b becomes LRU
    c.set("d", 4); // b must be evicted

    assert!(c.get(&"b").is_none(), "b must have been evicted");
    assert!(c.get(&"a").is_some());
    assert!(c.get(&"c").is_some());
    assert!(c.get(&"d").is_some());
    println!("  LRU eviction verified  ✓");
}

fn demo_ttl_expiry() {
    println!("[ Demo 3 ] TTL expiry");

    let mut c: LruCache<&str, &str> = LruCache::new(CacheConfig::new(10));
    c.set_with_ttl("short", "value", Some(Duration::from_millis(50)));
    c.set_with_ttl("long",  "value", Some(Duration::from_secs(60)));

    assert!(c.get(&"short").is_some(), "short entry must exist immediately");
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(c.get(&"short"), None,  "short entry must have expired");
    assert!(c.get(&"long").is_some(),  "long entry must still be alive");

    println!("  TTL expiry verified  ✓");
}

fn demo_remove_and_clear() {
    println!("[ Demo 4 ] remove / clear");

    let mut c: LruCache<&str, i32> = LruCache::new(CacheConfig::new(10));
    c.set("p", 1);
    c.set("q", 2);

    assert_eq!(c.remove(&"p"), Some(1));
    assert_eq!(c.get(&"p"), None);
    assert_eq!(c.len(), 1);

    c.clear();
    assert!(c.is_empty());
    println!("  remove / clear  ✓");
}
