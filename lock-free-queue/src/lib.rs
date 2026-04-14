// ─── lock-free-queue ─────────────────────────────────────────────────────────
//
// Module layout:
//   spsc — Single-Producer Single-Consumer bounded queue (lock-free)
//   mpmc — Multi-Producer Multi-Consumer bounded queue

pub mod mpmc;
pub mod spsc;

pub use mpmc::MpmcQueue;
pub use spsc::SpscQueue;
