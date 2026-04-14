// ─── lock-free-queue: integration tests ──────────────────────────────────────
//
// Black-box tests through the public API.
// Exercises both SpscQueue and MpmcQueue in isolation and under concurrency.

use lock_free_queue::{MpmcQueue, SpscQueue};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::thread;

// ── SpscQueue ─────────────────────────────────────────────────────────────────

#[test]
fn test_spsc_new_is_empty() {
    let q = SpscQueue::<i32, 8>::new();
    assert!(q.is_empty());
    assert_eq!(q.len(), 0);
    assert_eq!(q.capacity(), 7); // N-1 sentinel
}

#[test]
fn test_spsc_push_pop_roundtrip() {
    let q = SpscQueue::<i32, 8>::new();
    assert!(q.push(42));
    assert_eq!(q.pop(), Some(42));
    assert!(q.is_empty());
}

#[test]
fn test_spsc_fifo_order() {
    let q = SpscQueue::<i32, 16>::new();
    for i in 0..10 { q.push(i); }
    for i in 0..10 { assert_eq!(q.pop(), Some(i)); }
}

#[test]
fn test_spsc_push_returns_false_when_full() {
    let q = SpscQueue::<i32, 4>::new(); // capacity = 3
    assert!(q.push(1));
    assert!(q.push(2));
    assert!(q.push(3));
    assert!(!q.push(4), "must reject when full");
}

#[test]
fn test_spsc_pop_empty_returns_none() {
    let q = SpscQueue::<i32, 4>::new();
    assert_eq!(q.pop(), None);
}

// Push 3, pop 1, push 2 more — verifies the ring wraps correctly.
#[test]
fn test_spsc_ring_wrap_around() {
    let q = SpscQueue::<i32, 4>::new(); // capacity = 3
    q.push(10); q.push(20); q.push(30);
    q.pop(); q.pop();           // consume two
    q.push(40); q.push(50);    // wrap
    assert_eq!(q.pop(), Some(30));
    assert_eq!(q.pop(), Some(40));
    assert_eq!(q.pop(), Some(50));
    assert!(q.is_empty());
}

// Producer sends 0..200; consumer receives all values in order.
#[test]
fn test_spsc_concurrent_producer_consumer() {
    let q  = Arc::new(SpscQueue::<i32, 256>::new());
    let qp = q.clone();
    let qc = q.clone();

    let producer = thread::spawn(move || {
        for i in 0..200i32 {
            while !qp.push(i) { std::hint::spin_loop(); }
        }
    });

    let consumer = thread::spawn(move || {
        let mut out = Vec::with_capacity(200);
        while out.len() < 200 {
            match qc.pop() {
                Some(v) => out.push(v),
                None    => std::hint::spin_loop(),
            }
        }
        out
    });

    producer.join().unwrap();
    let result = consumer.join().unwrap();
    assert_eq!(result, (0..200).collect::<Vec<_>>());
}

// ── MpmcQueue ─────────────────────────────────────────────────────────────────

#[test]
fn test_mpmc_new_is_empty() {
    let q = MpmcQueue::<i32>::new(16);
    assert!(q.is_empty());
    assert_eq!(q.len(), 0);
    assert_eq!(q.capacity(), 16);
}

#[test]
fn test_mpmc_push_pop_roundtrip() {
    let q = MpmcQueue::new(8);
    assert!(q.push(99i32));
    assert_eq!(q.pop(), Some(99));
}

#[test]
fn test_mpmc_fifo_order_single_thread() {
    let q = MpmcQueue::new(32);
    for i in 0..10 { q.push(i); }
    for i in 0..10 { assert_eq!(q.pop(), Some(i)); }
}

#[test]
fn test_mpmc_push_when_full_returns_false() {
    let q = MpmcQueue::new(2);
    q.push("a"); q.push("b");
    assert!(!q.push("c"), "full queue must reject");
}

// 4 producers × 50 items; 2 consumers; all 200 items must be received.
#[test]
fn test_mpmc_concurrent_producers_and_consumers() {
    const PRODUCERS: u32 = 4;
    const PER_PRODUCER: u32 = 50;
    const TOTAL: usize = (PRODUCERS * PER_PRODUCER) as usize;

    let q        = Arc::new(MpmcQueue::<u32>::new(1024));
    let consumed = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for p in 0..PRODUCERS {
        let qp = q.clone();
        handles.push(thread::spawn(move || {
            for i in 0..PER_PRODUCER {
                let val = p * PER_PRODUCER + i;
                while !qp.push(val) { std::hint::spin_loop(); }
            }
        }));
    }

    let consumers = 2usize;
    let per_consumer = TOTAL / consumers;
    for _ in 0..consumers {
        let qc  = q.clone();
        let cnt = consumed.clone();
        handles.push(thread::spawn(move || {
            let mut local = 0;
            while local < per_consumer {
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
    assert_eq!(consumed.load(Ordering::Relaxed), TOTAL);
}

// ── Interoperability: fill then drain ────────────────────────────────────────

// SPSC: fill to capacity, drain completely, fill again — verifies reuse.
#[test]
fn test_spsc_fill_drain_refill() {
    let q = SpscQueue::<i32, 8>::new();
    for i in 0..7 { assert!(q.push(i), "push {i} failed"); }
    assert!(!q.push(99), "at capacity");
    for i in 0..7 { assert_eq!(q.pop(), Some(i)); }
    assert!(q.is_empty());
    for i in 10..17 { assert!(q.push(i)); }
    assert_eq!(q.len(), 7);
}
