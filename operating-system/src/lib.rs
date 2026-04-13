// OS concepts simulated on the host (no #![no_std]).
// Build: memory management (paging) + process scheduling.

// ─── Paging ──────────────────────────────────────────────────────────────────

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtAddr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysAddr(pub u64);

impl VirtAddr {
    pub fn page_number(&self) -> u64 {
        self.0 / PAGE_SIZE as u64
    }
    pub fn page_offset(&self) -> u64 {
        self.0 % PAGE_SIZE as u64
    }
}

impl PhysAddr {
    pub fn frame_number(&self) -> u64 {
        self.0 / PAGE_SIZE as u64
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PageEntry {
    pub frame_number: u64,
    pub present: bool,
    pub writable: bool,
    pub executable: bool,
}

/// Single-level (flat) page table.
pub struct PageTable {
    // TODO: add your fields
}

impl PageTable {
    pub fn new() -> Self {
        todo!()
    }

    /// Map a virtual page to a physical frame.
    pub fn map(&mut self, virt: VirtAddr, phys: PhysAddr, writable: bool, executable: bool) {
        todo!()
    }

    /// Remove the mapping for the page containing `virt`.
    /// Returns `true` if a mapping existed.
    pub fn unmap(&mut self, virt: VirtAddr) -> bool {
        todo!()
    }

    /// Translate a virtual address to a physical address.
    /// Returns `None` for unmapped or non-present pages (page fault).
    pub fn translate(&self, virt: VirtAddr) -> Option<PhysAddr> {
        todo!()
    }

    pub fn is_mapped(&self, virt: VirtAddr) -> bool {
        todo!()
    }
}

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
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub priority: u8,
    pub time_slice: u32,   // ticks allotted per scheduling round
    pub used_time: u32,    // ticks consumed in the current round
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

// ─── Round-robin scheduler ───────────────────────────────────────────────────

pub struct Scheduler {
    // TODO: add your fields (ready queue, running slot, terminated list, …)
}

impl Scheduler {
    pub fn new() -> Self {
        todo!()
    }

    /// Create a new process and put it in the ready queue.
    /// Returns the assigned PID (auto-incremented from 1).
    pub fn spawn(&mut self, name: &str, priority: u8, time_slice: u32) -> u32 {
        todo!()
    }

    /// Advance the scheduler by one tick.
    ///
    /// Rules:
    /// 1. Increment `used_time` of the running process.
    /// 2. If `used_time >= time_slice`, preempt: reset `used_time`, move the
    ///    process to the back of the ready queue, clear the running slot.
    /// 3. If no process is running, dequeue the front of the ready queue.
    ///
    /// Returns the PID of the currently-running process, or `None` if the
    /// ready queue is empty.
    pub fn tick(&mut self) -> Option<u32> {
        todo!()
    }

    /// Terminate the currently running process.
    pub fn terminate_current(&mut self) {
        todo!()
    }

    /// Block the currently running process (moves it out of the scheduler).
    pub fn block_current(&mut self) {
        todo!()
    }

    /// Unblock a process by PID and move it back to the ready queue.
    /// Returns `true` if the process was found in the blocked set.
    pub fn unblock(&mut self, pid: u32) -> bool {
        todo!()
    }

    pub fn running_pid(&self) -> Option<u32> {
        todo!()
    }

    pub fn ready_count(&self) -> usize {
        todo!()
    }

    pub fn terminated_count(&self) -> usize {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── VirtAddr / PhysAddr ──────────────────────────────────────────────────

    // page_number is the address divided by PAGE_SIZE.
    #[test]
    fn test_virt_addr_page_number() {
        let addr = VirtAddr(8192); // 2 × PAGE_SIZE
        assert_eq!(addr.page_number(), 2);
    }

    // page_offset is the remainder within the page.
    #[test]
    fn test_virt_addr_page_offset() {
        let addr = VirtAddr(4096 + 256);
        assert_eq!(addr.page_offset(), 256);
    }

    // frame_number for a PhysAddr.
    #[test]
    fn test_phys_addr_frame_number() {
        let addr = PhysAddr(PAGE_SIZE as u64 * 5);
        assert_eq!(addr.frame_number(), 5);
    }

    // ── PageTable ────────────────────────────────────────────────────────────

    // translate returns the correct physical address after mapping.
    #[test]
    fn test_page_table_translate_mapped_address() {
        let mut pt = PageTable::new();
        let virt = VirtAddr(0x1000);         // page 1
        let phys = VirtAddr(0x5000);         // frame 5
        pt.map(virt, PhysAddr(phys.0), true, false);

        let result = pt.translate(VirtAddr(0x1ABC));
        assert_eq!(result, Some(PhysAddr(0x5ABC)),
            "offset 0xABC should be preserved");
    }

    // translate returns None for an unmapped address (page fault).
    #[test]
    fn test_page_table_translate_unmapped_is_none() {
        let pt = PageTable::new();
        assert_eq!(pt.translate(VirtAddr(0xDEAD_0000)), None);
    }

    // is_mapped is true after map and false after unmap.
    #[test]
    fn test_page_table_map_and_unmap() {
        let mut pt = PageTable::new();
        let virt = VirtAddr(0x2000);
        pt.map(virt, PhysAddr(0x9000), true, true);
        assert!(pt.is_mapped(virt));
        assert!(pt.unmap(virt));
        assert!(!pt.is_mapped(virt));
    }

    // unmap returns false for an address that was never mapped.
    #[test]
    fn test_page_table_unmap_not_mapped_returns_false() {
        let mut pt = PageTable::new();
        assert!(!pt.unmap(VirtAddr(0xF000_0000)));
    }

    // Mapping the same virtual address twice overwrites the old mapping.
    #[test]
    fn test_page_table_remap_overwrites() {
        let mut pt = PageTable::new();
        let virt = VirtAddr(0x3000);
        pt.map(virt, PhysAddr(0x1000), true, false);
        pt.map(virt, PhysAddr(0x2000), false, false); // remap
        let result = pt.translate(VirtAddr(0x3100)).unwrap();
        assert_eq!(result, PhysAddr(0x2100));
    }

    // ── Scheduler ────────────────────────────────────────────────────────────

    // A spawned process is placed in the ready queue.
    #[test]
    fn test_scheduler_spawn_adds_to_ready_queue() {
        let mut s = Scheduler::new();
        s.spawn("init", 1, 4);
        assert_eq!(s.ready_count(), 1);
        assert_eq!(s.running_pid(), None);
    }

    // PIDs are auto-incremented starting from 1.
    #[test]
    fn test_scheduler_pids_increment() {
        let mut s = Scheduler::new();
        let pid1 = s.spawn("a", 1, 4);
        let pid2 = s.spawn("b", 1, 4);
        assert_eq!(pid1, 1);
        assert_eq!(pid2, 2);
    }

    // First tick dequeues the process and marks it Running.
    #[test]
    fn test_scheduler_first_tick_starts_process() {
        let mut s = Scheduler::new();
        let pid = s.spawn("proc", 1, 4);
        let running = s.tick();
        assert_eq!(running, Some(pid));
        assert_eq!(s.running_pid(), Some(pid));
    }

    // After `time_slice` ticks the process is preempted.
    #[test]
    fn test_scheduler_preemption_after_time_slice() {
        let mut s = Scheduler::new();
        let pid_a = s.spawn("A", 1, 2); // slice = 2
        let pid_b = s.spawn("B", 1, 2);

        // Tick 1 & 2: A runs (uses its full slice)
        assert_eq!(s.tick(), Some(pid_a));
        assert_eq!(s.tick(), Some(pid_a));
        // Tick 3: A is preempted; B starts
        assert_eq!(s.tick(), Some(pid_b));
    }

    // Round-robin: processes alternate fairly.
    #[test]
    fn test_scheduler_round_robin_two_processes() {
        let mut s = Scheduler::new();
        let a = s.spawn("A", 1, 1);
        let b = s.spawn("B", 1, 1);

        // With slice=1, each tick a different process should run.
        let r1 = s.tick();
        let r2 = s.tick();
        let r3 = s.tick();

        assert_eq!(r1, Some(a));
        assert_eq!(r2, Some(b));
        assert_eq!(r3, Some(a));
    }

    // terminate_current removes the process from the scheduler.
    #[test]
    fn test_scheduler_terminate_current() {
        let mut s = Scheduler::new();
        s.spawn("proc", 1, 4);
        s.tick(); // process starts running
        s.terminate_current();
        assert_eq!(s.running_pid(), None);
        assert_eq!(s.terminated_count(), 1);
    }

    // tick() returns None when there are no processes.
    #[test]
    fn test_scheduler_tick_with_no_processes_returns_none() {
        let mut s = Scheduler::new();
        assert_eq!(s.tick(), None);
    }

    // block_current removes the process; unblock re-queues it.
    #[test]
    fn test_scheduler_block_and_unblock() {
        let mut s = Scheduler::new();
        let pid = s.spawn("io_proc", 1, 4);
        s.tick(); // starts running
        s.block_current();

        // Process is no longer running or ready
        assert_eq!(s.running_pid(), None);
        assert_eq!(s.ready_count(), 0);

        // Unblock re-queues it
        assert!(s.unblock(pid));
        assert_eq!(s.ready_count(), 1);

        // Next tick resumes it
        assert_eq!(s.tick(), Some(pid));
    }

    // unblock on an unknown PID returns false.
    #[test]
    fn test_scheduler_unblock_unknown_pid_returns_false() {
        let mut s = Scheduler::new();
        assert!(!s.unblock(999));
    }
}
