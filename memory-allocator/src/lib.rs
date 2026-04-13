use std::ptr::NonNull;

// ─── Arena Allocator ─────────────────────────────────────────────────────────
//
// Bump allocator: allocation is O(1) pointer-bump; deallocation is all-at-once
// via reset().  Individual frees are not supported.
//
// Hint: allocate a backing buffer once in `new()` using `std::alloc::alloc`,
//       keep an offset cursor, and advance it on each `alloc()` call.
//       Use interior mutability (Cell / UnsafeCell) so that `alloc` can take
//       &self (convenient when handing out references that borrow the arena).

pub struct Arena {
    // TODO: add your fields
}

impl Arena {
    /// Allocate a backing buffer of `capacity` bytes.
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// Bump-allocate `size` bytes with the given `align`.
    /// Returns `None` when the remaining space is insufficient.
    pub fn alloc(&self, size: usize, align: usize) -> Option<NonNull<u8>> {
        todo!()
    }

    /// Reset the cursor to zero.  All previously returned pointers become
    /// invalid after this call.
    pub fn reset(&self) {
        todo!()
    }

    /// Bytes consumed so far (cursor position).
    pub fn used(&self) -> usize {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn remaining(&self) -> usize {
        todo!()
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        todo!() // free the backing buffer with std::alloc::dealloc
    }
}

// ─── Pool Allocator ──────────────────────────────────────────────────────────
//
// Fixed-size block allocator: O(1) alloc and free backed by a free-list.
// All blocks are the same size, which eliminates fragmentation.

pub struct PoolAllocator {
    // TODO: add your fields
}

impl PoolAllocator {
    /// Allocate a backing buffer for `num_blocks` blocks, each `block_size`
    /// bytes.  The actual block size is clamped to at least the size of a
    /// pointer so the free-list can be embedded in unused blocks.
    pub fn new(block_size: usize, num_blocks: usize) -> Self {
        todo!()
    }

    /// Pop a block off the free-list.  Returns `None` when exhausted.
    pub fn alloc(&mut self) -> Option<NonNull<u8>> {
        todo!()
    }

    /// Return a block to the free-list.
    ///
    /// # Panics
    /// Panics if `ptr` does not point to a block owned by this pool or if the
    /// pointer is misaligned.
    pub fn free(&mut self, ptr: NonNull<u8>) {
        todo!()
    }

    /// Number of blocks currently allocated (not in the free-list).
    pub fn allocated(&self) -> usize {
        todo!()
    }

    /// Number of blocks available for allocation.
    pub fn available(&self) -> usize {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn block_size(&self) -> usize {
        todo!()
    }
}

impl Drop for PoolAllocator {
    fn drop(&mut self) {
        todo!() // free backing buffer
    }
}

// ─── Allocation Stats ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AllocStats {
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub bytes_allocated: usize,
    pub bytes_freed: usize,
    pub peak_usage: usize,
    pub current_usage: usize,
}

impl AllocStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_alloc(&mut self, size: usize) {
        todo!()
    }

