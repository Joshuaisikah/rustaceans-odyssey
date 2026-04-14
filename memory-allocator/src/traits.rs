use std::ptr::NonNull;

// ─── Allocator trait ──────────────────────────────────────────────────────────
//
// Common interface for Arena and PoolAllocator so they can be used
// interchangeably and tested against the same contract.

pub trait Allocator {
    /// Allocate `size` bytes with at least `align`-byte alignment.
    /// Returns None when space is exhausted.
    fn alloc(&self, size: usize, align: usize) -> Option<NonNull<u8>>;

    /// Release all allocations at once (semantics differ per implementation).
    fn reset(&self);

    fn capacity(&self)  -> usize;
    fn used(&self)      -> usize;
    fn remaining(&self) -> usize { self.capacity() - self.used() }
    fn is_full(&self)   -> bool  { self.remaining() == 0 }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::Arena;

    // remaining() is derived from capacity() and used().
    #[test]
    fn test_remaining_derivation() {
        let a = Arena::new(1024);
        // remaining() = capacity() - used()
        // Before any alloc: remaining == capacity
        assert_eq!(a.remaining(), a.capacity());
    }

    // is_full() is false on a fresh allocator.
    #[test]
    fn test_is_full_initially_false() {
        let a = Arena::new(1024);
        assert!(!a.is_full());
    }
}
