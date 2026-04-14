// ─── Process management ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct Pcb {
    pub pid:        u32,
    pub name:       String,
    pub state:      ProcessState,
    pub priority:   u8,
    pub time_slice: u32, // ticks allotted per round
    pub used_time:  u32, // ticks consumed this round
}

impl Pcb {
    pub fn new(pid: u32, name: &str, priority: u8, time_slice: u32) -> Self {
        Pcb {
            pid,
            name: name.to_string(),
            state: ProcessState::Ready,
            priority,
            time_slice,
            used_time: 0,
        }
    }
}

// ─── Round-robin Scheduler ───────────────────────────────────────────────────
//
// Design hints:
//   - `ready`:      VecDeque<Pcb>
//   - `running`:    Option<Pcb>
//   - `blocked`:    HashMap<u32, Pcb>   (pid → Pcb)
//   - `terminated`: Vec<u32>            (just the PIDs)
//   - `next_pid`:   u32 counter starting at 1
//
// tick() algorithm:
//   1. If a process is running: increment used_time.
//      If used_time >= time_slice: move it to the back of the ready queue,
//      clear the running slot, reset used_time = 0.
//   2. If no process is running: dequeue the front of the ready queue.
//   3. Return running PID (or None).

pub struct Scheduler {
    // TODO: ready, running, blocked, terminated, next_pid
}

impl Scheduler {
    pub fn new() -> Self {
        todo!()
    }

    /// Create and enqueue a new process.  Returns the assigned PID (≥ 1).
    pub fn spawn(&mut self, name: &str, priority: u8, time_slice: u32) -> u32 {
        todo!()
    }

    /// Advance the scheduler by one tick.
    /// Returns the PID of the currently running process, or None.
    pub fn tick(&mut self) -> Option<u32> {
        todo!()
    }

    /// Move the running process to the terminated list.
    pub fn terminate_current(&mut self) {
        todo!()
    }

    /// Move the running process to the blocked set.
    pub fn block_current(&mut self) {
        todo!()
    }

    /// Move a blocked process back to the ready queue.
    /// Returns true if the PID was found in the blocked set.
    pub fn unblock(&mut self, pid: u32) -> bool {
        todo!()
    }

    pub fn running_pid(&self) -> Option<u32> {
        todo!()
    }

    pub fn ready_count(&self) -> usize {
        todo!()
    }

    pub fn blocked_count(&self) -> usize {
        todo!()
    }

    pub fn terminated_count(&self) -> usize {
        todo!()
    }
}

impl Default for Scheduler {
    fn default() -> Self { Self::new() }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // A spawned process goes into the ready queue.
    #[test]
    fn test_spawn_adds_to_ready_queue() {
        let mut s = Scheduler::new();
        s.spawn("init", 1, 4);
        assert_eq!(s.ready_count(), 1);
        assert_eq!(s.running_pid(), None);
    }

    // PIDs are auto-incremented from 1.
    #[test]
    fn test_pids_increment() {
        let mut s = Scheduler::new();
        assert_eq!(s.spawn("a", 1, 4), 1);
        assert_eq!(s.spawn("b", 1, 4), 2);
        assert_eq!(s.spawn("c", 1, 4), 3);
    }

    // First tick picks up the process.
    #[test]
    fn test_first_tick_starts_process() {
        let mut s   = Scheduler::new();
        let pid     = s.spawn("proc", 1, 4);
        let running = s.tick();
        assert_eq!(running, Some(pid));
        assert_eq!(s.running_pid(), Some(pid));
        assert_eq!(s.ready_count(), 0);
    }

    // Process is preempted after its time slice is exhausted.
    #[test]
    fn test_preemption_after_time_slice() {
        let mut s   = Scheduler::new();
        let pid_a   = s.spawn("A", 1, 2);
        let pid_b   = s.spawn("B", 1, 2);

        assert_eq!(s.tick(), Some(pid_a)); // tick 1: A runs
        assert_eq!(s.tick(), Some(pid_a)); // tick 2: A uses full slice
        assert_eq!(s.tick(), Some(pid_b)); // tick 3: A preempted; B starts
    }

    // Round-robin: two processes with slice=1 alternate every tick.
    #[test]
    fn test_round_robin_alternates() {
        let mut s = Scheduler::new();
        let a     = s.spawn("A", 1, 1);
        let b     = s.spawn("B", 1, 1);

        assert_eq!(s.tick(), Some(a));
        assert_eq!(s.tick(), Some(b));
        assert_eq!(s.tick(), Some(a));
        assert_eq!(s.tick(), Some(b));
    }

    // tick() returns None when no processes exist.
    #[test]
    fn test_tick_with_no_processes_returns_none() {
        let mut s = Scheduler::new();
        assert_eq!(s.tick(), None);
    }

    // terminate_current removes the running process.
    #[test]
    fn test_terminate_current() {
        let mut s = Scheduler::new();
        s.spawn("proc", 1, 4);
        s.tick();
        s.terminate_current();
        assert_eq!(s.running_pid(), None);
        assert_eq!(s.terminated_count(), 1);
    }

    // block_current moves the process out; unblock re-queues it.
    #[test]
    fn test_block_and_unblock() {
        let mut s   = Scheduler::new();
        let pid     = s.spawn("io", 1, 4);
        s.tick();
        s.block_current();

        assert_eq!(s.running_pid(), None);
        assert_eq!(s.ready_count(), 0);
        assert_eq!(s.blocked_count(), 1);

        assert!(s.unblock(pid));
        assert_eq!(s.ready_count(), 1);
        assert_eq!(s.blocked_count(), 0);

        assert_eq!(s.tick(), Some(pid));
    }

    // unblock on an unknown PID returns false.
    #[test]
    fn test_unblock_unknown_pid_returns_false() {
        let mut s = Scheduler::new();
        assert!(!s.unblock(999));
    }

    // Three processes: first terminates, remaining two keep rotating.
    #[test]
    fn test_process_lifecycle() {
        let mut s  = Scheduler::new();
        let pid1   = s.spawn("short", 1, 1);
        let pid2   = s.spawn("long1", 1, 2);
        let pid3   = s.spawn("long2", 1, 2);

        // pid1 runs for 1 tick then gets preempted
        assert_eq!(s.tick(), Some(pid1));
        // pid1 is back at the end of the ready queue; pid2 starts
        assert_eq!(s.tick(), Some(pid2));

        // Terminate pid2 while it's running
        s.terminate_current();
        assert_eq!(s.terminated_count(), 1);

        // pid3 should now start
        let next = s.tick();
        assert!(next == Some(pid3) || next == Some(pid1));
    }
}
