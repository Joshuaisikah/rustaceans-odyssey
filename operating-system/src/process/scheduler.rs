use super::pcb::{Pcb, ProcessState};

// ─── Scheduler — preemptive round-robin ───────────────────────────────────────
//
// Hint:
//   ready:      VecDeque<Pcb>
//   running:    Option<Pcb>
//   blocked:    HashMap<u32, Pcb>   (pid → Pcb)
//   terminated: Vec<u32>
//   next_pid:   u32  (auto-increment from 1)
//
// tick() rules:
//   1. If a process is running: increment used_time.
//      When used_time >= time_slice: preempt — reset used_time,
//      push to back of ready, clear running slot.
//   2. If no process is running: pop front of ready queue.
//   3. Return Some(running_pid) or None.

pub struct Scheduler {
    // TODO: ready, running, blocked, terminated, next_pid
}

impl Scheduler {
    pub fn new() -> Self { todo!() }

    /// Spawn a process and enqueue it.  Returns the new PID (≥ 1).
    pub fn spawn(&mut self, name: &str, priority: u8, time_slice: u32) -> u32 { todo!() }

    /// Advance one tick.  Returns the running PID or None.
    pub fn tick(&mut self) -> Option<u32> { todo!() }

    /// Terminate the currently running process.
    pub fn terminate_current(&mut self) { todo!() }

    /// Move the running process to the blocked set.
    pub fn block_current(&mut self) { todo!() }

    /// Move a blocked process back to the ready queue.
    /// Returns true if found.
    pub fn unblock(&mut self, pid: u32) -> bool { todo!() }

    pub fn running_pid(&self)      -> Option<u32> { todo!() }
    pub fn ready_count(&self)      -> usize        { todo!() }
    pub fn blocked_count(&self)    -> usize        { todo!() }
    pub fn terminated_count(&self) -> usize        { todo!() }
}

impl Default for Scheduler {
    fn default() -> Self { Self::new() }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_adds_to_ready_queue() {
        let mut s = Scheduler::new();
        s.spawn("init", 1, 4);
        assert_eq!(s.ready_count(), 1);
        assert_eq!(s.running_pid(), None);
    }

    #[test]
    fn test_pids_auto_increment_from_one() {
        let mut s = Scheduler::new();
        assert_eq!(s.spawn("a", 1, 4), 1);
        assert_eq!(s.spawn("b", 1, 4), 2);
    }

    #[test]
    fn test_first_tick_starts_process() {
        let mut s   = Scheduler::new();
        let pid     = s.spawn("proc", 1, 4);
        assert_eq!(s.tick(), Some(pid));
        assert_eq!(s.running_pid(), Some(pid));
    }

    #[test]
    fn test_preemption_after_time_slice() {
        let mut s = Scheduler::new();
        let a     = s.spawn("A", 1, 2);
        let b     = s.spawn("B", 1, 2);
        assert_eq!(s.tick(), Some(a));
        assert_eq!(s.tick(), Some(a));
        assert_eq!(s.tick(), Some(b)); // A preempted
    }

    #[test]
    fn test_round_robin_alternates() {
        let mut s = Scheduler::new();
        let a     = s.spawn("A", 1, 1);
        let b     = s.spawn("B", 1, 1);
        assert_eq!(s.tick(), Some(a));
        assert_eq!(s.tick(), Some(b));
        assert_eq!(s.tick(), Some(a));
    }

    #[test]
    fn test_tick_with_no_processes_returns_none() {
        let mut s = Scheduler::new();
        assert_eq!(s.tick(), None);
    }

    #[test]
    fn test_terminate_current() {
        let mut s = Scheduler::new();
        s.spawn("p", 1, 4);
        s.tick();
        s.terminate_current();
        assert_eq!(s.running_pid(), None);
        assert_eq!(s.terminated_count(), 1);
    }

    #[test]
    fn test_block_and_unblock() {
        let mut s   = Scheduler::new();
        let pid     = s.spawn("io", 1, 4);
        s.tick();
        s.block_current();
        assert_eq!(s.blocked_count(), 1);
        assert_eq!(s.running_pid(), None);
        assert!(s.unblock(pid));
        assert_eq!(s.blocked_count(), 0);
        assert_eq!(s.tick(), Some(pid));
    }

    #[test]
    fn test_unblock_unknown_pid_returns_false() {
        let mut s = Scheduler::new();
        assert!(!s.unblock(999));
    }
}
