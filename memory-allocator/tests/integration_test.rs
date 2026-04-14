// ─── memory-allocator: integration tests ─────────────────────────────────────
//
// Black-box tests through the public API.
// Exercises Arena, PoolAllocator, and AllocStats in realistic usage scenarios.

use memory_allocator::{AllocStats, Arena, PoolAllocator};

// ── Arena ─────────────────────────────────────────────────────────────────────

#[test]
fn test_arena_starts_empty() {
    let a = Arena::new(1024);
    assert_eq!(a.used(), 0);
    assert_eq!(a.capacity(), 1024);
    assert_eq!(a.remaining(), 1024);
}

#[test]
fn test_arena_alloc_returns_valid_non_null_pointer() {
    let a = Arena::new(4096);
    let p = a.alloc(64, 8).expect("allocation must succeed");
    assert!(!p.as_ptr().is_null());
}

#[test]
fn test_arena_used_grows_after_each_alloc() {
    let a = Arena::new(4096);
    a.alloc(100, 1).unwrap();
    assert!(a.used() >= 100);
    let used_after_first = a.used();
    a.alloc(200, 1).unwrap();
    assert!(a.used() >= used_after_first + 200);
}

// Each pointer satisfies the requested alignment.
#[test]
fn test_arena_alignment_guarantees() {
    let a = Arena::new(4096);
    for align in [1usize, 2, 4, 8, 16, 32, 64] {
        let p = a.alloc(1, align).expect("alloc should succeed");
        assert_eq!(p.as_ptr() as usize % align, 0,
            "pointer must be {align}-byte aligned");
    }
}

// Two allocations must not overlap — their addresses must differ.
#[test]
fn test_arena_allocations_do_not_alias() {
    let a = Arena::new(4096);
    let p1 = a.alloc(64, 8).unwrap();
    let p2 = a.alloc(64, 8).unwrap();
    assert_ne!(p1.as_ptr(), p2.as_ptr());
}

// alloc returns None when the remaining space is exhausted.
#[test]
fn test_arena_returns_none_when_full() {
    let a = Arena::new(32);
    a.alloc(32, 1).unwrap();
    assert_eq!(a.alloc(1, 1), None);
}

// reset() restores full capacity and allows fresh allocations.
#[test]
fn test_arena_reset_restores_full_capacity() {
    let a = Arena::new(256);
    a.alloc(200, 1).unwrap();
    assert!(a.used() >= 200);
    a.reset();
    assert_eq!(a.used(), 0);
    assert_eq!(a.remaining(), 256);
    // Can now fill again.
    let p = a.alloc(256, 1);
    assert!(p.is_some(), "full capacity must be available after reset");
}

// Simulate a bump-allocation workload: allocate many small objects, reset, repeat.
#[test]
fn test_arena_bump_workload() {
    let a = Arena::new(4096);
    for round in 0..3 {
        let mut pointers = Vec::new();
        for _ in 0..16 {
            let p = a.alloc(16, 8).expect(&format!("alloc failed in round {round}"));
            pointers.push(p);
        }
        // All pointers from this round must be distinct.
        let addrs: Vec<_> = pointers.iter().map(|p| p.as_ptr() as usize).collect();
        let unique: std::collections::HashSet<_> = addrs.iter().collect();
        assert_eq!(unique.len(), pointers.len(), "pointers must not alias");
        a.reset();
    }
}

// ── PoolAllocator ─────────────────────────────────────────────────────────────

#[test]
fn test_pool_starts_fully_available() {
    let p = PoolAllocator::new(64, 8);
    assert_eq!(p.available(), 8);
    assert_eq!(p.allocated(), 0);
    assert_eq!(p.capacity(), 8);
}

#[test]
fn test_pool_alloc_and_free_roundtrip() {
    let mut pool = PoolAllocator::new(64, 4);
    let ptr = pool.alloc().expect("alloc must succeed");
    assert_eq!(pool.allocated(), 1);
    assert_eq!(pool.available(), 3);
    pool.free(ptr);
    assert_eq!(pool.allocated(), 0);
    assert_eq!(pool.available(), 4);
}

#[test]
fn test_pool_returns_none_when_exhausted() {
    let mut pool = PoolAllocator::new(64, 2);
    let _p1 = pool.alloc().unwrap();
    let _p2 = pool.alloc().unwrap();
    assert_eq!(pool.alloc(), None);
}

#[test]
fn test_pool_blocks_are_distinct() {
    let mut pool = PoolAllocator::new(64, 4);
    let p1 = pool.alloc().unwrap();
    let p2 = pool.alloc().unwrap();
    assert_ne!(p1.as_ptr(), p2.as_ptr());
}

// Freed block can be immediately re-allocated.
#[test]
fn test_pool_block_reuse() {
    let mut pool = PoolAllocator::new(64, 1);
    let p = pool.alloc().unwrap();
    pool.free(p);
    assert!(pool.alloc().is_some(), "freed block must be reusable");
}

// capacity() is invariant across alloc/free operations.
#[test]
fn test_pool_capacity_is_invariant() {
    let mut pool = PoolAllocator::new(64, 5);
    assert_eq!(pool.capacity(), 5);
    let p = pool.alloc().unwrap();
    assert_eq!(pool.capacity(), 5);
    pool.free(p);
    assert_eq!(pool.capacity(), 5);
}

// block_size is at least the size of a raw pointer (needed for free-list).
#[test]
fn test_pool_block_size_at_least_pointer_size() {
    let pool = PoolAllocator::new(1, 4); // requested 1 byte; must be clamped up
    assert!(pool.block_size() >= std::mem::size_of::<usize>());
}

// Simulate a pool workload: allocate all, free all, allocate all again.
#[test]
fn test_pool_full_cycle_workload() {
    let mut pool = PoolAllocator::new(64, 8);
    let mut ptrs = Vec::new();
    for _ in 0..8 {
        ptrs.push(pool.alloc().expect("alloc must succeed"));
    }
    assert_eq!(pool.available(), 0);
    for p in ptrs { pool.free(p); }
    assert_eq!(pool.available(), 8);
    // Can fill again.
    for _ in 0..8 {
        pool.alloc().expect("re-alloc must succeed");
    }
    assert_eq!(pool.available(), 0);
}

// ── AllocStats ────────────────────────────────────────────────────────────────

#[test]
fn test_stats_new_is_all_zero() {
    let s = AllocStats::new();
    assert_eq!(s.total_allocations,   0);
    assert_eq!(s.total_deallocations, 0);
    assert_eq!(s.bytes_allocated,     0);
    assert_eq!(s.bytes_freed,         0);
    assert_eq!(s.peak_usage,          0);
    assert_eq!(s.current_usage,       0);
    assert!(s.is_balanced());
}

#[test]
fn test_stats_record_alloc_and_free_cycle() {
    let mut s = AllocStats::new();
    s.record_alloc(512);
    assert_eq!(s.total_allocations, 1);
    assert_eq!(s.current_usage,     512);
    assert_eq!(s.peak_usage,        512);

    s.record_free(512);
    assert_eq!(s.total_deallocations, 1);
    assert_eq!(s.current_usage,       0);
    assert_eq!(s.peak_usage,          512); // peak must not drop
    assert!(s.is_balanced());
}

#[test]
fn test_stats_peak_usage_is_high_water_mark() {
    let mut s = AllocStats::new();
    s.record_alloc(100);
    s.record_alloc(200); // peak = 300
    s.record_free(150);  // current = 150, peak stays 300
    s.record_alloc(50);  // current = 200, peak stays 300
    assert_eq!(s.peak_usage, 300);
}
