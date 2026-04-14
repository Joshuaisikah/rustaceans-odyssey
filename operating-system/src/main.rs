// ─── operating-system — integration demo ─────────────────────────────────────
//
// Run with:  cargo run -p operating-system

use operating_system::{PageTable, PhysAddr, Scheduler, VirtAddr, PAGE_SIZE};

fn main() {
    println!("=== operating-system integration demo ===\n");

    demo_paging();
    demo_scheduler();

    println!("\nAll demos completed.");
}

// ── Demo 1: Page table ────────────────────────────────────────────────────────
fn demo_paging() {
    println!("[ Demo 1 ] Page table");

    let mut pt = PageTable::new();

    // Map three virtual pages to distinct physical frames.
    pt.map(VirtAddr(0 * PAGE_SIZE as u64), PhysAddr(0xA000), true,  false);
    pt.map(VirtAddr(1 * PAGE_SIZE as u64), PhysAddr(0xB000), false, true);
    pt.map(VirtAddr(2 * PAGE_SIZE as u64), PhysAddr(0xC000), true,  true);

    // Translate with offset preservation.
    let virt   = VirtAddr(1 * PAGE_SIZE as u64 + 256);
    let phys   = pt.translate(virt).expect("page should be mapped");
    assert_eq!(phys, PhysAddr(0xB000 + 256));
    println!("  translate 0x{:X} → 0x{:X}  ✓", virt.0, phys.0);

    // Page fault for unmapped address.
    assert_eq!(pt.translate(VirtAddr(0xDEAD_0000)), None);
    println!("  unmapped address → None (page fault simulated)  ✓");

    // Unmap a page.
    assert!(pt.unmap(VirtAddr(0)));
    assert!(!pt.is_mapped(VirtAddr(0)));
    println!("  unmap verified  ✓");

    // Remap overwrites.
    pt.map(VirtAddr(0x1000), PhysAddr(0xF000), false, false);
    assert_eq!(pt.translate(VirtAddr(0x1100)), Some(PhysAddr(0xF100)));
    println!("  remap overwrites old mapping  ✓");
}

// ── Demo 2: Round-robin scheduler ────────────────────────────────────────────
fn demo_scheduler() {
    println!("[ Demo 2 ] Round-robin scheduler");

    let mut s = Scheduler::new();

    let pid_a = s.spawn("shell",   1, 2);
    let pid_b = s.spawn("editor",  1, 2);
    let pid_c = s.spawn("network", 1, 1);

    println!("  spawned {} processes", s.ready_count());

    // Run a few ticks to see round-robin in action.
    let mut history = Vec::new();
    for _ in 0..6 {
        if let Some(pid) = s.tick() {
            history.push(pid);
        }
    }
    println!("  tick history: {history:?}");
    assert!(!history.is_empty());

    // Block the currently running process (simulate I/O wait).
    s.block_current();
    let blocked_pid = history.last().copied().unwrap();
    println!("  blocked PID {blocked_pid}");
    assert_eq!(s.blocked_count(), 1);

    // Unblock it.
    assert!(s.unblock(blocked_pid));
    assert_eq!(s.blocked_count(), 0);
    println!("  unblocked PID {blocked_pid}  ✓");

    // Terminate the currently running process.
    s.tick();
    s.terminate_current();
    assert_eq!(s.terminated_count(), 1);
    println!("  terminated 1 process  ✓");

    // Suppress unused variable warnings.
    let _ = (pid_a, pid_b, pid_c);
}
