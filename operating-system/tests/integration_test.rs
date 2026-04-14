// ─── operating-system: integration tests ─────────────────────────────────────
//
// Black-box tests through the public API.
// Exercises virtual memory (VirtAddr, PhysAddr, PageTable) and process
// management (ProcessState, Pcb, Scheduler) as an integrated system.

use operating_system::{
    PageTable, PhysAddr, ProcessState, Scheduler, VirtAddr, PAGE_SIZE,
};

// ── Address types ─────────────────────────────────────────────────────────────

#[test]
fn test_page_size_is_4096() {
    assert_eq!(PAGE_SIZE, 4096);
}

#[test]
fn test_virt_addr_page_arithmetic_is_consistent() {
    let v  = VirtAddr(12345);
    let pn = v.page_number();
    let of = v.page_offset();
    assert_eq!(pn * PAGE_SIZE as u64 + of, v.0);
}

#[test]
fn test_page_aligned_address_has_zero_offset() {
    let v = VirtAddr(4096 * 7);
    assert_eq!(v.page_offset(), 0);
}

#[test]
fn test_phys_addr_frame_number() {
    let p = PhysAddr(PAGE_SIZE as u64 * 5);
    assert_eq!(p.frame_number(), 5);
}

// ── PageTable ─────────────────────────────────────────────────────────────────

#[test]
fn test_page_table_translate_preserves_offset() {
    let mut pt = PageTable::new();
    // Map virtual page 0x1000 → physical frame 0x5000.
    pt.map(VirtAddr(0x1000), PhysAddr(0x5000), true, false);
    // Virtual 0x1ABC → physical 0x5ABC (offset 0xABC preserved).
    assert_eq!(pt.translate(VirtAddr(0x1ABC)), Some(PhysAddr(0x5ABC)));
}

#[test]
fn test_page_table_unmapped_translates_to_none() {
    let pt = PageTable::new();
    assert_eq!(pt.translate(VirtAddr(0xDEAD_0000)), None);
}

#[test]
fn test_page_table_map_and_unmap() {
    let mut pt = PageTable::new();
    let virt   = VirtAddr(0x2000);
    pt.map(virt, PhysAddr(0x9000), true, true);
    assert!(pt.is_mapped(virt));
    assert!(pt.unmap(virt));
    assert!(!pt.is_mapped(virt));
    assert_eq!(pt.translate(virt), None);
}

#[test]
fn test_page_table_unmap_not_mapped_returns_false() {
    let mut pt = PageTable::new();
    assert!(!pt.unmap(VirtAddr(0xFFFF_0000)));
}

#[test]
fn test_page_table_remap_overwrites_previous_mapping() {
    let mut pt = PageTable::new();
    let virt   = VirtAddr(0x3000);
    pt.map(virt, PhysAddr(0x1000), true, false);
    pt.map(virt, PhysAddr(0x2000), false, false); // remap
    assert_eq!(pt.translate(VirtAddr(0x3100)), Some(PhysAddr(0x2100)));
}

#[test]
fn test_page_table_entry_flags() {
    let mut pt = PageTable::new();
    pt.map(VirtAddr(0x4000), PhysAddr(0x8000), true, false);
    let entry = pt.get_entry(VirtAddr(0x4000)).unwrap();
    assert!(entry.present   && entry.writable && !entry.executable);
}

// Multiple distinct virtual pages map independently.
#[test]
fn test_page_table_multiple_independent_mappings() {
    let mut pt = PageTable::new();
    pt.map(VirtAddr(0x0000), PhysAddr(0xA000), true, false);
    pt.map(VirtAddr(0x1000), PhysAddr(0xB000), true, false);
    pt.map(VirtAddr(0x2000), PhysAddr(0xC000), true, false);

    assert_eq!(pt.translate(VirtAddr(0x0100)), Some(PhysAddr(0xA100)));
    assert_eq!(pt.translate(VirtAddr(0x1200)), Some(PhysAddr(0xB200)));
    assert_eq!(pt.translate(VirtAddr(0x2300)), Some(PhysAddr(0xC300)));
}

// ── Scheduler ─────────────────────────────────────────────────────────────────

#[test]
fn test_scheduler_starts_empty() {
    let s = Scheduler::new();
    assert_eq!(s.ready_count(), 0);
    assert_eq!(s.running_pid(), None);
}

#[test]
fn test_scheduler_spawn_assigns_sequential_pids() {
    let mut s = Scheduler::new();
    assert_eq!(s.spawn("a", 1, 4), 1);
    assert_eq!(s.spawn("b", 1, 4), 2);
    assert_eq!(s.spawn("c", 1, 4), 3);
}

#[test]
fn test_scheduler_first_tick_starts_first_process() {
    let mut s   = Scheduler::new();
    let pid = s.spawn("init", 1, 4);
    assert_eq!(s.tick(), Some(pid));
    assert_eq!(s.running_pid(), Some(pid));
}

#[test]
fn test_scheduler_tick_with_no_processes_returns_none() {
    let mut s = Scheduler::new();
    assert_eq!(s.tick(), None);
}

// After the time slice is exhausted the next process gets the CPU.
#[test]
fn test_scheduler_preemption_after_time_slice() {
    let mut s = Scheduler::new();
    let a = s.spawn("A", 1, 2);
    let b = s.spawn("B", 1, 2);
    assert_eq!(s.tick(), Some(a)); // tick 1 — A runs
    assert_eq!(s.tick(), Some(a)); // tick 2 — A still (time slice = 2)
    assert_eq!(s.tick(), Some(b)); // tick 3 — A preempted, B starts
}

// Round-robin: A(ts=1), B(ts=1) — they alternate every tick.
#[test]
fn test_scheduler_round_robin_alternation() {
    let mut s = Scheduler::new();
    let a = s.spawn("A", 1, 1);
    let b = s.spawn("B", 1, 1);
    assert_eq!(s.tick(), Some(a));
    assert_eq!(s.tick(), Some(b));
    assert_eq!(s.tick(), Some(a));
    assert_eq!(s.tick(), Some(b));
}

#[test]
fn test_scheduler_terminate_current_process() {
    let mut s = Scheduler::new();
    s.spawn("p", 1, 4);
    s.tick();
    s.terminate_current();
    assert_eq!(s.running_pid(), None);
    assert_eq!(s.terminated_count(), 1);
}

#[test]
fn test_scheduler_block_and_unblock() {
    let mut s   = Scheduler::new();
    let pid = s.spawn("io", 1, 4);
    s.tick();
    s.block_current();
    assert_eq!(s.blocked_count(), 1);
    assert_eq!(s.running_pid(), None);

    assert!(s.unblock(pid));
    assert_eq!(s.blocked_count(), 0);
    assert_eq!(s.tick(), Some(pid));
}

#[test]
fn test_scheduler_unblock_unknown_pid_returns_false() {
    let mut s = Scheduler::new();
    assert!(!s.unblock(999));
}

// Full lifecycle: spawn 3 processes, run each to termination one at a time.
#[test]
fn test_scheduler_full_lifecycle_three_processes() {
    let mut s = Scheduler::new();
    let _a = s.spawn("A", 1, 2);
    let _b = s.spawn("B", 1, 2);
    let _c = s.spawn("C", 1, 2);

    assert_eq!(s.ready_count(), 3);

    // Tick until running, then terminate.
    for _ in 0..3 {
        while s.running_pid().is_none() { s.tick(); }
        s.terminate_current();
    }

    assert_eq!(s.terminated_count(), 3);
    assert_eq!(s.tick(), None); // no processes left
}
