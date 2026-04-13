use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// ─── Types you need to implement ─────────────────────────────────────────────

/// A minimal single-threaded async executor.
///
/// Spawned tasks are polled cooperatively.  When a task returns `Poll::Pending`
/// it is re-queued and polled again on the next iteration.  `run()` drives the
/// queue until every task has completed.
pub struct Executor {
    // TODO: add your fields
}

impl Executor {
    pub fn new() -> Self {
        todo!()
    }

    /// Queue a future for execution.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }

    /// Poll all tasks to completion.
    pub fn run(&mut self) {
        todo!()
    }
}

/// A future that resolves immediately with the provided value.
pub struct Ready<T>(/* TODO */std::marker::PhantomData<T>);

pub fn ready<T>(val: T) -> Ready<T> {
    let _ = val;
    todo!()
}

impl<T: Unpin> Future for Ready<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        todo!()
    }
}

/// A future that yields `Poll::Pending` exactly once, then resolves.
pub struct YieldNow(/* TODO */);

pub fn yield_now() -> YieldNow {
    todo!()
}

impl Future for YieldNow {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // A Ready future must resolve on the very first poll.
    #[test]
    fn test_ready_future_resolves_immediately() {
        use std::task::{RawWaker, RawWakerVTable, Waker};

        fn noop_clone(_: *const ()) -> RawWaker { noop_raw_waker() }
        fn noop_fn(_: *const ()) {}
        fn noop_raw_waker() -> RawWaker {
            static VTABLE: RawWakerVTable =
                RawWakerVTable::new(noop_clone, noop_fn, noop_fn, noop_fn);
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut cx = Context::from_waker(&waker);

        let mut f = Box::pin(ready(42i32));
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(v) => assert_eq!(v, 42),
            Poll::Pending => panic!("Ready should not return Pending"),
        }
    }

    // YieldNow must return Pending on the first poll and Ready on the second.
    #[test]
    fn test_yield_now_pends_once_then_resolves() {
        use std::task::{RawWaker, RawWakerVTable, Waker};

        fn noop_clone(_: *const ()) -> RawWaker { noop_raw_waker() }
        fn noop_fn(_: *const ()) {}
        fn noop_raw_waker() -> RawWaker {
            static VTABLE: RawWakerVTable =
                RawWakerVTable::new(noop_clone, noop_fn, noop_fn, noop_fn);
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut cx = Context::from_waker(&waker);

        let mut f = Box::pin(yield_now());
        assert_eq!(
            f.as_mut().poll(&mut cx),
            Poll::Pending,
            "first poll should be Pending"
        );
        assert_eq!(
            f.as_mut().poll(&mut cx),
            Poll::Ready(()),
            "second poll should be Ready"
        );
    }

    // The executor must drive a simple task to completion.
    #[test]
    fn test_executor_runs_single_task() {
        let log = Arc::new(Mutex::new(Vec::<&str>::new()));
        let log2 = log.clone();

        let mut exec = Executor::new();
        exec.spawn(async move {
            log2.lock().unwrap().push("done");
        });
        exec.run();

        assert_eq!(*log.lock().unwrap(), vec!["done"]);
    }

    // Multiple tasks must all complete.
    #[test]
    fn test_executor_runs_multiple_tasks() {
        let counter = Arc::new(Mutex::new(0u32));

        let mut exec = Executor::new();
        for _ in 0..5 {
            let c = counter.clone();
            exec.spawn(async move {
                *c.lock().unwrap() += 1;
            });
        }
        exec.run();

        assert_eq!(*counter.lock().unwrap(), 5);
    }

    // Tasks that yield should still complete after being re-polled.
    #[test]
    fn test_executor_completes_yielding_tasks() {
        let log = Arc::new(Mutex::new(Vec::<u32>::new()));
        let log2 = log.clone();
        let log3 = log.clone();

        let mut exec = Executor::new();

        exec.spawn(async move {
            log2.lock().unwrap().push(1);
            yield_now().await;
            log2.lock().unwrap().push(2);
        });

        exec.spawn(async move {
            log3.lock().unwrap().push(10);
        });

        exec.run();

        let result = log.lock().unwrap().clone();
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&10));
    }

    // The executor has nothing to do when no tasks are spawned.
    #[test]
    fn test_executor_run_with_no_tasks_is_noop() {
        let mut exec = Executor::new();
        exec.run(); // must not panic
    }
}
