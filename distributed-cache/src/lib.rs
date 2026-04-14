// ─── distributed-cache ───────────────────────────────────────────────────────
//
// An LRU cache with optional per-entry TTL.
//
// Module layout:
//   config — CacheConfig (capacity + default TTL)
//   cache  — LruCache<K, V>
//   policy — EvictionPolicy trait + LruPolicy implementation

pub mod cache;
pub mod config;
pub mod policy;

pub use cache::LruCache;
pub use config::CacheConfig;
pub use policy::{EvictionPolicy, LruPolicy};
