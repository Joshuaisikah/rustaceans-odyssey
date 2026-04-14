// ─── Process Control Block ────────────────────────────────────────────────────

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
    pub time_slice: u32, // ticks allotted per scheduling round
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

    pub fn is_ready(&self)      -> bool { self.state == ProcessState::Ready }
    pub fn is_running(&self)    -> bool { self.state == ProcessState::Running }
    pub fn is_blocked(&self)    -> bool { self.state == ProcessState::Blocked }
    pub fn is_terminated(&self) -> bool { self.state == ProcessState::Terminated }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pcb_is_ready() {
        let p = Pcb::new(1, "init", 0, 4);
        assert_eq!(p.state, ProcessState::Ready);
        assert!(p.is_ready());
        assert!(!p.is_running());
    }

    #[test]
    fn test_pcb_fields_stored_correctly() {
        let p = Pcb::new(42, "shell", 2, 8);
        assert_eq!(p.pid,        42);
        assert_eq!(p.name,       "shell");
        assert_eq!(p.priority,   2);
        assert_eq!(p.time_slice, 8);
        assert_eq!(p.used_time,  0);
    }

    #[test]
    fn test_state_transitions_via_field() {
        let mut p = Pcb::new(1, "p", 0, 4);
        p.state = ProcessState::Running;
        assert!(p.is_running());
        p.state = ProcessState::Blocked;
        assert!(p.is_blocked());
        p.state = ProcessState::Terminated;
        assert!(p.is_terminated());
    }
}
