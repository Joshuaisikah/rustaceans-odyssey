// ─── MPMC Queue ──────────────────────────────────────────────────────────────
//
// Multi-Producer Multi-Consumer bounded queue.
//
// A Mutex-backed ring buffer is acceptable for correctness.
// For extra credit: implement a CAS-based (compare-and-swap) version using
// AtomicUsize sequence numbers per slot (like the Dmitry Vyukov MPMC queue).

pub struct MpmcQueue<T> {
    // TODO: add fields
    _p: std::marker::PhantomData<T>,
}

impl<T> MpmcQueue<T> {
    /// Create a queue with the given capacity.
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// Try to push a value.  Returns false if the queue is full.
    pub fn push(&self, val: T) -> bool {
        todo!()
    }

    /// Try to pop a value.  Returns None if the queue is empty.
    pub fn pop(&self) -> Option<T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
    use std::thread;

    // Basic push/pop round-trip.
    #[test]
    fn test_push_and_pop() {
        let q = MpmcQueue::new(8);
        assert!(q.push(1));
        assert!(q.push(2));
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
    }

    // Push beyond capacity returns false.
    #[test]
    fn test_full_queue_returns_false() {
        let q = MpmcQueue::new(2);
        assert!(q.push("a"));
        assert!(q.push("b"));
        assert!(!q.push("c"), "full queue must reject push");
    }

    // Pop on an empty queue returns None.
    #[test]
    fn test_pop_empty_returns_none() {
        let q = MpmcQueue::<i32>::new(8);
        assert_eq!(q.pop(), None);
    }

    // capacity() reflects the value passed to new().
    #[test]
    fn test_capacity_reflects_new() {
        let q = MpmcQueue::<i32>::new(32);
        assert_eq!(q.capacity(), 32);
    }

    // is_empty is true on a fresh queue.
    #[test]
    fn test_new_queue_is_empty() {
        let q = MpmcQueue::<i32>::new(4);
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    // len tracks correctly through push/pop.
    #[test]
    fn test_len_tracking() {
        let q = MpmcQueue::new(8);
        q.push(1); assert_eq!(q.len(), 1);
        q.push(2); assert_eq!(q.len(), 2);
        q.pop();   assert_eq!(q.len(), 1);
        q.pop();   assert_eq!(q.len(), 0);
        assert!(q.is_empty());
    }

    // After freeing slots, new items can be pushed.
    #[test]
    fn test_reuse_after_pop() {
        let q = MpmcQueue::new(2);
        q.push(1); q.push(2);
        q.pop(); // free one slot
        assert!(q.push(3), "slot must be reusable after pop");
    }

    // 4 producers × 25 items each, 2 consumers — all 100 items must be consumed.
    #[test]
    fn test_concurrent_mpmc() {
        let q        = Arc::new(MpmcQueue::new(1024));
        let consumed = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for p in 0..4u32 {
            let qp = q.clone();
            handles.push(thread::spawn(move || {
                for i in 0..25u32 {
                    let val = p * 25 + i;
                    while !qp.push(val) { std::hint::spin_loop(); }
                }
            }));
        }

        for _ in 0..2 {
            let qc  = q.clone();
            let cnt = consumed.clone();
            handles.push(thread::spawn(move || {
                let mut local = 0usize;
                while local < 50 {
                    if qc.pop().is_some() {
                        local += 1;
                        cnt.fetch_add(1, Ordering::Relaxed);
                    } else {
                        std::hint::spin_loop();
                    }
                }
            }));
        }

        for h in handles { h.join().unwrap(); }
        assert_eq!(consumed.load(Ordering::Relaxed), 100);
    }
}
