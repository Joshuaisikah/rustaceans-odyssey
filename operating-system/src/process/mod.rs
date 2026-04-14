// ─── process — process management subsystem ──────────────────────────────────

pub mod pcb;
pub mod scheduler;

pub use pcb::{Pcb, ProcessState};
pub use scheduler::Scheduler;
