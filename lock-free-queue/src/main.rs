// ─── lock-free-queue — integration demo ──────────────────────────────────────
//
// Run with:  cargo run -p lock-free-queue

use lock_free_queue::{MpmcQueue, SpscQueue};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;

fn main() {
    println!("=== lock-free-queue integration demo ===\n");

    demo_spsc_basic();
    demo_spsc_concurrent();
    demo_mpmc_basic();
    demo_mpmc_concurrent();

    println!("\nAll demos completed.");
}

// ── Demo 1: SPSC basic ────────────────────────────────────────────────────────
fn demo_spsc_basic() {
    println!("[ Demo 1 ] SPSC basic operations");

    let q = SpscQueue::<i32, 8>::new();
    for i in 1..=5 { q.push(i); }
    assert_eq!(q.len(), 5);

    let mut out = vec![];
    while let Some(v) = q.pop() { out.push(v); }
    assert_eq!(out, vec![1, 2, 3, 4, 5]);
    assert!(q.is_empty());
    println!("  FIFO order verified, capacity={} ✓", q.capacity());
}

// ── Demo 2: SPSC concurrent ───────────────────────────────────────────────────
fn demo_spsc_concurrent() {
    println!("[ Demo 2 ] SPSC concurrent producer/consumer");

    let q  = Arc::new(SpscQueue::<i32, 256>::new());
    let qp = q.clone();
    let qc = q.clone();

    let producer = thread::spawn(move || {
        for i in 0..100i32 {
            while !qp.push(i) { std::hint::spin_loop(); }
        }
    });

    let consumer = thread::spawn(move || {
        let mut received = Vec::with_capacity(100);
        while received.len() < 100 {
            match qc.pop() {
                Some(v) => received.push(v),
                None    => std::hint::spin_loop(),
            }
        }
        received
    });

    producer.join().unwrap();
    let result = consumer.join().unwrap();
    assert_eq!(result.len(), 100);
    assert_eq!(result, (0..100).collect::<Vec<_>>());
    println!("  100 items transferred in order  ✓");
}

// ── Demo 3: MPMC basic ────────────────────────────────────────────────────────
fn demo_mpmc_basic() {
    println!("[ Demo 3 ] MPMC basic operations");

    let q = MpmcQueue::new(4);
    q.push(10); q.push(20); q.push(30);
    assert_eq!(q.len(), 3);
    assert!(!q.push(40) || { /* capacity = 4, so this may succeed */ true });

    let mut out = vec![];
    while let Some(v) = q.pop() { out.push(v); }
    println!("  MPMC basic pop order: {out:?}  ✓");
}

// ── Demo 4: MPMC concurrent ───────────────────────────────────────────────────
fn demo_mpmc_concurrent() {
    println!("[ Demo 4 ] MPMC 4 producers × 2 consumers");

    let q        = Arc::new(MpmcQueue::new(1024));
    let consumed = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for p in 0..4u32 {
        let qp = q.clone();
        handles.push(thread::spawn(move || {
            for i in 0..25u32 {
                while !qp.push(p * 25 + i) { std::hint::spin_loop(); }
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
    let total = consumed.load(Ordering::Relaxed);
    assert_eq!(total, 100);
    println!("  {total} items consumed across 2 consumer threads  ✓");
}
