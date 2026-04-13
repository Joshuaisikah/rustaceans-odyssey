// ─── SPSC Queue ──────────────────────────────────────────────────────────────
//
// Single-Producer Single-Consumer bounded queue.
// Hint: use a power-of-two ring buffer with two atomic indices (head / tail).
// Enforce Send + Sync manually with appropriate unsafe impl blocks.

pub struct SpscQueue<T, const N: usize> {
    // TODO: add your fields
    _p: std::marker::PhantomData<T>,
}

unsafe impl<T: Send, const N: usize> Send for SpscQueue<T, N> {}
unsafe impl<T: Send, const N: usize> Sync for SpscQueue<T, N> {}

impl<T, const N: usize> SpscQueue<T, N> {
    pub fn new() -> Self {
        todo!()
    }

    /// Try to push a value.  Returns `false` if the queue is full.
    pub fn push(&self, val: T) -> bool {
        todo!()
    }

    /// Try to pop a value.  Returns `None` if the queue is empty.
    pub fn pop(&self) -> Option<T> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Number of items currently in the queue.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Maximum number of items that can be stored simultaneously (N - 1).
    pub fn capacity(&self) -> usize {
        todo!()
    }
}

// ─── MPMC Queue ──────────────────────────────────────────────────────────────
//
// Multi-Producer Multi-Consumer bounded queue.
// A Mutex-backed ring is acceptable for correctness; feel free to implement
// a CAS-based version for extra credit.

pub struct MpmcQueue<T> {
    // TODO: add your fields
    _p: std::marker::PhantomData<T>,
}

impl<T> MpmcQueue<T> {
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// Try to push a value.  Returns `false` if the queue is full.
    pub fn push(&self, val: T) -> bool {
        todo!()
    }

    /// Try to pop a value.  Returns `None` if the queue is empty.
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
    use std::sync::Arc;
    use std::thread;

    // ── SpscQueue ────────────────────────────────────────────────────────────

    // A freshly created queue is empty.
    #[test]
    fn test_spsc_new_is_empty() {
        let q = SpscQueue::<i32, 8>::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    // Push and pop round-trip single values.
    #[test]
    fn test_spsc_push_and_pop() {
        let q = SpscQueue::<i32, 8>::new();
        assert!(q.push(42));
        assert_eq!(q.pop(), Some(42));
    }

    // FIFO order is preserved.
    #[test]
    fn test_spsc_fifo_order() {
        let q = SpscQueue::<i32, 8>::new();
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), Some(3));
    }

    // Pop on an empty queue returns None.
    #[test]
    fn test_spsc_pop_empty_returns_none() {
        let q = SpscQueue::<i32, 4>::new();
        assert_eq!(q.pop(), None);
    }

    // Push returns false when the queue is full.
    #[test]
    fn test_spsc_push_when_full_returns_false() {
        let q = SpscQueue::<i32, 4>::new(); // capacity = N-1 = 3
        assert!(q.push(1));
        assert!(q.push(2));
        assert!(q.push(3));
        assert!(!q.push(4), "queue is full; push must return false");
    }

    // len tracks the number of elements.
    #[test]
    fn test_spsc_len_tracking() {
        let q = SpscQueue::<i32, 8>::new();
        assert_eq!(q.len(), 0);
        q.push(1);
        assert_eq!(q.len(), 1);
        q.push(2);
        assert_eq!(q.len(), 2);
        q.pop();
        assert_eq!(q.len(), 1);
    }

    // capacity() is one less than N (one slot kept empty to distinguish
    // full from empty).
    #[test]
    fn test_spsc_capacity() {
        let q = SpscQueue::<i32, 16>::new();
        assert_eq!(q.capacity(), 15);
    }

    // Concurrent producer / consumer: producer sends 0..100, consumer
    // receives them all in order.
    #[test]
    fn test_spsc_concurrent_producer_consumer() {
        let q = Arc::new(SpscQueue::<i32, 128>::new());
        let qp = q.clone();
        let qc = q.clone();

        let producer = thread::spawn(move || {
            for i in 0..100 {
                while !qp.push(i) {
                    std::hint::spin_loop();
                }
            }
        });

        let consumer = thread::spawn(move || {
            let mut received = Vec::with_capacity(100);
            while received.len() < 100 {
                if let Some(v) = qc.pop() {
                    received.push(v);
                } else {
                    std::hint::spin_loop();
                }
            }
            received
        });

        producer.join().unwrap();
        let result = consumer.join().unwrap();
        assert_eq!(result, (0..100).collect::<Vec<_>>());
    }

    // ── MpmcQueue ────────────────────────────────────────────────────────────

    // Basic push / pop.
    #[test]
    fn test_mpmc_push_and_pop() {
        let q = MpmcQueue::new(8);
        assert!(q.push(1));
        assert!(q.push(2));
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
    }

    // Push beyond capacity returns false.
    #[test]
    fn test_mpmc_full_returns_false() {
        let q = MpmcQueue::new(2);
        assert!(q.push("a"));
        assert!(q.push("b"));
        assert!(!q.push("c"), "full queue must reject push");
    }

    // Pop on an empty queue returns None.
    #[test]
    fn test_mpmc_pop_empty_returns_none() {
        let q = MpmcQueue::<i32>::new(8);
        assert_eq!(q.pop(), None);
    }

    // capacity() reflects the value passed to new().
    #[test]
    fn test_mpmc_capacity() {
        let q = MpmcQueue::<i32>::new(32);
        assert_eq!(q.capacity(), 32);
    }

    // Multiple threads can push and pop concurrently.
    // The test only verifies total item counts, not ordering.
    #[test]
    fn test_mpmc_concurrent_access() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let q = Arc::new(MpmcQueue::new(1024));
        let consumed = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        // 4 producers × 25 items each = 100 total
        for p in 0..4u32 {
            let qp = q.clone();
            handles.push(thread::spawn(move || {
                for i in 0..25u32 {
                    let val = p * 25 + i;
                    while !qp.push(val) {
                        std::hint::spin_loop();
                    }
                }
            }));
        }

        // 2 consumers, each drain until they've seen 50 items
        for _ in 0..2 {
            let qc = q.clone();
            let cnt = consumed.clone();
            handles.push(thread::spawn(move || {
                let mut local = 0;
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

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(consumed.load(Ordering::Relaxed), 100);
    }
}
