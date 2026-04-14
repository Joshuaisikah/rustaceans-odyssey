// ─── operating-system ────────────────────────────────────────────────────────
//
// OS concepts simulated in safe Rust (hosted environment).
//
// Crate layout
// ┌────────────────────────────────────────────────────────┐
// │ memory/               Virtual memory subsystem         │
// │   address.rs       →  VirtAddr, PhysAddr, PAGE_SIZE   │
// │   page_table.rs    →  PageEntry, PageTable             │
// ├────────────────────────────────────────────────────────┤
// │ process/              Process management subsystem     │
// │   pcb.rs           →  ProcessState, Pcb               │
// │   scheduler.rs     →  Scheduler (round-robin)         │
// └────────────────────────────────────────────────────────┘

pub mod memory;
pub mod process;

pub use memory::{PageEntry, PageTable, PhysAddr, VirtAddr, PAGE_SIZE};
pub use process::{Pcb, ProcessState, Scheduler};
