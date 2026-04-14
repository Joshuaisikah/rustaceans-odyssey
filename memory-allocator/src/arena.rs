use std::ptr::NonNull;

// ─── Arena (Bump Allocator) ───────────────────────────────────────────────────
//
// Bump allocator: allocation is O(1); deallocation is all-at-once via reset().
//
// Design hints:
//   - In new(), call std::alloc::alloc with a Layout of `capacity` bytes and
//     alignment 1 (or the system's max align for safety).
//   - Store the base pointer, capacity, and a cursor (UnsafeCell<usize>).
//   - alloc(size, align): round the cursor up to `align`, write the aligned
//     position as the returned pointer, then advance the cursor by `size`.
//   - reset(): set the cursor back to 0 — does NOT free the backing buffer.
//   - Drop: call std::alloc::dealloc on the backing buffer.

pub struct Arena {
    // TODO: base: NonNull<u8>, capacity: usize, cursor: UnsafeCell<usize>
}

impl Arena {
    /// Allocate a backing buffer of `capacity` bytes.
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// Bump-allocate `size` bytes with the given `align`.
    /// Returns None when remaining space is insufficient.
    pub fn alloc(&self, size: usize, align: usize) -> Option<NonNull<u8>> {
        todo!()
    }

    /// Reset the cursor to zero.  All previously returned pointers become
    /// invalid — the caller must ensure they are not used afterward.
    pub fn reset(&self) {
        todo!()
    }

    /// Bytes consumed so far (cursor position, including alignment padding).
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
        todo!() // dealloc the backing buffer via std::alloc::dealloc
    }
}

// ─── Allocator impl ───────────────────────────────────────────────────────────
//
// Allows Arena to be used interchangeably with PoolAllocator behind the
// Allocator trait.  Delegate to the Arena's own methods.

impl crate::traits::Allocator for Arena {
    fn alloc(&self, size: usize, align: usize) -> Option<std::ptr::NonNull<u8>> {
        Arena::alloc(self, size, align)
    }

    fn reset(&self) {
        Arena::reset(self)
    }

    fn capacity(&self) -> usize {
        Arena::capacity(self)
    }

    fn used(&self) -> usize {
        Arena::used(self)
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // A new arena reports 0 bytes used.
    #[test]
    fn test_starts_empty() {
        let a = Arena::new(1024);
        assert_eq!(a.used(), 0);
        assert_eq!(a.capacity(), 1024);
        assert_eq!(a.remaining(), 1024);
    }

    // alloc() returns a non-null pointer.
    #[test]
    fn test_alloc_returns_valid_pointer() {
        let a = Arena::new(1024);
        let p = a.alloc(64, 8).expect("allocation failed");
        assert!(!p.as_ptr().is_null());
    }

    // used() grows by at least `size` after each allocation.
    #[test]
    fn test_used_grows_after_alloc() {
        let a = Arena::new(1024);
        a.alloc(64, 8).unwrap();
        assert!(a.used() >= 64);
    }

    // remaining() = capacity() - used().
    #[test]
    fn test_remaining_is_capacity_minus_used() {
        let a = Arena::new(1024);
        a.alloc(100, 1).unwrap();
        assert_eq!(a.remaining(), a.capacity() - a.used());
    }

    // Returned pointer satisfies the requested alignment.
    #[test]
    fn test_alloc_alignment() {
        let a = Arena::new(4096);
        for align in [1usize, 2, 4, 8, 16, 32, 64] {
            let p = a.alloc(1, align).unwrap();
            assert_eq!(p.as_ptr() as usize % align, 0,
                "pointer is not {align}-byte aligned");
        }
    }

    // alloc() returns None when there is no space left.
    #[test]
    fn test_alloc_returns_none_when_full() {
        let a = Arena::new(32);
        a.alloc(32, 1).unwrap();
        assert_eq!(a.alloc(1, 1), None, "must return None when full");
    }

    // reset() restores full capacity.
    #[test]
    fn test_reset_restores_capacity() {
        let a = Arena::new(256);
        a.alloc(128, 8).unwrap();
        a.reset();
        assert_eq!(a.used(), 0);
        assert_eq!(a.remaining(), 256);
    }

    // Two allocations from the same arena do not alias each other.
    #[test]
    fn test_allocations_do_not_alias() {
        let a = Arena::new(4096);
        let p1 = a.alloc(64, 8).unwrap();
        let p2 = a.alloc(64, 8).unwrap();
        assert_ne!(p1.as_ptr(), p2.as_ptr());
    }

    // After reset, the arena can serve fresh allocations up to full capacity.
    #[test]
    fn test_alloc_after_reset() {
        let a = Arena::new(128);
        a.alloc(100, 1).unwrap();
        a.reset();
        let p = a.alloc(128, 1);
        assert!(p.is_some(), "full capacity should be available after reset");
    }
}
