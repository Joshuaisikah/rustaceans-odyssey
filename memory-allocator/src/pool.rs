use std::ptr::NonNull;

// ─── Pool Allocator (Fixed-size block allocator) ──────────────────────────────
//
// All blocks are the same size → no fragmentation.
// O(1) alloc and free via an intrusive free-list embedded in unused blocks.
//
// Design hints:
//   - Allocate one contiguous buffer for `num_blocks` blocks of `block_size`
//     each (block size clamped to ≥ size_of::<usize>() so a pointer can be
//     stored in the free slot).
//   - The free-list is a linked list of raw pointers stored inside the free
//     blocks themselves.
//   - alloc(): pop from the free-list; None when list is empty.
//   - free(): push the pointer back onto the free-list.
//   - Drop: dealloc the backing buffer.

pub struct PoolAllocator {
    // TODO: backing buffer pointer, block_size, capacity, free_list head, counts
}

impl PoolAllocator {
    pub fn new(block_size: usize, num_blocks: usize) -> Self {
        todo!()
    }

    /// Pop a free block from the list.  Returns None when exhausted.
    pub fn alloc(&mut self) -> Option<NonNull<u8>> {
        todo!()
    }

    /// Return a block to the free-list.
    ///
    /// # Panics
    /// Panics if `ptr` is not inside the pool's backing buffer.
    pub fn free(&mut self, ptr: NonNull<u8>) {
        todo!()
    }

    /// Number of blocks currently in use (not on the free-list).
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

    /// The effective block size (≥ size_of::<usize>()).
    pub fn block_size(&self) -> usize {
        todo!()
    }
}

impl Drop for PoolAllocator {
    fn drop(&mut self) {
        todo!() // dealloc the backing buffer
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // A new pool has all blocks available.
    #[test]
    fn test_starts_fully_available() {
        let pool = PoolAllocator::new(64, 8);
        assert_eq!(pool.available(), 8);
        assert_eq!(pool.allocated(), 0);
        assert_eq!(pool.capacity(), 8);
    }

    // alloc decreases available; free restores it.
    #[test]
    fn test_alloc_and_free() {
        let mut pool = PoolAllocator::new(64, 4);
        let p = pool.alloc().expect("alloc failed");
        assert_eq!(pool.allocated(), 1);
        assert_eq!(pool.available(), 3);
        pool.free(p);
        assert_eq!(pool.allocated(), 0);
        assert_eq!(pool.available(), 4);
    }

    // alloc returns None when all blocks are taken.
    #[test]
    fn test_alloc_when_exhausted_returns_none() {
        let mut pool = PoolAllocator::new(32, 2);
        let _p1 = pool.alloc().unwrap();
        let _p2 = pool.alloc().unwrap();
        assert_eq!(pool.alloc(), None, "pool is exhausted");
    }

    // Blocks can be reused after being freed.
    #[test]
    fn test_reuse_after_free() {
        let mut pool = PoolAllocator::new(64, 1);
        let p = pool.alloc().unwrap();
        pool.free(p);
        assert!(pool.alloc().is_some(), "block must be reusable");
    }

    // block_size() is at least the size of a pointer.
    #[test]
    fn test_block_size_at_least_pointer_size() {
        let pool = PoolAllocator::new(1, 4);
        assert!(pool.block_size() >= std::mem::size_of::<usize>());
    }

    // Two allocated blocks have distinct addresses.
    #[test]
    fn test_allocated_blocks_are_distinct() {
        let mut pool = PoolAllocator::new(64, 4);
        let p1 = pool.alloc().unwrap();
        let p2 = pool.alloc().unwrap();
        assert_ne!(p1.as_ptr(), p2.as_ptr());
    }

    // capacity() never changes regardless of alloc/free calls.
    #[test]
    fn test_capacity_is_immutable() {
        let mut pool = PoolAllocator::new(64, 5);
        let p = pool.alloc().unwrap();
        assert_eq!(pool.capacity(), 5);
        pool.free(p);
        assert_eq!(pool.capacity(), 5);
    }
}
