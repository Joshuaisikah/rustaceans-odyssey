use std::future::Future;

// ─── Executor ────────────────────────────────────────────────────────────────
//
// A minimal single-threaded cooperative task executor.
//
// Design notes:
//   - Store tasks as Pin<Box<dyn Future<Output = ()> + Send>>.
//   - Use a VecDeque as the ready queue.
//   - On each iteration pop a task, poll it with a no-op waker, and push it
//     back only if it returned Poll::Pending.
//   - run() loops until the queue is empty.

pub struct Executor {
    // TODO: add your fields (e.g. VecDeque of boxed futures)
}

impl Executor {
    pub fn new() -> Self {
        todo!()
    }

    /// Queue a future for cooperative execution.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        todo!()
    }

    /// Drive all tasks to completion (cooperative, single-threaded).
    pub fn run(&mut self) {
        todo!()
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::future::yield_now;
    use std::sync::{Arc, Mutex};

    // Spawning nothing and calling run() must not panic.
    #[test]
    fn test_run_with_no_tasks_is_noop() {
        let mut exec = Executor::new();
        exec.run();
    }

    // A single task executes exactly once.
    #[test]
    fn test_single_task_runs_to_completion() {
        let log = Arc::new(Mutex::new(Vec::<&str>::new()));
        let log2 = log.clone();

        let mut exec = Executor::new();
        exec.spawn(async move {
            log2.lock().unwrap().push("done");
        });
        exec.run();

        assert_eq!(*log.lock().unwrap(), vec!["done"]);
    }

    // Multiple tasks all complete.
    #[test]
    fn test_multiple_tasks_all_complete() {
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

    // Tasks that yield once still complete after being re-polled.
    #[test]
    fn test_yielding_task_completes() {
        let log = Arc::new(Mutex::new(Vec::<u32>::new()));
        let log2 = log.clone();

        let mut exec = Executor::new();
        exec.spawn(async move {
            log2.lock().unwrap().push(1);
            yield_now().await;
            log2.lock().unwrap().push(2);
        });
        exec.run();

        let result = log.lock().unwrap().clone();
        assert!(result.contains(&1));
        assert!(result.contains(&2));
    }

    // When one task yields, other tasks get a chance to run before it resumes.
    #[test]
    fn test_interleaved_tasks_via_yield() {
        let log = Arc::new(Mutex::new(Vec::<u32>::new()));
        let log_a = log.clone();
        let log_b = log.clone();

        let mut exec = Executor::new();

        exec.spawn(async move {
            log_a.lock().unwrap().push(1);
            yield_now().await;
            log_a.lock().unwrap().push(3);
        });

        exec.spawn(async move {
            log_b.lock().unwrap().push(2);
        });

        exec.run();

        let result = log.lock().unwrap().clone();
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        // 2 should appear after 1 but before 3 (interleaving proof)
        let pos1 = result.iter().position(|&x| x == 1).unwrap();
        let pos2 = result.iter().position(|&x| x == 2).unwrap();
        let pos3 = result.iter().position(|&x| x == 3).unwrap();
        assert!(pos1 < pos3, "task A's steps must be in order");
        assert!(pos2 > pos1, "task B must run after A yields");
        assert!(pos2 < pos3, "task B must complete before A resumes");
    }

    // Spawning after run() must work if run() is called again.
    #[test]
    fn test_spawn_after_run() {
        let flag = Arc::new(Mutex::new(false));
        let flag2 = flag.clone();

        let mut exec = Executor::new();
        exec.run(); // first run with nothing

        exec.spawn(async move {
            *flag2.lock().unwrap() = true;
        });
        exec.run();

        assert!(*flag.lock().unwrap());
    }
}
