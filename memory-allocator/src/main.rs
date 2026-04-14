// ─── memory-allocator — integration demo ─────────────────────────────────────
//
// Run with:  cargo run -p memory-allocator

use memory_allocator::{AllocStats, Arena, PoolAllocator};

fn main() {
    println!("=== memory-allocator integration demo ===\n");

    demo_arena();
    demo_pool();
    demo_alloc_stats();

    println!("\nAll demos completed.");
}

// ── Demo 1: Arena bump allocator ─────────────────────────────────────────────
fn demo_arena() {
    println!("[ Demo 1 ] Arena bump allocator");

    let arena = Arena::new(4096);
    assert_eq!(arena.used(), 0);

    // Allocate several chunks with varying alignments.
    let p1 = arena.alloc(64, 8).expect("alloc 64B");
    let p2 = arena.alloc(128, 16).expect("alloc 128B");
    let p3 = arena.alloc(256, 32).expect("alloc 256B");

    assert_ne!(p1.as_ptr(), p2.as_ptr());
    assert_ne!(p2.as_ptr(), p3.as_ptr());
    assert!(arena.used() >= 448, "used must cover all three allocations");

    println!("  3 allocations, used={} / {}", arena.used(), arena.capacity());

    // Write through the pointer to prove it's valid memory.
    unsafe {
        p1.as_ptr().write(0xAA);
        assert_eq!(p1.as_ptr().read(), 0xAA);
    }
    println!("  pointer write/read verified  ✓");

    // Reset restores full capacity.
    arena.reset();
    assert_eq!(arena.used(), 0);
    assert_eq!(arena.remaining(), arena.capacity());
    println!("  reset restores capacity  ✓");

    // Exhaustion returns None.
    let _full = arena.alloc(4096, 1).expect("fill arena");
    assert_eq!(arena.alloc(1, 1), None, "must return None when full");
    println!("  exhaustion handled correctly  ✓");
}

// ── Demo 2: Pool fixed-size block allocator ───────────────────────────────────
fn demo_pool() {
    println!("[ Demo 2 ] Pool allocator");

    let mut pool = PoolAllocator::new(64, 4);
    assert_eq!(pool.capacity(), 4);
    assert_eq!(pool.available(), 4);

    let b1 = pool.alloc().expect("block 1");
    let b2 = pool.alloc().expect("block 2");
    let b3 = pool.alloc().expect("block 3");
    let b4 = pool.alloc().expect("block 4");

    assert_eq!(pool.available(), 0);
    assert_eq!(pool.alloc(), None, "must return None when exhausted");
    println!("  4/4 blocks allocated, exhaustion handled  ✓");

    // Free two and reallocate.
    pool.free(b1);
    pool.free(b3);
    assert_eq!(pool.available(), 2);

    let _r1 = pool.alloc().expect("realloc 1");
    let _r2 = pool.alloc().expect("realloc 2");
    assert_eq!(pool.available(), 0);
    println!("  blocks reused after free  ✓");

    // Keep b2 / b4 alive until end to avoid use-after-free.
    drop(b2); drop(b4);
}

// ── Demo 3: AllocStats ────────────────────────────────────────────────────────
fn demo_alloc_stats() {
    println!("[ Demo 3 ] AllocStats tracking");

    let mut stats = AllocStats::new();
    assert!(stats.is_balanced());

    stats.record_alloc(512);
    stats.record_alloc(256);
    assert_eq!(stats.total_allocations, 2);
    assert_eq!(stats.current_usage, 768);
    assert_eq!(stats.peak_usage, 768);

    stats.record_free(512);
    assert_eq!(stats.current_usage, 256);
    assert_eq!(stats.peak_usage, 768, "peak must not shrink");
    assert!(!stats.is_balanced());

    stats.record_free(256);
    assert!(stats.is_balanced());
    println!(
        "  allocs={} frees={} peak={}B  ✓",
        stats.total_allocations, stats.total_deallocations, stats.peak_usage
    );
}
