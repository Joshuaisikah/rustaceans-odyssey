// ─── async-runtime — integration demo ────────────────────────────────────────
//
// This file wires together the Executor + future primitives and shows the
// complete system working end-to-end.  Run with:
//
//   cargo run -p async-runtime
//
// Each section is a mini integration test you can expand.

use async_runtime::{Executor, ready, yield_now};
use std::sync::{Arc, Mutex};

fn main() {
    println!("=== async-runtime integration demo ===\n");

    demo_ready_future();
    demo_yield_now();
    demo_multiple_tasks();
    demo_interleaved_execution();

    println!("\nAll demos completed.");
}

// ── Demo 1: ready() ──────────────────────────────────────────────────────────
//
// A ready() future should resolve on the very first poll.
// The executor should run the task in a single pass.
fn demo_ready_future() {
    println!("[ Demo 1 ] ready() future");

    let result = Arc::new(Mutex::new(None::<i32>));
    let r = result.clone();

    let mut exec = Executor::new();
    exec.spawn(async move {
        let val = ready(42).await;
        *r.lock().unwrap() = Some(val);
    });
    exec.run();

    let val = result.lock().unwrap().unwrap();
    assert_eq!(val, 42, "ready(42) should produce 42");
    println!("  ready(42) produced: {val}  ✓");
}

// ── Demo 2: yield_now() ──────────────────────────────────────────────────────
//
// A yield_now() future should pend once then resolve.
// The task must complete, just after being re-queued once.
fn demo_yield_now() {
    println!("[ Demo 2 ] yield_now()");

    let steps = Arc::new(Mutex::new(Vec::<u32>::new()));
    let s = steps.clone();

    let mut exec = Executor::new();
    exec.spawn(async move {
        s.lock().unwrap().push(1); // before yield
        yield_now().await;
        s.lock().unwrap().push(2); // after yield
    });
    exec.run();

    let result = steps.lock().unwrap().clone();
    assert_eq!(result, vec![1, 2], "steps must be [1, 2] in order");
    println!("  execution steps: {result:?}  ✓");
}

// ── Demo 3: multiple independent tasks ───────────────────────────────────────
fn demo_multiple_tasks() {
    println!("[ Demo 3 ] multiple tasks");

    let counter = Arc::new(Mutex::new(0u32));
    let mut exec = Executor::new();

    for _ in 0..4 {
        let c = counter.clone();
        exec.spawn(async move {
            *c.lock().unwrap() += 1;
        });
    }
    exec.run();

    let count = *counter.lock().unwrap();
    assert_eq!(count, 4, "all 4 tasks must run");
    println!("  tasks completed: {count}  ✓");
}

// ── Demo 4: interleaved execution via yield ───────────────────────────────────
//
// Shows that when task A yields, task B gets CPU time before A resumes.
fn demo_interleaved_execution() {
    println!("[ Demo 4 ] interleaved execution");

    let log = Arc::new(Mutex::new(Vec::<&'static str>::new()));
    let la = log.clone();
    let lb = log.clone();

    let mut exec = Executor::new();

    exec.spawn(async move {
        la.lock().unwrap().push("A:start");
        yield_now().await;
        la.lock().unwrap().push("A:end");
    });

    exec.spawn(async move {
        lb.lock().unwrap().push("B:runs");
    });

    exec.run();

    let result = log.lock().unwrap().clone();
    println!("  execution order: {result:?}");

    // B should appear between A's two steps
    let pos_a_start = result.iter().position(|&s| s == "A:start").unwrap();
    let pos_b       = result.iter().position(|&s| s == "B:runs").unwrap();
    let pos_a_end   = result.iter().position(|&s| s == "A:end").unwrap();

    assert!(pos_a_start < pos_b,   "A must start before B runs");
    assert!(pos_b < pos_a_end,     "B must run before A resumes");
    println!("  interleaving verified  ✓");
}
