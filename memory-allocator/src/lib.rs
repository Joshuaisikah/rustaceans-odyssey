// ─── memory-allocator ────────────────────────────────────────────────────────
//
// Custom memory allocators built from scratch.
//
// Module layout:
//   arena — Bump (arena) allocator: O(1) alloc, all-at-once free via reset()
//   pool  — Fixed-size block allocator: O(1) alloc + free via free-list
//   stats — AllocStats: counters you can embed in any allocator

// ─── memory-allocator ────────────────────────────────────────────────────────
//
// Custom memory allocators built from scratch.
//
// Crate layout
// ┌────────────────────────────────────────────────────────┐
// │ traits.rs    Allocator trait (shared interface)       │
// │ arena.rs     Bump allocator: O(1) alloc, reset()      │
// │ pool.rs      Fixed-size block allocator: free-list    │
// │ stats.rs     AllocStats: counters / peak tracking     │
// └────────────────────────────────────────────────────────┘

pub mod arena;
pub mod pool;
pub mod stats;
pub mod traits;

pub use arena::Arena;
pub use pool::PoolAllocator;
pub use stats::AllocStats;
pub use traits::Allocator;
