// ─── SPSC Queue ──────────────────────────────────────────────────────────────
//
// Single-Producer Single-Consumer bounded lock-free queue.
//
// Design hints:
//   - Use a power-of-two ring buffer backed by an array (or Vec) of
//     MaybeUninit<T> so you can write/read without initialising the whole buffer.
//   - Store two AtomicUsize indices: `head` (consumer) and `tail` (producer).
//   - push()  writes at tail, increments tail.
//   - pop()   reads at head, increments head.
//   - Full  when (tail - head) == N - 1.  (one slot sentinel)
//   - Empty when tail == head.
//   - Use Acquire/Release ordering for the index loads/stores.
//   - Manually implement Send + Sync (unsafe impl) since the compiler cannot
//     infer them for the raw array.

pub struct SpscQueue<T, const N: usize> {
    // TODO: add fields
    _p: std::marker::PhantomData<T>,
}

unsafe impl<T: Send, const N: usize> Send for SpscQueue<T, N> {}
unsafe impl<T: Send, const N: usize> Sync for SpscQueue<T, N> {}

impl<T, const N: usize> SpscQueue<T, N> {
    pub fn new() -> Self {
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

    /// True when there are no items in the queue.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Current number of items in the queue.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Maximum items that can be stored at once (N - 1).
    pub fn capacity(&self) -> usize {
        N - 1
    }
}

impl<T, const N: usize> Default for SpscQueue<T, N> {
    fn default() -> Self { Self::new() }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // A fresh queue is empty and has zero length.
    #[test]
    fn test_new_is_empty() {
        let q = SpscQueue::<i32, 8>::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    // Push one value then pop it back.
    #[test]
    fn test_push_and_pop_roundtrip() {
        let q = SpscQueue::<i32, 8>::new();
        assert!(q.push(42));
        assert_eq!(q.pop(), Some(42));
    }

    // Pop on an empty queue returns None.
    #[test]
    fn test_pop_empty_returns_none() {
        let q = SpscQueue::<i32, 4>::new();
        assert_eq!(q.pop(), None);
    }

    // Items come out in FIFO order.
    #[test]
    fn test_fifo_order() {
        let q = SpscQueue::<i32, 8>::new();
        q.push(1); q.push(2); q.push(3);
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), Some(3));
    }

    // Push returns false when the queue is full (capacity = N-1).
    #[test]
    fn test_push_when_full_returns_false() {
        let q = SpscQueue::<i32, 4>::new(); // capacity = 3
        assert!(q.push(1));
        assert!(q.push(2));
        assert!(q.push(3));
        assert!(!q.push(4), "queue is full; push must return false");
    }

    // len() tracks item count through push/pop cycles.
    #[test]
    fn test_len_tracking() {
        let q = SpscQueue::<i32, 8>::new();
        assert_eq!(q.len(), 0);
        q.push(10); assert_eq!(q.len(), 1);
        q.push(20); assert_eq!(q.len(), 2);
        q.pop();    assert_eq!(q.len(), 1);
        q.pop();    assert_eq!(q.len(), 0);
    }

    // capacity() is always N - 1.
    #[test]
    fn test_capacity_is_n_minus_one() {
        let q = SpscQueue::<i32, 16>::new();
        assert_eq!(q.capacity(), 15);
    }

    // A pop after the queue wraps around still returns the correct value.
    #[test]
    fn test_wrap_around() {
        let q = SpscQueue::<i32, 4>::new(); // capacity = 3
        q.push(1); q.push(2); q.push(3);
        q.pop(); q.pop(); // consume two — free two slots
        q.push(4); q.push(5); // wrap
        assert_eq!(q.pop(), Some(3));
        assert_eq!(q.pop(), Some(4));
        assert_eq!(q.pop(), Some(5));
    }

    // Concurrent producer/consumer: producer sends 0..100, consumer receives all.
    #[test]
    fn test_concurrent_producer_consumer() {
        let q   = Arc::new(SpscQueue::<i32, 128>::new());
        let qp  = q.clone();
        let qc  = q.clone();

        let producer = thread::spawn(move || {
            for i in 0..100i32 {
                while !qp.push(i) { std::hint::spin_loop(); }
            }
        });

        let consumer = thread::spawn(move || {
            let mut out = Vec::with_capacity(100);
            while out.len() < 100 {
                match qc.pop() {
                    Some(v) => out.push(v),
                    None    => std::hint::spin_loop(),
                }
            }
            out
        });

        producer.join().unwrap();
        let result = consumer.join().unwrap();
        assert_eq!(result, (0..100).collect::<Vec<_>>());
    }
}