    pub fn record_free(&mut self, size: usize) {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Arena ────────────────────────────────────────────────────────────────

    // A new arena has zero bytes used.
    #[test]
    fn test_arena_starts_empty() {
        let a = Arena::new(1024);
        assert_eq!(a.used(), 0);
        assert_eq!(a.capacity(), 1024);
        assert_eq!(a.remaining(), 1024);
    }

    // alloc() returns a valid non-null pointer.
    #[test]
    fn test_arena_alloc_returns_valid_pointer() {
        let a = Arena::new(1024);
        let p = a.alloc(64, 8).expect("allocation failed");
        assert!(!p.as_ptr().is_null());
    }

    // used() grows by at least the requested size after each allocation.
    #[test]
    fn test_arena_used_grows_after_alloc() {
        let a = Arena::new(1024);
        a.alloc(64, 8).unwrap();
        assert!(a.used() >= 64);
    }

    // Returned pointer satisfies the requested alignment.
    #[test]
    fn test_arena_alloc_alignment() {
        let a = Arena::new(4096);
        for align in [1usize, 2, 4, 8, 16, 32, 64] {
            let p = a.alloc(1, align).unwrap();
            assert_eq!(
                p.as_ptr() as usize % align,
                0,
                "pointer is not {align}-byte aligned"
            );
        }
    }

    // alloc() returns None when out of space.
    #[test]
    fn test_arena_alloc_returns_none_when_full() {
        let a = Arena::new(32);
        a.alloc(32, 1).unwrap(); // exhaust all space
        assert_eq!(a.alloc(1, 1), None, "should return None when full");
    }

    // reset() makes the full capacity available again.
    #[test]
    fn test_arena_reset_restores_capacity() {
        let a = Arena::new(256);
        a.alloc(128, 8).unwrap();
        a.reset();
        assert_eq!(a.used(), 0);
        assert_eq!(a.remaining(), 256);
    }

    // Multiple allocations from a reset arena do not alias each other.
    #[test]
    fn test_arena_allocations_do_not_alias() {
        let a = Arena::new(4096);
        let p1 = a.alloc(64, 8).unwrap();
        let p2 = a.alloc(64, 8).unwrap();
        assert_ne!(p1.as_ptr(), p2.as_ptr());
    }

    // ── PoolAllocator ────────────────────────────────────────────────────────

    // A new pool has all blocks available.
    #[test]
    fn test_pool_starts_fully_available() {
        let pool = PoolAllocator::new(64, 8);
        assert_eq!(pool.available(), 8);
        assert_eq!(pool.allocated(), 0);
        assert_eq!(pool.capacity(), 8);
    }

    // alloc() decreases available count; free() restores it.
    #[test]
    fn test_pool_alloc_and_free() {
        let mut pool = PoolAllocator::new(64, 4);
        let p = pool.alloc().expect("alloc failed");
        assert_eq!(pool.allocated(), 1);
        assert_eq!(pool.available(), 3);
        pool.free(p);
        assert_eq!(pool.allocated(), 0);
        assert_eq!(pool.available(), 4);
    }

    // alloc() returns None when all blocks are taken.
    #[test]
    fn test_pool_alloc_when_full_returns_none() {
        let mut pool = PoolAllocator::new(32, 2);
        let _p1 = pool.alloc().unwrap();
        let _p2 = pool.alloc().unwrap();
        assert_eq!(pool.alloc(), None, "pool is exhausted");
    }

    // Blocks can be allocated and freed repeatedly.
    #[test]
    fn test_pool_reuse_after_free() {
        let mut pool = PoolAllocator::new(64, 1);
        let p1 = pool.alloc().unwrap();
        pool.free(p1);
        let p2 = pool.alloc();
        assert!(p2.is_some(), "block should be reusable after free");
    }

    // block_size() returns the actual block size (clamped to ≥ pointer size).
    #[test]
    fn test_pool_block_size_at_least_pointer_size() {
        let pool = PoolAllocator::new(1, 4); // request 1-byte blocks
        assert!(pool.block_size() >= std::mem::size_of::<usize>());
    }

    // ── AllocStats ───────────────────────────────────────────────────────────

    // record_alloc increments counters and tracks peak.
    #[test]
    fn test_alloc_stats_record_alloc() {
        let mut s = AllocStats::new();
        s.record_alloc(100);
        assert_eq!(s.total_allocations, 1);
        assert_eq!(s.bytes_allocated, 100);
        assert_eq!(s.current_usage, 100);
        assert_eq!(s.peak_usage, 100);
    }

    // record_free decrements current usage.
    #[test]
    fn test_alloc_stats_record_free() {
        let mut s = AllocStats::new();
        s.record_alloc(200);
        s.record_free(100);
        assert_eq!(s.total_deallocations, 1);
        assert_eq!(s.bytes_freed, 100);
        assert_eq!(s.current_usage, 100);
    }

    // peak_usage is the maximum current_usage ever observed.
    #[test]
    fn test_alloc_stats_peak_usage() {
        let mut s = AllocStats::new();
        s.record_alloc(500);
        s.record_free(300);
        s.record_alloc(100); // current = 300, peak stays 500
        assert_eq!(s.peak_usage, 500);
    }

    // current_usage never goes below zero (saturating subtraction).
    #[test]
    fn test_alloc_stats_no_underflow() {
        let mut s = AllocStats::new();
        s.record_alloc(10);
        s.record_free(50); // free more than allocated — should not wrap
        assert_eq!(s.current_usage, 0);
    }
}
