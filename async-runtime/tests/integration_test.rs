// ─── async-runtime: integration tests ────────────────────────────────────────
//
// These tests use only the public API exported from lib.rs.
// They verify whole-system behaviour that crosses module boundaries.

use async_runtime::{ready, yield_now, Executor};
use std::sync::{Arc, Mutex};

// ── Basic executor lifecycle ──────────────────────────────────────────────────

// An executor with no tasks completes immediately without panicking.
#[test]
fn test_empty_executor_runs_cleanly() {
    let mut exec = Executor::new();
    exec.run();
}

// ── Single-task execution ─────────────────────────────────────────────────────

// A task built from `ready` resolves in one poll.
#[test]
fn test_ready_task_completes() {
    let flag = Arc::new(Mutex::new(false));
    let flag2 = flag.clone();

    let mut exec = Executor::new();
    exec.spawn(async move {
        let v = ready(true).await;
        *flag2.lock().unwrap() = v;
    });
    exec.run();

    assert!(*flag.lock().unwrap(), "task must have written true");
}

// ── Yield-based cooperative multitasking ─────────────────────────────────────

// A task that calls yield_now() still runs to completion.
#[test]
fn test_yielding_task_runs_to_completion() {
    let log = Arc::new(Mutex::new(Vec::<u32>::new()));
    let l = log.clone();

    let mut exec = Executor::new();
    exec.spawn(async move {
        l.lock().unwrap().push(1);
        yield_now().await;
        l.lock().unwrap().push(2);
    });
    exec.run();

    let result = log.lock().unwrap().clone();
    assert_eq!(result, vec![1, 2]);
}

// ── Multi-task interleaving ───────────────────────────────────────────────────

// Two tasks that yield interleave correctly: the second task runs while the
// first is suspended.
#[test]
fn test_two_tasks_interleave_via_yield() {
    let log: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let la = log.clone();
    let lb = log.clone();

    let mut exec = Executor::new();

    exec.spawn(async move {
        la.lock().unwrap().push("A:start");
        yield_now().await;
        la.lock().unwrap().push("A:end");
    });

    exec.spawn(async move {
        lb.lock().unwrap().push("B:run");
    });

    exec.run();

    let result = log.lock().unwrap().clone();
    // All three steps must be recorded.
    assert!(result.contains(&"A:start"));
    assert!(result.contains(&"A:end"));
    assert!(result.contains(&"B:run"));

    // B must run after A yields (A:start precedes B:run, B:run precedes A:end).
    let pos = |s: &'static str| result.iter().position(|&x| x == s).unwrap();
    assert!(pos("A:start") < pos("B:run"),  "B should run after A yields");
    assert!(pos("B:run")   < pos("A:end"),  "B should complete before A resumes");
}

// ── N-task fan-out ────────────────────────────────────────────────────────────

// Spawning many tasks, each writing to a shared counter, all complete.
#[test]
fn test_many_tasks_all_complete() {
    const N: u32 = 20;
    let counter = Arc::new(Mutex::new(0u32));
    let mut exec = Executor::new();

    for _ in 0..N {
        let c = counter.clone();
        exec.spawn(async move {
            yield_now().await; // ensures tasks must be re-scheduled
            *c.lock().unwrap() += 1;
        });
    }

    exec.run();
    assert_eq!(*counter.lock().unwrap(), N);
}

// ── Chained async operations ──────────────────────────────────────────────────

// A task that awaits several ready() values in sequence records them all.
#[test]
fn test_chained_ready_futures() {
    let log = Arc::new(Mutex::new(Vec::<i32>::new()));
    let l = log.clone();

    let mut exec = Executor::new();
    exec.spawn(async move {
        let a = ready(1).await;
        let b = ready(2).await;
        let c = ready(3).await;
        l.lock().unwrap().extend([a, b, c]);
    });
    exec.run();

    assert_eq!(*log.lock().unwrap(), vec![1, 2, 3]);
}

// ── Executor can be reused ────────────────────────────────────────────────────

// After run() drains all tasks the executor can accept new tasks and run again.
#[test]
fn test_executor_is_reusable() {
    let mut exec = Executor::new();

    let first = Arc::new(Mutex::new(false));
    let f2 = first.clone();
    exec.spawn(async move { *f2.lock().unwrap() = true; });
    exec.run();

    let second = Arc::new(Mutex::new(false));
    let s2 = second.clone();
    exec.spawn(async move { *s2.lock().unwrap() = true; });
    exec.run();

    assert!(*first.lock().unwrap());
    assert!(*second.lock().unwrap());
}
